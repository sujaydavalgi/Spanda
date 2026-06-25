//! CLI for verify-time integrity verification.
//!
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::{format_integrity_report, generate_integrity_report, IntegrityFormat};
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
            "--baseline" | "--agent" => index += 2,
            "--json" => index += 1,
            other if !other.starts_with('-') => return other.to_string(),
            _ => index += 1,
        }
    }
    eprintln!("Usage: spanda integrity <file.sd> [--baseline <file.sd>] [--json]");
    process::exit(1);
}

fn baseline_arg(args: &[String]) -> Option<String> {
    args.windows(2)
        .find(|window| window[0] == "--baseline")
        .map(|window| window[1].clone())
}

/// `spanda integrity <file.sd> [--baseline <file.sd>] [--json]`
pub fn integrity_dispatch(args: &[String]) {
    let file = file_arg(args);
    let path = Path::new(&file);
    let program = parse_program(path);
    let baseline_path = baseline_arg(args);
    if args.iter().any(|arg| arg == "--agent") {
        eprintln!("Note: --agent comparison is not yet implemented; using static baseline only.");
    }
    let (baseline_program, baseline_label) = if let Some(baseline_file) = baseline_path {
        let baseline = parse_program(Path::new(&baseline_file));
        (Some(baseline), Some(baseline_file))
    } else {
        (None, None)
    };
    let report = generate_integrity_report(
        &program,
        &file,
        baseline_program.as_ref(),
        baseline_label.as_deref(),
    );
    let format = if args.iter().any(|a| a == "--json") {
        IntegrityFormat::Json
    } else {
        IntegrityFormat::Text
    };
    println!("{}", format_integrity_report(&report, format));
    if !report.passed {
        process::exit(1);
    }
}
