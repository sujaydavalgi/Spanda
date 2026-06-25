//! Integration tests for ADR generation.

use spanda_adr::{generate_adrs, AdrFormat, format_adr_report};
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
fn warehouse_program_generates_deploy_and_policy_adrs() {
    let program = parse_file(repo_path(&[
        "examples",
        "showcase",
        "policy",
        "warehouse.sd",
    ]));
    let report = generate_adrs(&program, "warehouse.sd");
    assert!(report.records.len() >= 3);
    assert!(report.records.iter().any(|record| record.title.contains("Deploy")));
    assert!(report
        .records
        .iter()
        .any(|record| record.title.contains("WarehousePolicy")));
}

#[test]
fn markdown_format_includes_sections() {
    let program = parse_file(repo_path(&[
        "examples",
        "showcase",
        "policy",
        "warehouse.sd",
    ]));
    let report = generate_adrs(&program, "warehouse.sd");
    let markdown = format_adr_report(&report, AdrFormat::Markdown);
    assert!(markdown.contains("### Context"));
    assert!(markdown.contains("ADR-001"));
}
