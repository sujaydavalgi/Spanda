use spanda_core::{check, compile, run, RunOptions};

#[test]
fn event_trigger_dispatch() {
    let source = r#"
robot Rover {
  actuator wheels: DifferentialDrive;

  event ObstacleDetected;

  on ObstacleDetected {
    wheels.stop();
  }

  behavior run() {
    emit ObstacleDetected;
  }
}
"#;
    compile(source).expect("event trigger should compile");
    let result = run(
        source,
        RunOptions {
            max_loop_iterations: 1,
            trace_events: true,
            ..Default::default()
        },
    )
    .expect("event trigger should run");
    assert!(result
        .logs
        .iter()
        .any(|l| l.contains("emit ObstacleDetected")));
    assert!(result.logs.iter().any(|l| l.contains("trace-event:")));
}

#[test]
fn message_trigger_on_publish() {
    let source = r#"
robot Rover {
  topic lidar_scan: String subscribe on "/scan";
  actuator wheels: DifferentialDrive;

  on lidar_scan {
    wheels.stop();
  }

  behavior run() {
    publish lidar_scan with "ok";
  }
}
"#;
    check(source).expect("message trigger should type-check");
    let result = run(source, RunOptions::default()).expect("message trigger should run");
    assert!(result.logs.iter().any(|l| l.contains("publish /scan")));
}

#[test]
fn timer_trigger_every() {
    let source = r#"
robot Rover {
  topic status: String publish on "/status";

  every 100ms {
    publish status with "tick";
  }
}
"#;
    compile(source).expect("timer trigger should compile");
    let result = run(
        source,
        RunOptions {
            max_loop_iterations: 3,
            trace_triggers: true,
            ..Default::default()
        },
    )
    .expect("timer trigger should run");
    assert!(
        result
            .logs
            .iter()
            .filter(|l| l.contains("publish /status"))
            .count()
            >= 2
    );
}

#[test]
fn condition_trigger_when() {
    let source = r#"
robot Rover {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    stop_if lidar.nearest_distance < 1.0 m;
  }

  when lidar.nearest_distance < 1.0 m {
    wheels.stop();
  }

  behavior run() {
    wheels.drive(linear: 0.5 m/s, angular: 0.0 rad/s);
  }
}
"#;
    check(source).expect("condition trigger should type-check");
}

#[test]
fn safety_trigger_priority_critical() {
    let source = r#"
robot Rover {
  actuator wheels: DifferentialDrive;

  on safety EmergencyStop priority critical {
    wheels.stop();
  }

  behavior run() {
    emergency_stop;
  }
}
"#;
    let result = run(source, RunOptions::default()).expect("safety trigger should run");
    assert!(result.state.emergency_stop);
    assert!(result.logs.iter().any(|l| l.contains("system trigger")));
}

#[test]
fn state_trigger_entered() {
    let source = r#"
robot Rover {
  state_machine Nav {
    state Idle;
    state Navigating;
    transition Idle -> Navigating;
  }

  on state Entered(Navigating) {
    let started = true;
  }

  behavior run() {
    enter Navigating;
  }
}
"#;
    check(source).expect("state trigger should type-check");
    run(source, RunOptions::default()).expect("state trigger should run");
}

#[test]
fn verification_trigger_syntax() {
    let source = r#"
robot Rover {
  on verification Failed {
    let blocked = true;
  }

  verify {
    true;
  }
}
"#;
    check(source).expect("verification trigger should type-check");
}

#[test]
fn agent_message_trigger() {
    let source = r#"
robot Rover {
  topic camera_frame: String subscribe on "/camera";

  agent Vision {
    on camera_frame {
      let frame = "received";
    }
  }
}
"#;
    check(source).expect("agent trigger should type-check");
}

#[test]
fn condition_trigger_rejects_non_boolean() {
    let source = r#"
robot Rover {
  when 42 {
    let x = 1;
  }
}
"#;
    assert!(
        check(source).is_err(),
        "non-boolean when should fail type-check"
    );
}

#[test]
fn trigger_metrics_recorded() {
    let source = r#"
robot Rover {
  event Tick;
  on Tick { let x = 1; }
  behavior run() { emit Tick; }
}
"#;
    let result = run(
        source,
        RunOptions {
            max_loop_iterations: 1,
            trace_triggers: true,
            ..Default::default()
        },
    )
    .expect("trigger metrics run");
    assert!(!result.metrics.triggers.is_empty());
}

#[test]
fn hardware_trigger_from_simulate_compatibility() {
    let source = r#"
simulate_compatibility {
  fault LidarFailure;
}

robot Rover {
  sensor lidar: Lidar on "/scan";

  on hardware LidarFailure {
    let fault = true;
  }

  every 50ms {
    let scan = lidar.read();
  }
}
"#;
    let result = run(
        source,
        RunOptions {
            max_loop_iterations: 2,
            ..Default::default()
        },
    )
    .expect("hardware trigger run");
    assert!(result.logs.iter().any(|l| l.contains("system trigger")));
}

#[test]
fn ai_goal_completed_trigger() {
    let source = r#"
robot Rover {
  ai_model planner: LLM {
    provider: "mock";
    model: "test";
  }

  on ai GoalCompleted {
    let done = true;
  }

  agent Nav {
    uses planner;
    plan {
      let x = 1;
    }
  }

  behavior run() {
    Nav.plan();
  }
}
"#;
    let result = run(source, RunOptions::default()).expect("ai goal trigger");
    assert!(
        result
            .logs
            .iter()
            .any(|l| l.contains("Ai") && l.contains("GoalCompleted"))
            || result.logs.iter().any(|l| l.contains("system trigger"))
    );
}

#[test]
fn ai_confidence_low_trigger() {
    let source = r#"
robot Rover {
  sensor lidar: Lidar on "/scan";

  ai_model planner: LLM {
    provider: "mock";
    model: "test";
  }

  on ai ConfidenceLow {
    let review = true;
  }

  behavior run() {
    let scan = lidar.read();
    let proposal = planner.reason(prompt: "avoid obstacle", input: scan);
  }
}
"#;
    check(source).expect("confidence trigger should type-check");
}

#[test]
fn verification_warning_trigger() {
    let source = r#"
robot Rover {
  on verification Warning {
    let review = true;
  }

  verify {
    warning false;
  }

  behavior run() {
    let x = 1;
  }
}
"#;
    let result = run(source, RunOptions::default()).expect("verification warning");
    assert!(result.logs.iter().any(|l| l.contains("verify warning")));
}

#[test]
fn while_level_condition_trigger() {
    let source = r#"
robot Rover {
  topic status: String publish on "/status";

  while true {
    publish status with "heartbeat";
  }
}
"#;
    let result = run(
        source,
        RunOptions {
            max_loop_iterations: 3,
            ..Default::default()
        },
    )
    .expect("while trigger");
    assert!(
        result
            .logs
            .iter()
            .filter(|l| l.contains("publish /status"))
            .count()
            >= 2
    );
}

#[test]
fn twin_fault_injected_trigger() {
    let source = r#"
simulate_compatibility {
  fault InjectedFault;
}

robot Rover {
  on twin FaultInjected {
    let alert = true;
  }

  twin DemoTwin {
    mirror pose;
    replay true;
  }

  every 50ms {
    let x = 1;
  }
}
"#;
    let result = run(
        source,
        RunOptions {
            max_loop_iterations: 2,
            ..Default::default()
        },
    )
    .expect("twin fault trigger");
    assert!(result.logs.iter().any(|l| l.contains("system trigger")));
}
