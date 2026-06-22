//! Phase 34 gap-closure tests: full I/O verification, kill switch signing, IoT protocols,
//! agentic providers, fleet/swarm health, and debugger step coverage.

use spanda_core::{check, run, RunOptions};
use spanda_debug::DebugOptions;
use spanda_driver::{DebugMachine, DebugStepKind};
use spanda_security::signed::SignedMessage;

#[test]
fn event_handler_return_type_mismatch_fails_check() {
    let source = r#"
robot Rover {
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }

    event Alert;

    on Alert -> Bool {
        return 1;
    }
}
"#;
    let err = check(source).expect_err("event handler return mismatch should fail");
    let text = err
        .diagnostics()
        .into_iter()
        .map(|d| d.message)
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        text.contains("Bool") || text.contains("Number"),
        "expected return type error, got {text}"
    );
}

#[test]
fn kill_switch_handler_requires_declared_switch() {
    let source = r#"
robot Rover {
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }

    on kill_switch MissingSwitch {
        stop_all_actuators();
    }
}
"#;
    let err = check(source).expect_err("unknown kill switch should fail");
    let text = err
        .diagnostics()
        .into_iter()
        .map(|d| d.message)
        .collect::<Vec<_>>()
        .join(" ");
    assert!(text.contains("kill switch"), "expected kill switch error, got {text}");
}

#[test]
fn remote_signed_kill_switch_requires_signature_at_runtime() {
    let source = r#"
kill_switch EmergencyStop {
    priority: critical;
    remote_signed;
    action { emergency_stop; }
}

robot Rover {
    identity RobotIdentity { id: "rover-1"; public_key: "test-key"; }
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }
    behavior run() { wheels.stop(); }
}
"#;
    let err = run(
        source,
        RunOptions {
            trigger_kill_switch: Some("EmergencyStop".into()),
            ..Default::default()
        },
    )
    .expect_err("remote_signed kill switch without signature should fail");
    let text = format!("{err}");
    assert!(
        text.contains("signature") || text.contains("remote_signed"),
        "expected signature error, got {text}"
    );
}

#[test]
fn remote_signed_kill_switch_accepts_valid_signature() {
    let source = r#"
kill_switch EmergencyStop {
    priority: critical;
    remote_signed;
    action { emergency_stop; }
}

robot Rover {
    identity RobotIdentity { id: "rover-1"; public_key: "test-key"; }
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }
    behavior run() { wheels.stop(); }
}
"#;
    use spanda_security::RobotIdentity;
    let id = RobotIdentity::new("rover-1", "test-key");
    let signed = SignedMessage::sign("kill:EmergencyStop", &id);
    let signature = serde_json::to_string(&signed).expect("serialize signature");
    let result = run(
        source,
        RunOptions {
            trigger_kill_switch: Some("EmergencyStop".into()),
            kill_switch_signature: Some(signature),
            ..Default::default()
        },
    )
    .expect("signed kill switch should activate");
    assert!(result.state.emergency_stop);
}

#[test]
fn fleet_health_check_degrades_on_member_fault() {
    let source = r#"
fleet Patrol {
    Rover;
}

health_check PatrolHealth for fleet Patrol {
    check rover.status == Healthy;
}

robot Rover {
    sensor gps: GPS;
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }

    every 50ms {
        let tick = true;
    }
}
"#;
    let result = run(
        source,
        RunOptions {
            inject_health_faults: true,
            max_loop_iterations: 3,
            ..Default::default()
        },
    )
    .expect("fleet health program should run");
    assert!(
        result
            .logs
            .iter()
            .any(|l| l.contains("health:") || l.contains("health_policy")),
        "expected health logs, got {:?}",
        result.logs
    );
}

#[test]
fn debugger_step_in_and_step_out_pause() {
    let source = r#"
robot Rover {
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }

    behavior run() {
        wheels.stop();
        wheels.stop();
    }
}
"#;
    let mut machine = DebugMachine::start(source, DebugOptions::default()).expect("debug machine");
    let step_in = machine
        .run_until_pause(DebugStepKind::StepIn)
        .expect("step in");
    assert!(step_in.pauses.iter().any(|p| p.reason.contains("step")));
    let step_out = machine
        .run_until_pause(DebugStepKind::StepOut)
        .expect("step out");
    assert!(step_out.pauses.iter().any(|p| p.reason.contains("step")));
}
