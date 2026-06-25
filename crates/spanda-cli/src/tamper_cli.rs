//! CLI for verify-time tamper analysis.
//!
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::{format_tamper_report, generate_tamper_check, TamperFormat};
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

fn file_arg(args: &[String]) -> String {
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => index += 1,
            other if !other.starts_with('-') => return other.to_string(),
            _ => index += 1,
        }
    }
    eprintln!("Usage: spanda tamper-check <file.sd> [--json]");
    process::exit(1);
}

/// `spanda tamper-check <file.sd> [--json]`
pub fn tamper_check_dispatch(args: &[String]) {
    let file = file_arg(args);
    let path = Path::new(&file);
    let program = parse_program(path);
    let report = generate_tamper_check(&program, &file);
    let format = if args.iter().any(|a| a == "--json") {
        TamperFormat::Json
    } else {
        TamperFormat::Text
    };
    println!("{}", format_tamper_report(&report, format));
    if !report.passed {
        process::exit(1);
    }
}
