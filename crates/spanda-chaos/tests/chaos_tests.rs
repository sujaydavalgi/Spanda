//! Integration tests for chaos engineering experiments.

use spanda_chaos::{default_injections, run_chaos_experiment, ChaosExperimentOptions};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use std::path::PathBuf;

fn repo_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    for part in parts {
        path.push(part);
    }
    path
}

fn parse_file(path: PathBuf) -> spanda_ast::nodes::Program {
    let source = std::fs::read_to_string(&path).unwrap();
    let tokens = tokenize(&source).unwrap();
    parse(tokens).unwrap()
}

#[test]
fn self_healing_defaults_include_gps_and_lidar() {
    let program = parse_file(repo_path(&[
        "examples",
        "showcase",
        "self_healing",
        "rover.sd",
    ]));
    let kinds = default_injections(&program);
    assert!(kinds.iter().any(|k| k.contains("gps")));
    assert!(kinds.iter().any(|k| k.contains("lidar")));
}

#[test]
fn self_healing_passes_gps_chaos_injection() {
    let program = parse_file(repo_path(&[
        "examples",
        "showcase",
        "self_healing",
        "rover.sd",
    ]));
    let report = run_chaos_experiment(
        &program,
        "rover.sd",
        &ChaosExperimentOptions {
            injections: vec!["gps-failure".into()],
        },
    );
    let gps = report
        .injections
        .iter()
        .find(|result| result.injection == "gps-failure")
        .expect("gps injection result");
    assert!(gps.recovery_passed, "{:?}", gps.details);
    assert!(gps.passed, "{:?}", gps.details);
}

#[test]
fn readiness_rover_fails_default_chaos_without_recovery_policy() {
    let program = parse_file(repo_path(&[
        "examples",
        "showcase",
        "readiness",
        "rover.sd",
    ]));
    let report = run_chaos_experiment(
        &program,
        "rover.sd",
        &ChaosExperimentOptions {
            injections: vec!["gps-failure".into()],
        },
    );
    assert!(!report.passed);
}
