//! Robot capability inference from hardware, packages, and safety rules.

use crate::registry::{lookup_capability, package_contributions, CapabilityDefinition};
use serde::{Deserialize, Serialize};
use spanda_ast::foundations::HardwareDecl;
use spanda_ast::nodes::{ActuatorDecl, Program, RobotDecl, SensorDecl};
use std::collections::HashSet;

/// Single inferred or declared robot capability row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotCapabilityRow {
    pub capability: String,
    pub source: String,
    pub required_components: Vec<String>,
    pub status: String,
    pub notes: Option<String>,
}

/// Robot capability analysis report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotCapabilityReport {
    pub robot: String,
    pub rows: Vec<RobotCapabilityRow>,
    pub declared: Vec<String>,
    pub inferred: Vec<String>,
}

/// Infer robot capabilities from program structure.
pub fn infer_robot_capabilities(program: &Program) -> Vec<RobotCapabilityReport> {
    let Program::Program {
        hardware_profiles,
        robots,
        ..
    } = program;

    let hw_index: std::collections::HashMap<String, &HardwareDecl> = hardware_profiles
        .iter()
        .map(|h| {
            let HardwareDecl::HardwareDecl { name, .. } = h;
            (name.clone(), h)
        })
        .collect();

    robots
        .iter()
        .map(|robot| analyze_robot(robot, &hw_index))
        .collect()
}

fn analyze_robot(
    robot: &RobotDecl,
    hw_index: &std::collections::HashMap<String, &HardwareDecl>,
) -> RobotCapabilityReport {
    let RobotDecl::RobotDecl {
        name,
        sensors,
        actuators,
        uses_hardware,
        exposes_capabilities,
        mission,
        permissions,
        kill_switches,
        ..
    } = robot;

    let mut rows = Vec::new();
    let mut inferred = HashSet::new();

    // Collect hardware component types from robot and linked profile.
    let mut sensor_types: HashSet<String> = sensors
        .iter()
        .map(|s| {
            let SensorDecl::SensorDecl { sensor_type, .. } = s;
            sensor_type.clone()
        })
        .collect();
    let mut actuator_types: HashSet<String> = actuators
        .iter()
        .map(|a| {
            let ActuatorDecl::ActuatorDecl { actuator_type, .. } = a;
            actuator_type.clone()
        })
        .collect();
    let mut connectivity: HashSet<String> = HashSet::new();

    if let Some(hw_name) = uses_hardware {
        if let Some(HardwareDecl::HardwareDecl {
            sensors: hw_sensors,
            actuators: hw_actuators,
            connectivity: hw_conn,
            ..
        }) = hw_index.get(hw_name)
        {
            sensor_types.extend(hw_sensors.iter().cloned());
            actuator_types.extend(hw_actuators.iter().cloned());
            connectivity.extend(hw_conn.iter().cloned());
        }
    }

    // Infer capabilities from component combinations.
    if has_any(&sensor_types, &["GPS", "GNSS"]) && has_any(&actuator_types, &["DifferentialDrive"])
    {
        inferred.insert("gps_navigation".into());
    }
    if has_any(&sensor_types, &["Lidar", "DepthCamera", "Radar"]) {
        inferred.insert("obstacle_avoidance".into());
    }
    if has_any(&sensor_types, &["Lidar", "Camera"])
        && has_any(&actuator_types, &["DifferentialDrive"])
    {
        inferred.insert("autonomous_navigation".into());
    }
    if has_any(&connectivity, &["WiFi", "LTE", "FiveG", "MQTT"]) {
        inferred.insert("telemetry_streaming".into());
    }
    if has_any(&connectivity, &["WiFi", "LTE", "FiveG", "Bluetooth"]) {
        inferred.insert("remote_control".into());
    }
    if !kill_switches.is_empty() || actuator_types.iter().any(|a| a.contains("Drive")) {
        inferred.insert("emergency_stop".into());
    }
    if sensor_types.contains("Camera") {
        inferred.insert("vision_processing".into());
    }
    if permissions.is_some() {
        inferred.insert("secure_communication".into());
    }

    // Mission-required capabilities from mission block name.
    if let Some(spanda_ast::foundations::MissionDecl::MissionDecl {
        name: Some(mname), ..
    }) = mission
    {
        if mname == "Patrol" {
            inferred.insert("gps_navigation".into());
            inferred.insert("obstacle_avoidance".into());
            inferred.insert("emergency_stop".into());
            inferred.insert("telemetry_streaming".into());
        }
    }

    // Build report rows for declared + inferred capabilities.
    let all_caps: HashSet<String> = exposes_capabilities
        .iter()
        .cloned()
        .chain(inferred.iter().cloned())
        .collect();

    for cap in &all_caps {
        let is_declared = exposes_capabilities.contains(cap);
        let is_inferred = inferred.contains(cap);
        let def = lookup_capability(cap);
        rows.push(RobotCapabilityRow {
            capability: cap.clone(),
            source: if is_declared && is_inferred {
                "declared+inferred".into()
            } else if is_declared {
                "declared".into()
            } else {
                "inferred".into()
            },
            required_components: def
                .as_ref()
                .map(|d| components_for(&d.minimum))
                .unwrap_or_default(),
            status: if satisfies(cap, &sensor_types, &actuator_types, &connectivity) {
                "PASS".into()
            } else {
                "PARTIAL".into()
            },
            notes: def.map(|d| d.description),
        });
    }

    RobotCapabilityReport {
        robot: name.clone(),
        declared: exposes_capabilities.clone(),
        inferred: inferred.into_iter().collect(),
        rows,
    }
}

fn has_any(set: &HashSet<String>, candidates: &[&str]) -> bool {
    candidates.iter().any(|c| set.contains(*c))
}

fn components_for(req: &crate::registry::CapabilityRequirement) -> Vec<String> {
    let mut parts = Vec::new();
    parts.extend(req.any_of_sensors.iter().cloned());
    parts.extend(req.any_of_actuators.iter().cloned());
    parts.extend(req.any_of_connectivity.iter().cloned());
    parts.extend(req.required_packages.iter().cloned());
    parts
}

fn satisfies(
    cap: &str,
    sensors: &HashSet<String>,
    actuators: &HashSet<String>,
    connectivity: &HashSet<String>,
) -> bool {
    let Some(def) = lookup_capability(cap) else {
        return false;
    };
    let req = &def.minimum;
    let sensors_ok =
        req.any_of_sensors.is_empty() || req.any_of_sensors.iter().any(|s| sensors.contains(s));
    let actuators_ok = req.any_of_actuators.is_empty()
        || req.any_of_actuators.iter().any(|a| actuators.contains(a));
    let conn_ok = req.any_of_connectivity.is_empty()
        || req
            .any_of_connectivity
            .iter()
            .any(|c| connectivity.contains(c));
    sensors_ok && actuators_ok && conn_ok
}

/// List package contributions relevant to a robot's inferred capabilities.
pub fn package_capabilities_for_robot(report: &RobotCapabilityReport) -> Vec<CapabilityDefinition> {
    let contribs = package_contributions();
    let caps: HashSet<String> = report.rows.iter().map(|r| r.capability.clone()).collect();
    contribs
        .into_iter()
        .filter(|c| c.capabilities.iter().any(|cap| caps.contains(cap)))
        .flat_map(|c| {
            c.capabilities
                .into_iter()
                .filter_map(|cap| lookup_capability(&cap))
        })
        .collect()
}
