//! OTA deploy CLI handlers (`spanda deploy plan|rollout|rollback|status`).

use spanda_core::{
    apply_rollout, build_deploy_plan, compile, default_state_path, load_deploy_state,
    orchestrate_fleets, plan_rollout, rollback_targets, save_deploy_state, DeployState,
    RolloutOptions, RolloutStrategy,
};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn read_source(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error reading {path}: {e}");
        process::exit(1);
    })
}

fn parse_program(source: &str, file: &str) -> spanda_core::Program {
    compile(source).unwrap_or_else(|e| {
        eprintln!("Error compiling {file}: {e}");
        process::exit(1);
    }).program
}

fn state_path() -> std::path::PathBuf {
    env::var("SPANDA_DEPLOY_STATE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| default_state_path())
}

pub fn deploy_dispatch(args: &[String]) {
    if args.is_empty() {
        usage();
        process::exit(1);
    }
    match args[0].as_str() {
        "plan" => cmd_plan(&args[1..]),
        "rollout" => cmd_rollout(&args[1..]),
        "rollback" => cmd_rollback(&args[1..]),
        "status" => cmd_status(&args[1..]),
        other if !other.starts_with('-') => {
            eprintln!("Unknown deploy subcommand '{other}'");
            usage();
            process::exit(1);
        }
        _ => {
            usage();
            process::exit(1);
        }
    }
}

pub fn deploy_usage_lines() -> &'static str {
    "           spanda deploy plan [--json] [--version <ver>] <file.sd>\n\
     spanda deploy rollout [--json] [--strategy all|canary|staged] [--canary-percent N] [--version <ver>] [--dry-run] <file.sd>\n\
     spanda deploy rollback [--json] <file.sd>\n\
     spanda deploy status [--json]\n\
     spanda deploy --target wasm [--out <file.json>] <file.sd>"
}

fn usage() {
    eprintln!("Usage:\n{}", deploy_usage_lines());
}

fn cmd_plan(args: &[String]) {
    let mut json = false;
    let mut version = "1.0.0".to_string();
    let mut file: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => json = true,
            "--version" if i + 1 < args.len() => {
                version = args[i + 1].clone();
                i += 1;
            }
            other if !other.starts_with('-') && file.is_none() => file = Some(other.to_string()),
            other => {
                eprintln!("Unknown argument: {other}");
                process::exit(1);
            }
        }
        i += 1;
    }
    let file = file.unwrap_or_else(|| {
        eprintln!("Missing file path");
        process::exit(1);
    });
    let source = read_source(&file);
    let program = parse_program(&source, &file);
    let plan = build_deploy_plan(&program, &file, &version);
    if json {
        println!("{}", serde_json::to_string_pretty(&plan).unwrap());
    } else {
        println!("Deploy plan for {file} (version {version})");
        for a in &plan.assignments {
            println!("  {} -> {}", a.robot_name, a.hardware);
        }
        if !plan.certifications.is_empty() {
            println!("  certifications: {}", plan.certifications.join(", "));
        }
    }
}

