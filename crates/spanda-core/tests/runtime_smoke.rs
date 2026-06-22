//! Smoke tests for the interpreter runtime (moved from `runtime.rs`).
//!
use std::cell::RefCell;
use std::rc::Rc;

use spanda_core::lexer;
use spanda_core::parser;
use spanda_core::runtime::{Interpreter, InterpreterOptions};
use spanda_core::simulator::{create_default_simulator, Obstacle, SimulatorConfig};
use spanda_core::{RobotState, SpandaError};

fn compile_and_run(source: &str, max_iters: usize) -> Result<RobotState, SpandaError> {
    let tokens = lexer::tokenize(source)?;
    let program = parser::parse(tokens)?;
    let sim = create_default_simulator(SimulatorConfig {
        obstacles: vec![Obstacle {
            x: 100.0,
            y: 100.0,
            radius: 0.1,
        }],
        ..Default::default()
    });
    let mut interp = Interpreter::new(
        sim,
        InterpreterOptions {
            max_loop_iterations: max_iters,
            ..Default::default()
        },
    );
    interp.run(&program, None)
}

#[test]
fn executes_let_bindings_and_if_else() {
    let source = r#"
      robot R {
        sensor lidar: Lidar;
        actuator wheels: DifferentialDrive;
        behavior test() {
          let scan = lidar.read();
          if scan.nearest_distance < 0.5 m {
            wheels.stop();
          } else {
            wheels.drive(linear: 0.5 m/s, angular: 0.0 rad/s);
          }
        }
      }
    "#;
    let state = compile_and_run(source, 1).unwrap();
    assert!(state.velocity.linear > 0.0);
}

#[test]
fn runs_deterministic_loop() {
    let source = r#"
      robot R {
        actuator wheels: DifferentialDrive;
        behavior tick() {
          loop every 100ms {
            wheels.drive(linear: 0.1 m/s, angular: 0.0 rad/s);
          }
        }
      }
    "#;
    let state = compile_and_run(source, 5).unwrap();
    assert!(state.pose.x > 0.0);
}

#[test]
fn stops_on_close_obstacle() {
    let source = r#"
      robot R {
        sensor lidar: Lidar;
        actuator wheels: DifferentialDrive;
        behavior avoid() {
          loop every 50ms {
            let scan = lidar.read();
            if scan.nearest_distance < 0.5 m {
              wheels.stop();
            } else {
              wheels.drive(linear: 0.8 m/s, angular: 0.0 rad/s);
            }
          }
        }
      }
    "#;
    let tokens = lexer::tokenize(source).unwrap();
    let program = parser::parse(tokens).unwrap();
    let sim = create_default_simulator(SimulatorConfig {
        obstacles: vec![Obstacle {
            x: 0.3,
            y: 0.0,
            radius: 0.1,
        }],
        ..Default::default()
    });
    let mut interp = Interpreter::new(
        sim,
        InterpreterOptions {
            max_loop_iterations: 3,
            ..Default::default()
        },
    );
    let state = interp.run(&program, None).unwrap();
    assert_eq!(state.velocity.linear, 0.0);
}

#[test]
fn enforces_safety_in_interpreter() {
    let source = r#"
      robot R {
        sensor lidar: Lidar;
        actuator wheels: DifferentialDrive;
        safety {
          stop_if lidar.read().nearest_distance < 1.0 m;
        }
        behavior go() {
          wheels.drive(linear: 0.5 m/s, angular: 0.0 rad/s);
        }
      }
    "#;
    let tokens = lexer::tokenize(source).unwrap();
    let program = parser::parse(tokens).unwrap();
    let sim = create_default_simulator(SimulatorConfig {
        obstacles: vec![Obstacle {
            x: 0.5,
            y: 0.0,
            radius: 0.1,
        }],
        ..Default::default()
    });
    let blocked = Rc::new(RefCell::new(Vec::new()));
    let blocked_cb = blocked.clone();
    let mut interp = Interpreter::new(
        sim,
        InterpreterOptions {
            max_loop_iterations: 1,
            on_motion_blocked: Some(Rc::new(move |reason| {
                blocked_cb.borrow_mut().push(reason);
            })),
            ..Default::default()
        },
    );
    let state = interp.run(&program, None).unwrap();
    assert!(!blocked.borrow().is_empty());
    assert!(state.emergency_stop);
}
