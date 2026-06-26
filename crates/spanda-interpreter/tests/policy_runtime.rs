//! Interpreter runtime policy enforcement tests.

use spanda_interpreter::{run_program, RunOptions};
use spanda_lexer::tokenize;
use spanda_parser::parse;

fn parse_source(source: &str) -> spanda_ast::nodes::Program {
    let tokens = tokenize(source).expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn enforce_policy_blocks_drive_above_limit() {
    let source = r#"
policy SlowPolicy {
    max_speed = 0.2 m/s;
}

hardware RoverV1 {
  sensors [ GPS ];
  actuators [ DifferentialDrive ];
}

robot Rover {
  uses hardware RoverV1;
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }

  behavior patrol() {
    wheels.drive(linear: 0.5 m/s, angular: 0.0 rad/s);
  }
}
"#;
    let program = parse_source(source);
    let result = run_program(
        &program,
        RunOptions {
            entry_behavior: Some("patrol".into()),
            max_loop_iterations: 1,
            enforce_policy: Some("SlowPolicy".into()),
            ..Default::default()
        },
    )
    .expect("run");
    assert!(
        result.logs.iter().any(|line| line.contains("max_speed")),
        "expected runtime policy max_speed block log, got: {:?}",
        result.logs
    );
}
