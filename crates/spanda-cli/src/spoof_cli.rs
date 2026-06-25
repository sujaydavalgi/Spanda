//! CLI for GPS and sensor spoofing checks.
//!
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_spoofing::{
    analyze_path, format_spoofing_report, generate_program_spoof_check, SpoofingFormat,
};
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
    eprintln!("Usage: spanda spoof-check <file.sd|file.trace> [--json]");
    process::exit(1);
}

/// `spanda spoof-check <file.sd|file.trace> [--json]`
pub fn spoof_check_dispatch(args: &[String]) {
    let file = file_arg(args);
    let path = Path::new(&file);
    let format = if args.iter().any(|a| a == "--json") {
        SpoofingFormat::Json
    } else {
        SpoofingFormat::Text
    };

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let report = if extension == "trace" {
        analyze_path(path).unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(1);
        })
    } else {
        let program = parse_program(path);
        generate_program_spoof_check(&program, &file)
    };

    println!("{}", format_spoofing_report(&report, format));
    if !report.passed {
        process::exit(1);
    }
}
