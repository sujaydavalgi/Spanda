//! CLI commands for mission assurance and autonomous operations.

use crate::config_load::{ensure_config_valid, load_system_config_from_cli_args};

use spanda_assurance::{
    assure_program_with_config, check_resilience, diagnose_from_trace,
    diagnose_program_with_config, evaluate_prognostics, evaluate_recovery_coverage_with_config,
    evaluate_state_assurance, format_anomaly, format_assurance, format_diagnosis,
    format_mission_assurance, format_prognostics, format_recovery_coverage, format_resilience,
    format_state, scan_anomalies, verify_mission_assurance_with_config,
};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_readiness::ReportFormat;
use std::fs;
use std::path::Path;
use std::process;

fn read_file(path: &str) -> String {
    // Description:
    //     Read file.
    //
    // Inputs:
    //     path: &str
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: String
    //         Return value from `read_file`.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::read_file(path);

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {path}: {e}");
        process::exit(1);
    })
}

fn parse_program(source: &str) -> spanda_ast::nodes::Program {
    // Description:
    //     Parse program.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: spanda_ast::nodes::Program
    //         Return value from `parse_program`.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::parse_program(source);

    let tokens = tokenize(source).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    parse(tokens).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    })
}

fn parse_format(args: &[String]) -> ReportFormat {
    // Description:
    //     Parse format.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: ReportFormat
    //         Return value from `parse_format`.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::parse_format(args);

    if args.iter().any(|a| a == "--json") {
        ReportFormat::Json
    } else if args.iter().any(|a| a == "--markdown") {
        ReportFormat::Markdown
    } else if args.iter().any(|a| a == "--html") {
        ReportFormat::Html
    } else {
        ReportFormat::Text
    }
}

fn file_arg(args: &[String]) -> String {
    // Description:
    //     File arg.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: String
    //         Return value from `file_arg`.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::file_arg(args);

    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Missing file path");
            process::exit(1);
        })
}

fn assurance_config(
    args: &[String],
) -> Option<std::sync::Arc<spanda_config::ResolvedSystemConfig>> {
    load_system_config_from_cli_args(args)
}

/// `spanda assure <file.sd> [--json|--markdown|--html]`
pub fn cmd_assure(args: &[String]) {
    let cfg = assurance_config(args);
    if let Some(ref c) = cfg {
        ensure_config_valid(Some(c.as_ref()));
    }
    // Description:
    //     Cmd assure.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_assure(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let summary = assure_program_with_config(&program, &file, cfg.as_ref().map(|a| a.as_ref()));
    let output = match format {
        ReportFormat::Json => serde_json::to_string_pretty(&summary).unwrap_or_default(),
        _ => format_assurance(&summary.assurance, format),
    };
    println!("{output}");
    if !summary.passed {
        process::exit(1);
    }
}

