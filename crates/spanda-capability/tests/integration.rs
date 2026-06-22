//! Integration tests for capability, traceability, health, and minimum-hardware checks.

use spanda_capability::{
    capability_traceability, check_minimum_capabilities, evaluate_health_checks,
    hardware_traceability, infer_robot_capabilities, lookup_capability,
};
use spanda_lexer::tokenize;
use spanda_parser::parse;

fn parse_source(source: &str) -> spanda_ast::nodes::Program {
    parse(tokenize(source).expect("tokenize")).expect("parse")
}

#[test]
fn capability_registry_lookup() {
    assert!(lookup_capability("gps_navigation").is_some());
    assert!(lookup_capability("emergency_stop").is_some());
}

#[test]
fn hardware_traceability_matrix() {
    let source = r#"
hardware RoverV1 {
    sensors [GPS, Lidar];
    actuators [DifferentialDrive];
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
}

#[test]
fn robot_capability_inference() {
    let source = r#"
robot Rover {
    sensor gps: GPS;
    actuator wheels: DifferentialDrive;
    exposes capabilities [gps_navigation];
}
"#;
    let program = parse_source(source);
    let reports = infer_robot_capabilities(&program);
    assert_eq!(reports.len(), 1);
    assert!(reports[0]
        .rows
        .iter()
        .any(|r| r.capability == "gps_navigation"));
}

#[test]
fn minimum_capability_check_fails_without_lidar() {
    let source = r#"
robot Rover {
    sensor gps: GPS;
    actuator wheels: DifferentialDrive;
    mission Patrol { requires capabilities [obstacle_avoidance]; }
}
"#;
    let program = parse_source(source);
    let report = check_minimum_capabilities(&program);
    assert!(!report.compatible || !report.errors.is_empty());
}

#[test]
fn health_check_parsing() {
    let source = r#"
health_check RoverHealth for robot Rover {
    check battery.level > 20%;
}

health_policy SafetyPolicy {
    on Critical { enter degraded_mode; }
}
"#;
    let program = parse_source(source);
    let report = evaluate_health_checks(&program);
    assert!(!report.checks.is_empty());
    assert!(!report.policies.is_empty());
}

#[test]
fn kill_switch_traceability() {
    let source = r#"
kill_switch EmergencyStop {
    priority: critical;
    action { emergency_stop; }
}
"#;
    let program = parse_source(source);
    let report = capability_traceability(&program);
    assert!(report
        .hardware_rows
        .iter()
        .any(|r| r.capability == "emergency_stop"));
}

#[test]
fn capability_traceability_json_shape() {
    let source = r#"
requires_capability gps_navigation {
    any_of sensors [GPS, GNSS];
    any_of actuators [DifferentialDrive];
}

robot Rover {
    sensor gps: GPS;
    actuator wheels: DifferentialDrive;
}
"#;
    let program = parse_source(source);
    let report = capability_traceability(&program);
    let json = serde_json::to_string(&report).expect("json");
    assert!(json.contains("hardware_rows") || json.contains("capability_rows"));
}
