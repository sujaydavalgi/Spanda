//! Hardware and capability traceability matrix generation.

use crate::registry::{actuator_capabilities, lookup_capability, sensor_capabilities};
use serde::{Deserialize, Serialize};
use spanda_ast::foundations::{HardwareDecl, KillSwitchDecl};
use spanda_ast::nodes::{ActuatorDecl, Program, RobotDecl, SensorDecl};
use std::collections::{HashMap, HashSet};

/// Single row in the hardware traceability matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardwareTraceRow {
    pub hardware_component: String,
    pub used_by: String,
    pub source_location: String,
    pub capability: String,
    pub provider: String,
    pub verified: bool,
    pub safety_rule: Option<String>,
    pub notes: Option<String>,
}

/// Single row in the capability traceability matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityTraceRow {
    pub capability: String,
    pub required_by: String,
    pub provided_by: String,
    pub hardware: String,
    pub package: String,
    pub provider: String,
    pub safety_rule: Option<String>,
    pub status: String,
    pub notes: Option<String>,
}

/// Combined traceability report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceabilityReport {
    pub hardware_rows: Vec<HardwareTraceRow>,
    pub capability_rows: Vec<CapabilityTraceRow>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Generate hardware-to-code traceability matrix from a parsed program.
pub fn hardware_traceability(program: &Program) -> TraceabilityReport {
    let Program::Program {
        hardware_profiles,
        robots,
        kill_switches,
        ..
    } = program;

    let mut rows = Vec::new();
    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    let mut declared_sensors: HashSet<String> = HashSet::new();
    let mut declared_actuators: HashSet<String> = HashSet::new();
    let mut used_sensors: HashSet<String> = HashSet::new();
    let mut used_actuators: HashSet<String> = HashSet::new();

    // Index hardware profiles by name.
    let profiles: HashMap<String, &HardwareDecl> = hardware_profiles
        .iter()
        .map(|h| {
            let HardwareDecl::HardwareDecl { name, .. } = h;
            (name.clone(), h)
        })
        .collect();

    // Emit a row for each hardware component capability.
    for hw in hardware_profiles {
        let HardwareDecl::HardwareDecl {
            name,
            sensors,
            actuators,
            connectivity,
            components,
            ..
        } = hw;

        for sensor in sensors {
            declared_sensors.insert(format!("{name}.{sensor}"));
            for cap in sensor_capabilities(sensor) {
                rows.push(HardwareTraceRow {
                    hardware_component: format!("{name}.{sensor}"),
                    used_by: String::new(),
                    source_location: format!("hardware {name}"),
                    capability: cap.into(),
                    provider: infer_sensor_provider(sensor),
                    verified: true,
                    safety_rule: None,
                    notes: None,
                });
            }
        }

        for actuator in actuators {
            declared_actuators.insert(format!("{name}.{actuator}"));
            for cap in actuator_capabilities(actuator) {
                rows.push(HardwareTraceRow {
                    hardware_component: format!("{name}.{actuator}"),
                    used_by: String::new(),
                    source_location: format!("hardware {name}"),
                    capability: cap.into(),
                    provider: infer_actuator_provider(actuator),
                    verified: true,
                    safety_rule: if cap == "emergency_stop" {
                        Some("stop_all_actuators".into())
                    } else {
                        None
                    },
                    notes: None,
                });
            }
        }

        for conn in connectivity {
            rows.push(HardwareTraceRow {
                hardware_component: format!("{name}.{conn}"),
                used_by: String::new(),
                source_location: format!("hardware {name}"),
                capability: "network_connect".into(),
                provider: infer_connectivity_provider(conn),
                verified: true,
                safety_rule: None,
                notes: None,
            });
        }

        // Detailed component declarations with explicit capabilities.
        for comp in components {
            for cap in &comp.capabilities {
                rows.push(HardwareTraceRow {
                    hardware_component: format!("{name}.{}", comp.name),
                    used_by: String::new(),
                    source_location: format!(
                        "hardware {name} {}:{}",
                        comp.component_kind, comp.name
                    ),
                    capability: cap.clone(),
                    provider: infer_component_provider(&comp.component_kind, &comp.component_type),
                    verified: true,
                    safety_rule: None,
                    notes: Some(comp.component_type.clone()),
                });
            }
        }
    }

    // Cross-reference robot sensor/actuator usage.
    for robot in robots {
        let RobotDecl::RobotDecl {
            name,
            sensors,
            actuators,
            uses_hardware,
            ..
        } = robot;

        for sensor in sensors {
            let SensorDecl::SensorDecl {
                name: sensor_name,
                sensor_type,
                ..
            } = sensor;
            used_sensors.insert(sensor_name.clone());
            rows.push(HardwareTraceRow {
                hardware_component: sensor_name.clone(),
                used_by: name.clone(),
                source_location: format!("robot {name} sensor {sensor_name}"),
                capability: sensor_type.clone(),
                provider: infer_sensor_provider(sensor_type),
                verified: uses_hardware
                    .as_ref()
                    .map(|hw| profiles.contains_key(hw))
                    .unwrap_or(true),
                safety_rule: None,
                notes: uses_hardware.clone(),
            });
        }

        for actuator in actuators {
            let ActuatorDecl::ActuatorDecl {
                name: actuator_name,
                actuator_type,
                ..
            } = actuator;
            used_actuators.insert(actuator_name.clone());
            let has_safety = rows
                .iter()
                .any(|r| r.hardware_component.contains(actuator_name) && r.safety_rule.is_some());
            if !has_safety {
                warnings.push(format!(
                    "Actuator '{actuator_name}' on robot '{name}' has no safety gate"
                ));
            }
            rows.push(HardwareTraceRow {
                hardware_component: actuator_name.clone(),
                used_by: name.clone(),
                source_location: format!("robot {name} actuator {actuator_name}"),
                capability: actuator_type.clone(),
                provider: infer_actuator_provider(actuator_type),
                verified: true,
                safety_rule: if actuator_type.contains("Drive") {
                    Some("max_speed".into())
                } else {
                    None
                },
                notes: None,
            });
        }

        // Flag robots using hardware without declaration.
        if let Some(hw) = uses_hardware {
            if !profiles.contains_key(hw) {
                errors.push(format!("Robot '{name}' uses undeclared hardware '{hw}'"));
            }
        }
    }

    // Kill switch traceability.
    for ks in kill_switches {
        let KillSwitchDecl::KillSwitchDecl { name, priority, .. } = ks;
        rows.push(HardwareTraceRow {
            hardware_component: name.clone(),
            used_by: "system".into(),
            source_location: format!("kill_switch {name}"),
            capability: "emergency_stop".into(),
            provider: "KillSwitchProvider".into(),
            verified: true,
            safety_rule: Some(format!("priority={priority}")),
            notes: Some("stop_all_actuators".into()),
        });
    }

    // Warn about declared-but-unused hardware.
    for key in declared_sensors.difference(&used_sensors) {
        warnings.push(format!("Declared sensor '{key}' is unused"));
    }
    for key in declared_actuators.difference(&used_actuators) {
        warnings.push(format!("Declared actuator '{key}' is unused"));
    }

    TraceabilityReport {
        hardware_rows: rows,
        capability_rows: Vec::new(),
        warnings,
        errors,
    }
}