/// `spanda anomaly scan <file.sd> [--json|--markdown|--html]`
pub fn cmd_anomaly_scan(args: &[String]) {
    // Description:
    //     Cmd anomaly scan.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_anomaly_scan(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = scan_anomalies(&program);
    println!("{}", format_anomaly(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda diagnose <mission.trace|file.sd> [--json|--markdown|--html]`
pub fn cmd_diagnose_assurance(args: &[String]) {
    let cfg = assurance_config(args);
    if let Some(ref c) = cfg {
        ensure_config_valid(Some(c.as_ref()));
    }
    // Description:
    //     Cmd diagnose assurance.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_diagnose_assurance(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let report = if file.ends_with(".trace") {
        diagnose_from_trace(Path::new(&file)).unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        })
    } else {
        let source = read_file(&file);
        let program = parse_program(&source);
        diagnose_program_with_config(&program, cfg.as_ref().map(|a| a.as_ref()))
    };
    println!("{}", format_diagnosis(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda prognostics <file.sd> [--json|--markdown|--html]`
pub fn cmd_prognostics(args: &[String]) {
    // Description:
    //     Cmd prognostics.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_prognostics(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = evaluate_prognostics(&program);
    println!("{}", format_prognostics(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda mission verify <file.sd> [--json|--markdown|--html]`
pub fn cmd_mission_verify(args: &[String]) {
    let cfg = assurance_config(args);
    if let Some(ref c) = cfg {
        ensure_config_valid(Some(c.as_ref()));
    }
    // Description:
    //     Cmd mission verify.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_mission_verify(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = verify_mission_assurance_with_config(&program, cfg.as_ref().map(|a| a.as_ref()));
    println!("{}", format_mission_assurance(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda resilience check <file.sd> [--json|--markdown|--html]`
pub fn cmd_resilience_check(args: &[String]) {
    // Description:
    //     Cmd resilience check.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_resilience_check(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = check_resilience(&program);
    println!("{}", format_resilience(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// Dispatch `spanda anomaly` subcommands.
pub fn anomaly_dispatch(args: &[String]) {
    // Description:
    //     Anomaly dispatch.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::anomaly_dispatch(args);

    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "scan" => cmd_anomaly_scan(&args[1..]),
        _ => {
            eprintln!("Usage: spanda anomaly scan <file.sd>");
            process::exit(1);
        }
    }
}

/// Dispatch `spanda mission` subcommands.
pub fn mission_dispatch(args: &[String]) {
    // Description:
    //     Mission dispatch.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::mission_dispatch(args);

    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "verify" => cmd_mission_verify(&args[1..]),
        _ => {
            eprintln!("Usage: spanda mission verify <file.sd>");
            process::exit(1);
        }
    }
}

/// Dispatch `spanda resilience` subcommands.
pub fn resilience_dispatch(args: &[String]) {
    // Description:
    //     Resilience dispatch.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::resilience_dispatch(args);

    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "check" => cmd_resilience_check(&args[1..]),
        _ => {
            eprintln!("Usage: spanda resilience check <file.sd>");
            process::exit(1);
        }
    }
}

/// `spanda mitigation plan <file.sd> [--json|--markdown|--html]`
pub fn cmd_mitigation_plan(args: &[String]) {
    // Description:
    //     Cmd mitigation plan.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_mitigation_plan(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = spanda_assurance::mitigation_report(&program);
    println!("{}", spanda_assurance::format_mitigation(&report, format));
}

/// `spanda state estimate <file.sd> [--json|--markdown|--html]`
pub fn cmd_state_estimate(args: &[String]) {
    // Description:
    //     Cmd state estimate.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::cmd_state_estimate(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = evaluate_state_assurance(&program);
    println!("{}", format_state(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// Dispatch `spanda state` subcommands.
pub fn state_dispatch(args: &[String]) {
    // Description:
    //     State dispatch.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::state_dispatch(args);

    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "estimate" => cmd_state_estimate(&args[1..]),
        _ => {
            eprintln!("Usage: spanda state estimate <file.sd>");
            process::exit(1);
        }
    }
}

/// Dispatch `spanda mitigation` subcommands.
pub fn mitigation_dispatch(args: &[String]) {
    // Description:
    //     Mitigation dispatch.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::assurance_cli::mitigation_dispatch(args);

    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "plan" => cmd_mitigation_plan(&args[1..]),
        _ => {
            eprintln!("Usage: spanda mitigation plan <file.sd>");
            process::exit(1);
        }
    }
}

/// `spanda recovery-coverage <file.sd> [--json] [--format markdown]`
pub fn cmd_recovery_coverage(args: &[String]) {
    let cfg = assurance_config(args);
    if let Some(ref c) = cfg {
        ensure_config_valid(Some(c.as_ref()));
    }
    let json = args.iter().any(|a| a == "--json");
    let markdown = args.iter().any(|a| a == "--markdown")
        || args
            .windows(2)
            .any(|w| w[0] == "--format" && w[1] == "markdown");
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report =
        evaluate_recovery_coverage_with_config(&program, &file, cfg.as_ref().map(|a| a.as_ref()));
    println!("{}", format_recovery_coverage(&report, json, markdown));
}
