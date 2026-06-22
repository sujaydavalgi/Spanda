//! Minimum capable hardware safety verification.

use crate::registry::{lookup_capability, VerificationSeverity};
use crate::robot::infer_robot_capabilities;
use serde::{Deserialize, Serialize};
use spanda_ast::foundations::RequiresCapabilityDecl;
use spanda_ast::nodes::Program;

/// Row describing a missing minimum capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimumCapabilityRow {
    pub capability: String,
    pub required_by: String,
    pub status: String,
    pub missing: Vec<String>,
    pub suggested_fixes: Vec<String>,
}

/// Minimum capability safety check report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimumCapabilityReport {
    pub compatible: bool,
    pub rows: Vec<MinimumCapabilityRow>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Verify that robots satisfy minimum hardware for declared capabilities.
pub fn check_minimum_capabilities(program: &Program) -> MinimumCapabilityReport {
    let Program::Program {
        requires_capabilities,
        robots,
        ..
    } = program;

    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let robot_reports = infer_robot_capabilities(program);

    // Check explicit requires_capability declarations.
    for req in requires_capabilities {
        let row = check_requirement(req, &robot_reports);
        if row.status == "FAIL" {
            match req.severity {
                spanda_ast::foundations::RequiresCapabilitySeverity::Error => {
                    errors.push(format!(
                        "Mission requires {}. Missing: {}",
                        req.capability,
                        row.missing.join(", ")
                    ));
                }
                spanda_ast::foundations::RequiresCapabilitySeverity::Warning => {
                    warnings.push(format!(
                        "Capability {} partially satisfied: missing {}",
                        req.capability,
                        row.missing.join(", ")
                    ));
                }
                spanda_ast::foundations::RequiresCapabilitySeverity::Info => {}
            }
        }
        rows.push(row);
    }

    // Check mission-exposed capabilities on each robot.
    for report in &robot_reports {
        for cap_row in &report.rows {
            if cap_row.status == "PARTIAL" {
                if let Some(def) = lookup_capability(&cap_row.capability) {
                    let missing = compute_missing(&def.minimum, &cap_row.required_components);
                    let fixes = suggest_fixes(&cap_row.capability, &missing);
                    rows.push(MinimumCapabilityRow {
                        capability: cap_row.capability.clone(),
                        required_by: report.robot.clone(),
                        status: "FAIL".into(),
                        missing,
                        suggested_fixes: fixes,
                    });
                    if def.minimum.severity == VerificationSeverity::Error {
                        errors.push(format!(
                            "Robot '{}' requires capability '{}'",
                            report.robot, cap_row.capability
                        ));
                    }
                }
            }
        }
    }

    // Robots with mission blocks get default patrol checks.
    for robot in robots {
        let spanda_ast::nodes::RobotDecl::RobotDecl {
            name,
            mission,
            exposes_capabilities,
            ..
        } = robot;
        if mission.is_some()
            || exposes_capabilities
                .iter()
                .any(|c| c == "obstacle_avoidance")
        {
            let caps = ["obstacle_avoidance", "gps_navigation", "emergency_stop"];
            for cap in caps {
                if exposes_capabilities.iter().any(|c| c == cap) || mission.is_some() {
                    if let Some(report) = robot_reports.iter().find(|r| r.robot == *name) {
                        if !report.inferred.contains(&cap.to_string())
                            && !report.declared.contains(&cap.to_string())
                        {
                            let fixes =
                                suggest_fixes(cap, &["Lidar OR DepthCamera OR Radar".into()]);
                            rows.push(MinimumCapabilityRow {
                                capability: cap.into(),
                                required_by: name.clone(),
                                status: "FAIL".into(),
                                missing: vec!["minimum hardware".into()],
                                suggested_fixes: fixes,
                            });
                            errors.push(format!(
                                "Mission requires {cap}. Robot '{name}' lacks minimum hardware."
                            ));
                        }
                    }
                }
            }
        }
    }

    MinimumCapabilityReport {
        compatible: errors.is_empty(),
        rows,
        errors,
        warnings,
    }
}

fn check_requirement(
    req: &RequiresCapabilityDecl,
    robot_reports: &[crate::robot::RobotCapabilityReport],
) -> MinimumCapabilityRow {
    let Some(def) = lookup_capability(&req.capability) else {
        return MinimumCapabilityRow {
            capability: req.capability.clone(),
            required_by: req.required_by.clone().unwrap_or_else(|| "program".into()),
            status: "FAIL".into(),
            missing: vec!["unknown capability".into()],
            suggested_fixes: vec![],
        };
    };

    let satisfied = robot_reports.iter().any(|r| {
        r.rows
            .iter()
            .any(|row| row.capability == req.capability && row.status == "PASS")
    });

    let missing = if satisfied {
        Vec::new()
    } else {
        compute_missing(&def.minimum, &[])
    };

    MinimumCapabilityRow {
        capability: req.capability.clone(),
        required_by: req.required_by.clone().unwrap_or_else(|| "program".into()),
        status: if satisfied { "PASS" } else { "FAIL" }.into(),
        missing: missing.clone(),
        suggested_fixes: suggest_fixes(&req.capability, &missing),
    }
}

fn compute_missing(
    req: &crate::registry::CapabilityRequirement,
    _existing: &[String],
) -> Vec<String> {
    let mut missing = Vec::new();
    if !req.any_of_sensors.is_empty() {
        missing.push(format!("sensor: {}", req.any_of_sensors.join(" OR ")));
    }
    if !req.any_of_actuators.is_empty() {
        missing.push(format!("actuator: {}", req.any_of_actuators.join(" OR ")));
    }
    if !req.any_of_connectivity.is_empty() {
        missing.push(format!(
            "connectivity: {}",
            req.any_of_connectivity.join(" OR ")
        ));
    }
    if !req.required_packages.is_empty() {
        missing.push(format!("package: {}", req.required_packages.join(", ")));
    }
    if !req.required_safety_rules.is_empty() {
        missing.push(format!("safety: {}", req.required_safety_rules.join(", ")));
    }
    missing
}

fn suggest_fixes(capability: &str, missing: &[String]) -> Vec<String> {
    let mut fixes = Vec::new();
    for m in missing {
        if m.contains("Lidar") {
            fixes.push("Add Lidar sensor to hardware profile".into());
        }
        if m.contains("GPS") {
            fixes.push("Add GPS sensor to hardware profile".into());
        }
        if m.contains("spanda-nav") {
            fixes.push("Install spanda-nav package".into());
        }
    }
    if fixes.is_empty() {
        fixes.push(format!("Review minimum requirements for {capability}"));
    }
    fixes
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
    fn patrol_mission_missing_lidar_fails() {
        let source = r#"
hardware RoverV1 {
    sensors [GPS];
    actuators [DifferentialDrive];
}

robot Rover {
    uses hardware RoverV1;
    exposes capabilities [obstacle_avoidance, gps_navigation];
    mission Patrol { patrol_loop; }
}
"#;
        let program = parse_source(source);
        let report = check_minimum_capabilities(&program);
        assert!(!report.compatible || !report.errors.is_empty() || !report.rows.is_empty());
    }
}
