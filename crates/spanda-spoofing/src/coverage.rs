//! Static spoofing-detection coverage analysis for parsed programs.

use serde::{Deserialize, Serialize};
use spanda_ast::assurance_decl::StateEstimatorDecl;
use spanda_ast::foundations::{HardwareDecl, HealthCheckDecl, TriggerHandlerDecl, TriggerKind};
use spanda_ast::nodes::{Program, RobotDecl, SensorDecl};

/// One static coverage check for spoofing readiness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpoofingCoverageCheck {
    pub id: String,
    pub label: String,
    pub passed: bool,
    pub weight: u32,
    pub detail: Option<String>,
}

/// Analyze whether a program declares enough hooks to detect GPS/sensor spoofing.
pub fn analyze_spoofing_coverage(program: &Program) -> (u32, Vec<SpoofingCoverageCheck>) {
    // Score spoofing detection readiness from GPS sensors, fusion, handlers, and health bounds.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    //
    // Returns:
    // Coverage score (0–100) and per-check results.
    //
    // Options:
    // None.
    //
    // Example:
    // let (score, checks) = analyze_spoofing_coverage(&program);

    let Program::Program {
        hardware_profiles,
        requires_connectivity,
        geofences,
        state_estimators,
        health_checks,
        robots,
        ..
    } = program;

    let has_gps_hardware = hardware_profiles.iter().any(hardware_has_gps);
    let has_gps_robot_sensor = robots.iter().any(robot_has_gps_sensor);
    let has_gps = has_gps_hardware || has_gps_robot_sensor;
    let has_cross_sensor = state_estimators.iter().any(estimator_cross_checks_gps);
    let has_spoof_handler = robots.iter().any(|robot| match robot {
        RobotDecl::RobotDecl {
            trigger_handlers, ..
        } => trigger_list_has_spoof_handler(trigger_handlers),
    });
    let has_gps_health = health_checks.iter().any(health_check_targets_gps)
        || robots.iter().any(|robot| match robot {
            RobotDecl::RobotDecl {
                health_checks, ..
            } => health_checks.iter().any(health_check_targets_gps),
        });
    let has_geofence = !geofences.is_empty();
    let has_connectivity_requirement = requires_connectivity.is_some();

    let checks = vec![
        SpoofingCoverageCheck {
            id: "gps_sensor".into(),
            label: "GPS sensor declared on hardware or robot".into(),
            passed: has_gps,
            weight: 25,
            detail: if has_gps {
                None
            } else {
                Some("Add GPS to hardware sensors or declare a robot gps sensor".into())
            },
        },
        SpoofingCoverageCheck {
            id: "cross_sensor_fusion".into(),
            label: "State estimator fuses GPS with IMU or odometry".into(),
            passed: has_cross_sensor,
            weight: 30,
            detail: if has_cross_sensor {
                None
            } else {
                Some(
                    "Declare state_estimator with gps.fix plus imu.data or wheel_odometry inputs"
                        .into(),
                )
            },
        },
        SpoofingCoverageCheck {
            id: "spoof_handler".into(),
            label: "Handler for gps.spoofed connectivity trigger".into(),
            passed: has_spoof_handler,
            weight: 25,
            detail: if has_spoof_handler {
                None
            } else {
                Some("Add `on gps.spoofed { ... }` to audit and react to spoof events".into())
            },
        },
        SpoofingCoverageCheck {
            id: "gps_health_bounds".into(),
            label: "Health check bounds on GPS status or fix quality".into(),
            passed: has_gps_health,
            weight: 10,
            detail: if has_gps_health {
                None
            } else {
                Some("Add health_check conditions on gps.status or gps.fix_quality".into())
            },
        },
        SpoofingCoverageCheck {
            id: "geofence_plausibility".into(),
            label: "Geofence for positional plausibility".into(),
            passed: has_geofence,
            weight: 5,
            detail: if has_geofence {
                None
            } else {
                Some("Optional: geofence helps reject impossible off-mission coordinates".into())
            },
        },
        SpoofingCoverageCheck {
            id: "connectivity_requirement".into(),
            label: "requires_connectivity declares GPS dependency".into(),
            passed: has_connectivity_requirement,
            weight: 5,
            detail: if has_connectivity_requirement {
                None
            } else {
                Some("Optional: requires_connectivity { gps: required; } documents GNSS need".into())
            },
        },
    ];

    let earned: u32 = checks
        .iter()
        .filter(|check| check.passed)
        .map(|check| check.weight)
        .sum();
    let total: u32 = checks.iter().map(|check| check.weight).sum();
    let score = if total == 0 {
        0
    } else {
        ((earned as f64 / total as f64) * 100.0).round() as u32
    };

    (score, checks)
}

