//! Phase 33 gap-closure tests: trigger return types, live IoT env gates, live AI provider selection.

use spanda_core::{check, run, RunOptions};

#[test]
fn trigger_return_type_mismatch_fails_check() {
    let source = r#"
robot Rover {
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }

    every 100ms -> Bool {
        return 1;
    }
}
"#;
    let err = check(source).expect_err("trigger return mismatch should fail");
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
fn trigger_return_type_match_passes_check() {
    let source = r#"
robot Rover {
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }

    when true -> Bool {
        return true;
    }
}
"#;
    check(source).expect("valid trigger return should pass");
}

#[test]
fn live_modbus_env_gate_defaults_off() {
    std::env::remove_var("SPANDA_LIVE_MODBUS");
    assert!(!spanda_providers::iot_live::live_modbus_enabled());
}

#[test]
fn live_ai_env_gate_requires_api_key() {
    std::env::remove_var("OPENAI_API_KEY");
    std::env::remove_var("SPANDA_LIVE_AI");
    assert!(!spanda_ai::live::live_ai_enabled());
}

#[test]
fn openai_provider_uses_mock_without_api_key() {
    std::env::remove_var("OPENAI_API_KEY");
    let source = r#"
robot Rover {
    sensor lidar: Lidar on "/scan";
    ai_model planner: LLM { provider: "openai"; model: "gpt-4o-mini"; }
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }

    behavior run() {
        let scan = lidar.read();
        let proposal = planner.reason(prompt: "go forward", input: scan);
        let action = safety.validate(proposal);
        wheels.execute(action);
    }
}
"#;
    run(source, RunOptions::default()).expect("mock fallback should run without API key");
}
