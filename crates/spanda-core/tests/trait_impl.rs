//! trait impl support for Spanda.
//!
use spanda_core::{check, run, RunOptions};

#[test]
fn trait_impl_binds_agent_method() {
    // Trait impl binds agent method.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::trait_impl::trait_impl_binds_agent_method();

    let source = r#"
struct Pose {
  x: Distance;
  y: Distance;
  heading: Angle;
}

trait Navigator {
  fn plan(goal: Pose) -> Path;
}

robot R {
  actuator wheels: DifferentialDrive;

  agent Nav {
    tools [wheels];
    goal "Navigate";
    plan { wheels.stop(); }
  }

  impl Navigator for Nav {
    fn plan(goal: Pose) -> Path {
        // Plan.
        //
        // Parameters:
        // - `goal` — input value
        //
        // Returns:
        // Path.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::trait_impl::plan(goal);

      // Produce stop as the result.
      wheels.stop();
}
  }

  behavior run() {
    Nav.plan(Pose { x: 0.0 m, y: 0.0 m, heading: 0.0 rad });
  }
}
"#;
    check(source).expect("trait impl should type-check");
    run(source, RunOptions::default()).expect("trait impl method should run");
}

#[test]
fn trait_impl_unknown_trait_rejected() {
    // Trait impl unknown trait rejected.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::trait_impl::trait_impl_unknown_trait_rejected();

    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  agent Nav { tools [wheels]; goal "x"; plan { wheels.stop(); } }
  impl Missing for Nav { fn plan(goal: Pose) -> Path { wheels.stop(); } }
}
"#;
    let err = check(source).expect_err("unknown trait should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("Unknown trait")),
        "got {:?}",
        err.diagnostics()
    );
}
