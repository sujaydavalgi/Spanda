//! CLI for mission differencing between Spanda programs.
//!
use spanda_diff::{diff_programs_with_capabilities, format_mission_diff};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use std::fs;
use std::path::Path;
use std::process;

fn parse_program(path: &Path) -> spanda_ast::nodes::Program {
    let source = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {e}", path.display());
        process::exit(1);
    });
    let tokens = tokenize(&source).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    parse(tokens).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    })
}

fn file_args(args: &[String]) -> (String, String) {
    let files: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with('-'))
        .cloned()
        .collect();
    if files.len() < 2 {
        eprintln!("Usage: spanda diff <baseline.sd> <candidate.sd> [--json]");
        process::exit(1);
    }
    (files[0].clone(), files[1].clone())
}

/// `spanda diff <baseline.sd> <candidate.sd> [--json]`
pub fn diff_dispatch(args: &[String]) {
    let (baseline_file, candidate_file) = file_args(args);
    let baseline = parse_program(Path::new(&baseline_file));
    let candidate = parse_program(Path::new(&candidate_file));
    let json = args.iter().any(|a| a == "--json");
    let report =
        diff_programs_with_capabilities(&baseline, &candidate, &baseline_file, &candidate_file);
    println!("{}", format_mission_diff(&report, json));
    if report.has_deploy_impact || report.has_safety_impact {
        process::exit(2);
    }
    if !report.changes.is_empty() {
        process::exit(1);
    }
}
