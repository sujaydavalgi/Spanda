use spanda_core::{check, compile, run, RunOptions};

#[test]
fn foundation_types_with_annotations() {
    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let count: Int = 3;
    let label: String = "rover";
    let active: Bool = true;
    let _ = count;
    wheels.stop();
  }
}
"#;
    check(source).expect("foundation types should type-check");
}

#[test]
fn generic_collections_type_check() {
    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let goals: Array<Goal> = goals_placeholder;
    let _ = goals;
    wheels.stop();
  }
}
"#;
    let err = check(source).expect_err("undefined goals_placeholder should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("Undefined")),
        "got {:?}",
        err.diagnostics()
    );
    let parse_only = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let goals: Array<Goal>;
    let scan_topic: Topic<LidarScan>;
    let svc: Service<Command, Feedback>;
    wheels.stop();
  }
}
"#;
    check(parse_only).expect("generic type annotations should parse and type-check");
}

#[test]
fn generic_arity_mismatch_fails() {
    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let bad: Array<Int, Float>;
    wheels.stop();
  }
}
"#;
    let err = check(source).expect_err("Array arity mismatch should fail at parse");
    assert!(
        err.to_string().contains("expects 1") || err.diagnostics().iter().any(|d| d.message.contains("expects 1")),
        "got {err}"
    );
}

#[test]
fn unit_literals_and_valid_operations() {
    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let timeout: Duration = 500 ms;
    let speed: Velocity = 1.5 m/s;
    let distance: Distance = 2.0 m;
    let total: Distance = distance + 1.0 m;
    let _ = total;
    wheels.stop();
  }
}
"#;
    check(source).expect("valid unit operations should pass");
}

#[test]
fn invalid_unit_operation_fails() {
    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let speed: Velocity = 1.0 m/s;
    let distance: Distance = 2.0 m;
    let bad = speed + distance;
    wheels.stop();
  }
}
"#;
    let err = check(source).expect_err("speed + voltage should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("incompatible")),
        "got {:?}",
        err.diagnostics()
    );
}

#[test]
fn distance_plus_duration_fails() {
    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let d: Distance = 1.0 m;
    let t: Duration = 500 ms;
    let bad = d + t;
    wheels.stop();
  }
}
"#;
    let err = check(source).expect_err("distance + duration should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("incompatible")),
        "got {:?}",
        err.diagnostics()
    );
}

#[test]
fn spatial_sensor_and_ai_types_parse() {
    let source = r#"
robot R {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: "mock"; model: "p"; temperature: 0.1; }
  behavior run() {
    let pose: Pose;
    let path: Path;
    let scan: LidarScan;
    let frame: CameraFrame;
    let prompt: Prompt;
    wheels.stop();
  }
}
"#;
    check(source).expect("spatial/sensor/ai annotations should type-check");
}

#[test]
fn action_proposal_cannot_execute_directly() {
    let source = r#"
robot R {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: "mock"; model: "p"; temperature: 0.1; }
  behavior run() {
    let proposal: ActionProposal = planner.reason(prompt: "go");
    wheels.execute(proposal);
  }
}
"#;
    let err = check(source).expect_err("ActionProposal execute should fail typecheck");
    assert!(
        err.diagnostics().iter().any(|d| {
            d.message.contains("ActionProposal")
                && d.message.contains("execute")
        }),
        "got {:?}",
        err.diagnostics()
    );
}

#[test]
fn safe_action_can_execute() {
    let source = r#"
robot R {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: "mock"; model: "p"; temperature: 0.1; }
  safety { max_speed = 1.0 m/s; }
  behavior run() {
    let proposal: ActionProposal = planner.reason(prompt: "go");
    let action: SafeAction = safety.validate(proposal);
    wheels.execute(action);
  }
}
"#;
    check(source).expect("SafeAction execute should type-check");
}

#[test]
fn unknown_type_fails_at_parse() {
    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let x: NotARealType;
    wheels.stop();
  }
}
"#;
    let err = check(source).expect_err("unknown type should fail");
    assert!(
        err.to_string().contains("Unknown type") || err.diagnostics().iter().any(|d| d.message.contains("Unknown type")),
        "got {err}"
    );
}

#[test]
fn safety_example_runs() {
    let source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/types/safety.sd"
    ))
    .expect("read safety example");
    compile(&source).expect("safety example should compile");
    run(&source, RunOptions::default()).expect("safety example should run");
}
