//! Integration tests for mission differencing.

use spanda_diff::{diff_programs_with_capabilities, DiffChangeKind, MissionDiffDimension};
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
fn readiness_and_safety_rover_diff_detects_hardware_and_safety_changes() {
    let baseline = parse_file(repo_path(&[
        "examples",
        "showcase",
        "readiness",
        "rover.sd",
    ]));
    let candidate = parse_file(repo_path(&[
        "examples",
        "showcase",
        "safety_report",
        "rover.sd",
    ]));
    let report = diff_programs_with_capabilities(&baseline, &candidate, "readiness", "safety");
    assert!(!report.changes.is_empty());
    assert!(report.has_safety_impact || report.has_deploy_impact);
    assert!(report.changes.iter().any(|change| {
        matches!(
            change.dimension,
            MissionDiffDimension::Hardware | MissionDiffDimension::KillSwitch
        ) || change.kind == DiffChangeKind::Added
    }));
}

#[test]
fn identical_program_has_no_diff() {
    let program = parse_file(repo_path(&[
        "examples",
        "showcase",
        "readiness",
        "rover.sd",
    ]));
    let report = diff_programs_with_capabilities(&program, &program, "a", "b");
    assert!(report.changes.is_empty());
}