/// Generate capability traceability matrix.
pub fn capability_traceability(program: &Program) -> TraceabilityReport {
    let mut report = hardware_traceability(program);
    let Program::Program {
        robots,
        requires_capabilities,
        ..
    } = program;

    for req in requires_capabilities {
        let cap_name = &req.capability;
        let required_by = req.required_by.clone().unwrap_or_else(|| "program".into());
        if let Some(def) = lookup_capability(cap_name) {
            for robot in robots {
                let RobotDecl::RobotDecl {
                    name,
                    uses_hardware,
                    exposes_capabilities,
                    ..
                } = robot;
                let provided =
                    exposes_capabilities.contains(cap_name) || infer_provides(robot, cap_name);
                report.capability_rows.push(CapabilityTraceRow {
                    capability: cap_name.clone(),
                    required_by: required_by.clone(),
                    provided_by: name.clone(),
                    hardware: uses_hardware.clone().unwrap_or_default(),
                    package: def.minimum.required_packages.join("+"),
                    provider: def.minimum.required_providers.join("+"),
                    safety_rule: def.minimum.required_safety_rules.first().cloned(),
                    status: if provided { "PASS" } else { "FAIL" }.into(),
                    notes: Some(def.description.clone()),
                });
            }
        } else {
            report
                .errors
                .push(format!("Unknown capability '{cap_name}'"));
        }
    }

    report
}

