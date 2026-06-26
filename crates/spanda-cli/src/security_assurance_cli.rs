//! CLI for security assurance rollup reports.
//!
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::{
    format_security_assurance_report, generate_security_assurance, SecurityAssuranceFormat,
};
use std::fs;
use std::path::Path;
use std::process;

fn parse_format(args: &[String]) -> SecurityAssuranceFormat {
    if args.iter().any(|arg| arg == "--json") {
        return SecurityAssuranceFormat::Json;
    }
    for (index, arg) in args.iter().enumerate() {
        if arg == "--format" {
            if let Some(value) = args.get(index + 1) {
                return match value.as_str() {
                    "markdown" | "md" => SecurityAssuranceFormat::Markdown,
                    "json" => SecurityAssuranceFormat::Json,
                    _ => SecurityAssuranceFormat::Text,
                };
            }
        }
    }
    SecurityAssuranceFormat::Text
}

fn file_arg(args: &[String]) -> String {
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--format" => index += 2,
            "--json" => index += 1,
            other if !other.starts_with('-') => return other.to_string(),
            _ => index += 1,
        }
    }
    eprintln!("Usage: spanda security assurance <file.sd> [--json] [--format markdown]");
    process::exit(1);
}

/// `spanda security assurance <file.sd> [--json] [--format markdown]`
pub fn assurance_dispatch(args: &[String]) {
    let file = file_arg(args);
    let path = Path::new(&file);
    let source = fs::read_to_string(path).unwrap_or_else(|error| {
        eprintln!("Failed to read {}: {error}", path.display());
        process::exit(1);
    });
    let tokens = tokenize(&source).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    let program = parse(tokens).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    let report = generate_security_assurance(&program, &source, &file);
    let format = parse_format(args);
    println!("{}", format_security_assurance_report(&report, format));
    if !report.passed {
        process::exit(1);
    }
}
