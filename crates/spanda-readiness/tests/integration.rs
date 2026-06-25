//! Integration tests for operational readiness engine.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_readiness::{
    analyze_failure, audit_program, build_runtime_context, evaluate_agent_readiness_json,
    evaluate_fleet_readiness, evaluate_readiness, evaluate_readiness_with_runtime,
    readiness_options_from_flags, readiness_traceability, verify_approvals, verify_fleet,
    verify_mission, ReadinessOptions,
};

fn parse_source(source: &str) -> spanda_ast::nodes::Program {
    // Description:
    //     Parse source.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: spanda_ast::nodes::Program
    //         Return value from `parse_source`.
    //
    // Example:

    //     let result = spanda_readiness::integration::parse_source(source);

    parse(tokenize(source).expect("tokenize")).expect("parse")
}

const ROVER: &str = include_str!("../../../examples/showcase/capability_verification/rover.sd");

const WAREHOUSE_MISSION: &str = r#"
hardware RoverV1 {
  sensors [ GPS, Lidar ];
  actuators [ DifferentialDrive ];
}

robot Rover {
  uses hardware RoverV1;
  sensor gps: GPS;
  sensor lidar: Lidar;
  actuator wheels: DifferentialDrive;
  exposes capabilities [ gps_navigation, obstacle_avoidance ];
  mission WarehousePatrol {
    requires capabilities [ obstacle_avoidance, gps_navigation ];
    patrol;
  }
  behavior patrol() {
    loop every 100ms { wheels.drive(linear: 0.1 m/s, angular: 0.0 rad/s); }
  }
}
"#;

const APPROVAL_MISSION: &str = r#"
robot GateBot {
  actuator gate: DifferentialDrive;
  topic gate_approval: Approval publish on "/gate/approval";
  mode hold { gate.stop(); }
  mission OpenGate {
    requires approval Operator for: open_gate;
    open_sequence;
  }
  behavior open_sequence() { gate.drive(linear: 0.0 m/s, angular: 0.0 rad/s); }
}
"#;

const FLEET: &str = include_str!("../../../examples/showcase/fleet_readiness/warehouse.sd");

#[test]
fn agent_readiness_json_matches_http_envelope() {
    // Description:
    //     Agent readiness json matches http envelope.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::agent_readiness_json_matches_http_envelope();

    let json =
        evaluate_agent_readiness_json(ROVER, None, false, false).expect("agent readiness json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("parse json");
    assert_eq!(value.get("ok").and_then(|v| v.as_bool()), Some(true));
    assert!(value.get("mission_ready").is_some());
    assert!(value.get("readiness").is_some());
}

#[test]
fn readiness_engine_produces_score() {
    // Description:
    //     Readiness engine produces score.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::readiness_engine_produces_score();

    let program = parse_source(ROVER);
    let report = evaluate_readiness(&program, &ReadinessOptions::default());
    assert!(report.score.total > 0);
    assert!(!report.robots.is_empty());
}

#[test]
fn readiness_runtime_injects_health_faults() {
    // Description:
    //     Readiness runtime injects health faults.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::readiness_runtime_injects_health_faults();

    let program = parse_source(ROVER);
    let options = ReadinessOptions {
        include_runtime: true,
        inject_health_faults: true,
        ..ReadinessOptions::default()
    };
    let ctx = build_runtime_context(&program, true);
    let report = evaluate_readiness_with_runtime(&program, &options, Some(&ctx));
    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.factor == "Health" && issue.message.contains("Runtime status")),
        "expected runtime health issues: {:?}",
        report.issues
    );
    assert!(report
        .score
        .factors
        .iter()
        .any(|f| f.factor == "Health" && f.score < 100));
}

#[test]
fn readiness_target_flag_selects_deploy_profile() {
    // Description:
    //     Readiness target flag selects deploy profile.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::readiness_target_flag_selects_deploy_profile();

    let program = parse_source(FLEET);
    let options =
        readiness_options_from_flags(&program, Some("edge".into()), false, false, false, false);
    assert_eq!(options.target.as_deref(), Some("edge"));
    let report = evaluate_readiness(&program, &options);
    assert_eq!(report.target.as_deref(), Some("edge"));
}