fn hardware_has_gps(profile: &HardwareDecl) -> bool {
    match profile {
        HardwareDecl::HardwareDecl { sensors, .. } => {
            sensors.iter().any(|sensor| sensor_is_gps(sensor))
        }
    }
}

fn robot_has_gps_sensor(robot: &RobotDecl) -> bool {
    match robot {
        RobotDecl::RobotDecl { sensors, .. } => sensors.iter().any(sensor_decl_is_gps),
    }
}

fn sensor_decl_is_gps(sensor: &SensorDecl) -> bool {
    match sensor {
        SensorDecl::SensorDecl {
            name,
            sensor_type,
            ..
        } => sensor_is_gps(sensor_type) || name.to_ascii_lowercase().contains("gps"),
    }
}

fn sensor_is_gps(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == "gps" || lower == "gnss"
}

fn estimator_cross_checks_gps(estimator: &StateEstimatorDecl) -> bool {
    match estimator {
        StateEstimatorDecl::StateEstimatorDecl { inputs, .. } => {
            let has_gps_input = inputs.iter().any(|input| input.to_ascii_lowercase().contains("gps"));
            let has_inertial = inputs.iter().any(|input| {
                let lower = input.to_ascii_lowercase();
                lower.contains("imu")
                    || lower.contains("odometry")
                    || lower.contains("wheel")
                    || lower.contains("encoder")
            });
            has_gps_input && has_inertial
        }
    }
}

fn trigger_list_has_spoof_handler(handlers: &[TriggerHandlerDecl]) -> bool {
    handlers.iter().any(|handler| match handler {
        TriggerHandlerDecl::TriggerHandlerDecl { trigger_kind, .. } => {
            matches!(
                trigger_kind,
                TriggerKind::Connectivity { domain, event }
                    if sensor_is_gps(domain) && event == "spoofed"
            )
        }
    })
}

fn health_check_targets_gps(check: &HealthCheckDecl) -> bool {
    match check {
        HealthCheckDecl::HealthCheckDecl { conditions, .. } => conditions.iter().any(|cond| {
            let metric = cond.metric.to_ascii_lowercase();
            metric.contains("gps") || metric.contains("gnss") || metric.contains("fix_quality")
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;

    fn parse_fixture(source: &str) -> Program {
        let tokens = tokenize(source).expect("tokenize");
        parse(tokens).expect("parse")
    }

    #[test]
    fn full_coverage_program_scores_high() {
        let source = r#"
hardware RoverV1 {
  sensors [ GPS, IMU ];
  actuators [ DifferentialDrive ];
  connectivity [ GPS ];
}

requires_connectivity {
  gps: required;
}

state_estimator RoverState {
  inputs [gps.fix, imu.data];
  output StateEstimate;
}

geofence Zone {
  center: geo(37.0, -122.0);
  radius: 100.0;
}

health_check GpsHealth for robot Rover {
  check gps.status == Healthy;
}

robot Rover {
  sensor gps: GPS;
  sensor imu: IMU;
  on gps.spoofed { audit.record("spoof"); }
}

deploy Rover to RoverV1;
"#;
        let program = parse_fixture(source);
        let (score, checks) = analyze_spoofing_coverage(&program);
        assert!(score >= 90, "score={score} checks={checks:?}");
        assert!(checks.iter().find(|c| c.id == "spoof_handler").unwrap().passed);
    }

    #[test]
    fn bare_gps_program_flags_missing_fusion_and_handler() {
        let source = r#"
hardware RoverV1 { sensors [ GPS ]; actuators [ DifferentialDrive ]; }
robot Rover { sensor gps: GPS; }
deploy Rover to RoverV1;
"#;
        let program = parse_fixture(source);
        let (score, checks) = analyze_spoofing_coverage(&program);
        assert!(score < 60, "score={score}");
        assert!(!checks
            .iter()
            .find(|c| c.id == "cross_sensor_fusion")
            .unwrap()
            .passed);
        assert!(!checks.iter().find(|c| c.id == "spoof_handler").unwrap().passed);
    }
}
