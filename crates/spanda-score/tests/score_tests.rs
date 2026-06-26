//! Integration tests for scorecard rollup.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_score::{evaluate_scorecard, ScorecardOptions};
use std::path::PathBuf;

fn repo_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    for part in parts {
        path.push(part);
    }
    path
}

#[test]
fn readiness_rover_produces_scorecard() {
    let path = repo_path(&["examples", "showcase", "readiness", "rover.sd"]);
    let source = std::fs::read_to_string(&path).unwrap();
    let program = parse(tokenize(&source).unwrap()).unwrap();
    let report = evaluate_scorecard(&program, &source, "rover.sd", &ScorecardOptions::default());
    assert_eq!(report.categories.len(), 7);
    assert!(report.overall_score > 0);
    assert!(!report.tier.is_empty());
}

#[test]
fn defense_showcase_scorecard_blends_secure_boot() {
    let registry = repo_path(&["registry"]);
    std::env::set_var(
        "SPANDA_REGISTRY_URL",
        format!("file://{}", registry.display()),
    );
    let path = repo_path(&[
        "examples",
        "showcase",
        "compliance",
        "defense_rover.sd",
    ]);
    let source = std::fs::read_to_string(&path).unwrap();
    let program = parse(tokenize(&source).unwrap()).unwrap();
    let report = evaluate_scorecard(
        &program,
        &source,
        "compliance/defense_rover.sd",
        &ScorecardOptions::default(),
    );
    let security = report
        .categories
        .iter()
        .find(|entry| entry.name == "security")
        .expect("security category");
    assert!(
        security.detail.contains("secure boot"),
        "expected secure boot in security detail, got {}",
        security.detail
    );
    std::env::remove_var("SPANDA_REGISTRY_URL");
}
