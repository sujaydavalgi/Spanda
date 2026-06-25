//! CLI commands for explainability reports.

use crate::config_load::{ensure_config_valid, load_system_config};
use spanda_config::ConfigResolver;
use spanda_explain::{
    explain_decision_trace, explain_program_with_options, explain_readiness, explain_safety,
    explain_trace, explain_verify, format_explain_report, ExplainProgramOptions,
};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use std::fs;
use std::path::Path;
use std::process;
use std::sync::Arc;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {path}: {e}");
        process::exit(1);
    })
}

fn parse_program(source: &str) -> spanda_ast::nodes::Program {
    let tokens = tokenize(source).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    parse(tokens).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    })
}

fn file_arg(args: &[String]) -> String {
    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Missing file path");
            process::exit(1);
        })
}

fn json_flag(args: &[String]) -> bool {
    args.iter().any(|a| a == "--json")
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].clone())
}

fn resolve_baseline(path: &str) -> Arc<spanda_config::ResolvedSystemConfig> {
    let p = Path::new(path);
    let root = if p.is_dir() {
        p.to_path_buf()
    } else {
        p.parent().unwrap_or(p).to_path_buf()
    };
    Arc::new(
        ConfigResolver::new()
            .resolve_from_dir(&root)
            .unwrap_or_else(|e| {
                eprintln!("{e}");
                process::exit(1);
            }),
    )
}

/// `spanda explain <file.sd> [--json] [--config <spanda.toml>] [--baseline <dir|spanda.toml>]`
pub fn cmd_explain_program(args: &[String]) {
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let system_config = load_system_config(
        Path::new(&file),
        flag_value(args, "--config").as_deref().map(Path::new),
    );
    ensure_config_valid(system_config.as_ref().map(|arc| arc.as_ref()));
    let baseline_config = flag_value(args, "--baseline").map(|path| resolve_baseline(&path));
    let explain_options = ExplainProgramOptions {
        system_config: system_config.as_deref(),
        baseline_config: baseline_config.as_deref(),
    };
    let report = explain_program_with_options(&program, &file, &explain_options);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain readiness --file <file.sd> [--json]`
pub fn cmd_explain_readiness(args: &[String]) {
    let file = args
        .windows(2)
        .find(|w| w[0] == "--file")
        .map(|w| w[1].clone())
        .or_else(|| args.iter().find(|a| !a.starts_with('-')).cloned())
        .unwrap_or_else(|| {
            eprintln!("Missing --file <path>");
            process::exit(1);
        });
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = explain_readiness(&program, &file);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain verify --file <file.sd> [--json]`
pub fn cmd_explain_verify(args: &[String]) {
    let file = args
        .windows(2)
        .find(|w| w[0] == "--file")
        .map(|w| w[1].clone())
        .or_else(|| args.iter().find(|a| !a.starts_with('-')).cloned())
        .unwrap_or_else(|| {
            eprintln!("Missing --file <path>");
            process::exit(1);
        });
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = explain_verify(&program, &file);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain safety --file <file.sd> [--json]`
pub fn cmd_explain_safety(args: &[String]) {
    let file = args
        .windows(2)
        .find(|w| w[0] == "--file")
        .map(|w| w[1].clone())
        .or_else(|| args.iter().find(|a| !a.starts_with('-')).cloned())
        .unwrap_or_else(|| {
            eprintln!("Missing --file <path>");
            process::exit(1);
        });
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = explain_safety(&program, &file);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain decision <mission.trace> [--json]`
pub fn cmd_explain_decision(args: &[String]) {
    let file = args
        .iter()
        .find(|arg| !arg.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Usage: spanda explain decision <mission.trace> [--json]");
            process::exit(1);
        });
    match explain_decision_trace(&file) {
        Ok(report) => println!("{}", format_explain_report(&report, json_flag(args))),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

/// `spanda explain <trace> [--json]` when path ends with `.trace`
pub fn cmd_explain_trace(args: &[String]) {
    let file = file_arg(args);
    match explain_trace(&file) {
        Ok(report) => println!("{}", format_explain_report(&report, json_flag(args))),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

/// Dispatch `spanda explain` subcommands.
pub fn explain_dispatch(args: &[String]) {
    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "readiness" => cmd_explain_readiness(&args[1..]),
        "verify" => cmd_explain_verify(&args[1..]),
        "safety" => cmd_explain_safety(&args[1..]),
        "decision" => cmd_explain_decision(&args[1..]),
        "" => {
            eprintln!(
                "Usage:\n  spanda explain <file.sd> [--json] [--config <spanda.toml>] [--baseline <dir|spanda.toml>]\n  spanda explain readiness|verify|safety --file <file.sd> [--json]\n  spanda explain decision <mission.trace> [--json]\n  spanda explain <mission.trace> [--json]"
            );
            process::exit(1);
        }
        other if other.ends_with(".trace") || other.ends_with(".json") => {
            cmd_explain_trace(args);
        }
        other if other.ends_with(".sd") => cmd_explain_program(args),
        _ => {
            eprintln!(
                "Usage:\n  spanda explain <file.sd> [--json] [--config <spanda.toml>] [--baseline <dir|spanda.toml>]\n  spanda explain readiness|verify|safety --file <file.sd> [--json]\n  spanda explain decision <mission.trace> [--json]\n  spanda explain <mission.trace> [--json]"
            );
            process::exit(1);
        }
    }
}