#[test]
fn mission_verification_achievable() {
    // Description:
    //     Mission verification achievable.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::mission_verification_achievable();

    let program = parse_source(WAREHOUSE_MISSION);
    let reports = verify_mission(&program, None);
    assert!(!reports.is_empty());
    let report = reports.first().unwrap();
    assert!(report.capabilities_satisfied);
    assert_eq!(report.required_capabilities.len(), 2);
}

#[test]
fn failure_analysis_lists_impacts() {
    // Description:
    //     Failure analysis lists impacts.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::failure_analysis_lists_impacts();

    let program = parse_source(ROVER);
    let report = analyze_failure(&program);
    assert!(!report.impacts.is_empty());
    assert!(report.impacts.iter().any(|i| i.component == "GPS"));
}

#[test]
fn fleet_readiness_aggregates() {
    // Description:
    //     Fleet readiness aggregates.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::fleet_readiness_aggregates();

    let program = parse_source(FLEET);
    let report = evaluate_fleet_readiness(&program, &ReadinessOptions::default());
    assert_eq!(
        report.healthy_robots + report.degraded_robots + report.not_ready_robots,
        2
    );
}

#[test]
fn safety_auditor_flags_missing_kill_switch() {
    // Description:
    //     Safety auditor flags missing kill switch.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::safety_auditor_flags_missing_kill_switch();

    let program = parse_source(ROVER);
    let report = audit_program(&program, ROVER);
    assert!(report.critical_count >= 1);
}

#[test]
fn approval_verification_passes_with_topic() {
    // Description:
    //     Approval verification passes with topic.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::approval_verification_passes_with_topic();

    let program = parse_source(APPROVAL_MISSION);
    let report = verify_approvals(&program);
    assert!(report.compatible);
}

#[test]
fn fleet_verify_detects_multi_robot() {
    // Description:
    //     Fleet verify detects multi robot.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::fleet_verify_detects_multi_robot();

    let program = parse_source(FLEET);
    let report = verify_fleet(&program);
    assert!(!report.findings.is_empty());
}

#[test]
fn readiness_traceability_has_rows() {
    // Description:
    //     Readiness traceability has rows.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::readiness_traceability_has_rows();

    let program = parse_source(ROVER);
    let rows = readiness_traceability(&program);
    assert!(!rows.is_empty());
}

#[test]
fn root_cause_diagnose_trace() {
    // Description:
    //     Root cause diagnose trace.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::integration::root_cause_diagnose_trace();

    let trace = include_str!("../../../examples/showcase/root_cause_analysis/mission.trace");
    let path = std::env::temp_dir().join("spanda_test_mission.trace");
    std::fs::write(&path, trace).unwrap();
    let report = spanda_readiness::diagnose_trace(&path).expect("diagnose");
    assert!(!report.root_cause.is_empty());
    assert!(!report.timeline.is_empty());
}

#[test]
fn readiness_surfaces_missing_agent_attestation() {
    let expected = spanda_config::ExpectedAgentState {
        target_key: "Rover@Jetson".into(),
        robot_name: "Rover".into(),
        hardware_profile: Some("Jetson".into()),
        program_hash: None,
        firmware_by_device: std::collections::HashMap::new(),
        packages: Vec::new(),
        attestation_contracts: vec!["trust.jetson".into()],
    };
    let actual = spanda_config::AgentDriftSnapshot {
        agent_id: "Rover@Jetson".into(),
        healthy: true,
        ..spanda_config::AgentDriftSnapshot::default()
    };
    let program = parse_source(include_str!(
        "../../../examples/showcase/secure_boot/rover.sd"
    ));
    let options = ReadinessOptions {
        agent_drift: vec![(expected, actual)],
        ..ReadinessOptions::default()
    };
    let report = evaluate_readiness(&program, &options);
    assert!(report.issues.iter().any(|issue| {
        issue.factor == "Attestation" && issue.message.contains("missing attestation")
    }));
}