fn cmd_rollout(args: &[String]) {
    let mut json = false;
    let mut dry_run = false;
    let mut version = "1.0.0".to_string();
    let mut strategy = RolloutStrategy::All;
    let mut canary_percent = 10u8;
    let mut file: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => json = true,
            "--dry-run" => dry_run = true,
            "--version" if i + 1 < args.len() => {
                version = args[i + 1].clone();
                i += 1;
            }
            "--strategy" if i + 1 < args.len() => {
                strategy = match args[i + 1].as_str() {
                    "all" => RolloutStrategy::All,
                    "canary" => RolloutStrategy::Canary,
                    "staged" => RolloutStrategy::Staged,
                    other => {
                        eprintln!("Unknown strategy '{other}'");
                        process::exit(1);
                    }
                };
                i += 1;
            }
            "--canary-percent" if i + 1 < args.len() => {
                canary_percent = args[i + 1].parse().unwrap_or(10);
                i += 1;
            }
            other if !other.starts_with('-') && file.is_none() => file = Some(other.to_string()),
            other => {
                eprintln!("Unknown argument: {other}");
                process::exit(1);
            }
        }
        i += 1;
    }
    let file = file.unwrap_or_else(|| {
        eprintln!("Missing file path");
        process::exit(1);
    });
    let source = read_source(&file);
    let program = parse_program(&source, &file);
    let plan = build_deploy_plan(&program, &file, &version);
    let options = RolloutOptions {
        strategy,
        canary_percent,
        version: version.clone(),
        dry_run,
        ..Default::default()
    };
    let result = plan_rollout(&plan, &options);
    if !dry_run {
        let path = state_path();
        let mut state = load_deploy_state(&path);
        apply_rollout(&mut state, &result);
        if let Err(e) = save_deploy_state(&path, &state) {
            eprintln!("Warning: could not save deploy state: {e}");
        }
    }
    print_rollout(&result, json);
}

fn cmd_rollback(args: &[String]) {
    let mut json = false;
    let mut file: Option<String> = None;
    for arg in args {
        match arg.as_str() {
            "--json" => json = true,
            other if !other.starts_with('-') && file.is_none() => file = Some(other.to_string()),
            other => {
                eprintln!("Unknown argument: {other}");
                process::exit(1);
            }
        }
    }
    let file = file.unwrap_or_else(|| {
        eprintln!("Missing file path");
        process::exit(1);
    });
    let source = read_source(&file);
    let program = parse_program(&source, &file);
    let plan = build_deploy_plan(&program, &file, "rollback");
    let path = state_path();
    let mut state = load_deploy_state(&path);
    let result = rollback_targets(&mut state, &plan, true);
    if let Err(e) = save_deploy_state(&path, &state) {
        eprintln!("Warning: could not save deploy state: {e}");
    }
    print_rollout(&result, json);
}

fn cmd_status(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let path = state_path();
    let state: DeployState = load_deploy_state(&path);
    if json {
        println!("{}", serde_json::to_string_pretty(&state).unwrap());
    } else {
        println!("Deploy state ({})", path.display());
        for (key, ver) in &state.current_version {
            let prev = state
                .previous_version
                .get(key)
                .map(|s| s.as_str())
                .unwrap_or("-");
            println!("  {key}: {ver} (previous: {prev})");
        }
        if state.current_version.is_empty() {
            println!("  (no deployments recorded)");
        }
    }
}

fn print_rollout(result: &spanda_core::RolloutResult, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(result).unwrap());
    } else {
        println!(
            "Rollout {} ({:?}) — {}",
            result.version,
            result.strategy,
            if result.success { "ok" } else { "failed" }
        );
        for step in &result.steps {
            println!(
                "  {}@{} -> {:?} v{}",
                step.robot_name, step.hardware, step.status, step.version
            );
        }
    }
    let _ = io::stdout().flush();
}

pub fn fleet_orchestrate_dispatch(args: &[String]) {
    let mut json = false;
    let mut file: Option<String> = None;
    for arg in args {
        match arg.as_str() {
            "--json" => json = true,
            other if !other.starts_with('-') && file.is_none() => file = Some(other.to_string()),
            other => {
                eprintln!("Unknown argument: {other}");
                process::exit(1);
            }
        }
    }
    let file = file.unwrap_or_else(|| {
        eprintln!("Missing file path");
        process::exit(1);
    });
    let source = read_source(&file);
    let program = parse_program(&source, &file);
    let result = orchestrate_fleets(&program, &file);
    if json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("Fleet orchestration for {file}");
        for fleet in &result.fleets {
            println!("  fleet {} ({})", fleet.fleet_name, fleet.coordination_mode);
            for member in &fleet.members {
                println!(
                    "    {} mission={:?} state={} step='{}' peer={}",
                    member.robot_name,
                    member.mission_name,
                    member.mission_state,
                    member.current_step,
                    member.has_peer_link
                );
            }
        }
    }
}
