//! Integration tests for guardrailed generation and suggestions.

use spanda_generate::{
    generate_health_policy, generate_mission_program, generate_robot_program, suggest_program,
    GenerateOptions,
};
use spanda_lexer::tokenize;
use spanda_parser::parse;

fn parse_file(relative: &str) -> spanda_ast::nodes::Program {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative);
    let source =
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path}: {error}"));
    let tokens = tokenize(&source).expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn generated_mission_passes_validation() {
    let report = generate_mission_program(&GenerateOptions::default());
    assert!(report.validated, "{:?}", report.validation_error);
}

#[test]
fn generated_robot_passes_validation() {
    let report = generate_robot_program(&GenerateOptions::default());
    assert!(report.validated, "{:?}", report.validation_error);
}

#[test]
fn generated_health_policy_passes_validation() {
    let report = generate_health_policy(&GenerateOptions::default());
    assert!(report.validated, "{:?}", report.validation_error);
}

#[test]
fn suggest_readiness_rover_flags_kill_switch_gap() {
    let program = parse_file("../../examples/showcase/readiness/rover.sd");
    let report = suggest_program(&program, "rover.sd");
    assert!(!report.passed);
    assert!(report
        .suggestions
        .iter()
        .any(|item| item.message.contains("kill switch")));
}