fn infer_provides(robot: &RobotDecl, capability: &str) -> bool {
    let RobotDecl::RobotDecl {
        sensors,
        actuators,
        exposes_capabilities,
        kill_switches,
        ..
    } = robot;
    if exposes_capabilities.iter().any(|c| c == capability) {
        return true;
    }
    match capability {
        "gps_navigation" => sensors.iter().any(|s| {
            let SensorDecl::SensorDecl { sensor_type, .. } = s;
            sensor_type == "GPS" || sensor_type == "GNSS"
        }),
        "emergency_stop" => {
            !kill_switches.is_empty()
                || actuators.iter().any(|a| {
                    let ActuatorDecl::ActuatorDecl { actuator_type, .. } = a;
                    actuator_type.contains("Drive")
                })
        }
        "obstacle_avoidance" => sensors.iter().any(|s| {
            let SensorDecl::SensorDecl { sensor_type, .. } = s;
            matches!(sensor_type.as_str(), "Lidar" | "DepthCamera" | "Radar")
        }),
        _ => false,
    }
}

fn infer_sensor_provider(sensor_type: &str) -> String {
    match sensor_type {
        "GPS" | "GNSS" => "PositioningProvider".into(),
        "Camera" => "VisionProvider".into(),
        "Lidar" => "SensorProvider".into(),
        _ => "SensorProvider".into(),
    }
}

fn infer_actuator_provider(actuator_type: &str) -> String {
    match actuator_type {
        t if t.contains("Drive") => "ActuatorProvider".into(),
        "Arm" | "Gripper" => "ManipulationProvider".into(),
        _ => "ActuatorProvider".into(),
    }
}

fn infer_connectivity_provider(conn: &str) -> String {
    match conn {
        "WiFi" => "spanda-wifi".into(),
        "LTE" | "FiveG" | "Cellular" => "spanda-cellular".into(),
        "Bluetooth" | "BLE" => "spanda-ble".into(),
        "MQTT" => "spanda-mqtt".into(),
        _ => "ConnectivityProvider".into(),
    }
}

fn infer_component_provider(kind: &str, component_type: &str) -> String {
    match kind {
        "sensor" => infer_sensor_provider(component_type),
        "actuator" => infer_actuator_provider(component_type),
        "compute" => "ComputeProvider".into(),
        "connectivity" => infer_connectivity_provider(component_type),
        _ => "HardwareProvider".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;

    fn parse_source(source: &str) -> spanda_ast::nodes::Program {
        parse(tokenize(source).expect("tokenize")).expect("parse")
    }

    #[test]
    fn hardware_traceability_from_program() {
        let source = r#"
hardware RoverV1 {
    sensors [GPS, Camera, Lidar];
    actuators [DifferentialDrive];
    connectivity [WiFi, LTE];
}

robot Rover {
    sensor gps: GPS;
    actuator wheels: DifferentialDrive;
    uses hardware RoverV1;
}
"#;
        let program = parse_source(source);
        let report = hardware_traceability(&program);
        assert!(!report.hardware_rows.is_empty());
        assert!(report
            .hardware_rows
            .iter()
            .any(|r| r.capability == "read_location"));
    }
}
