//! Mission trace replay helpers shared by CLI commands.

use crate::config_load::{apply_system_config_to_run_options, load_system_config};
use spanda_driver::{playback_mission, replay_mission, RunOptions};
use spanda_runtime::replay::{parse_replay_offset, MissionTrace};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Arc;

/// Replay or inspect a mission trace with optional deterministic verification.
pub fn human_replay(
    trace_file: &str,
    from: Option<&str>,
    deterministic: bool,
    playback: bool,
    show_faults: bool,
    as_json: bool,
    config_flag: Option<&Path>,
) {
    let trace = MissionTrace::load(trace_file).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    if show_faults {
        if as_json {
            let faults = spanda_runtime_faults::faults_from_trace(&trace);
            println!(
                "{}",
                serde_json::to_string_pretty(&faults).unwrap_or_default()
            );
        } else {
            println!("{}", spanda_runtime_faults::format_trace_faults(&trace));
        }
        return;
    }
    let offset_ms = if let Some(raw) = from {
        parse_replay_offset(raw).unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(1);
        })
    } else {
        0.0
    };
    let frames = trace.frames_from(offset_ms);

    if playback {
        let source_path = resolve_trace_source(trace_file, &trace.source);
        let run_opts = replay_run_options(&source_path, config_flag, offset_ms, true, false);
        let (report, state) = playback_mission(trace_file, run_opts).unwrap_or_else(|error| {
            eprintln!("Playback failed: {error}");
            process::exit(1);
        });
        if as_json {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "ok": true,
                    "mode": "playback",
                    "frames_applied": report.frames_applied,
                    "states_applied": report.states_applied,
                    "offset_ms": offset_ms,
                    "state": state,
                }))
                .unwrap()
            );
            return;
        }
        println!(
            "Playback {}: {} frames ({} with state) from {:.0}ms",
            trace_file, report.frames_applied, report.states_applied, offset_ms
        );
        println!(
            "  Final pose: x={:.3} y={:.3} θ={:.3}",
            state.pose.x, state.pose.y, state.pose.theta
        );
        return;
    }

    if deterministic {
        let source_path = resolve_trace_source(trace_file, &trace.source);
        let source = fs::read_to_string(&source_path).unwrap_or_else(|error| {
            eprintln!("Failed to read trace source '{source_path}': {error}");
            process::exit(1);
        });
        let run_opts = replay_run_options(&source_path, config_flag, offset_ms, false, true);
        let (_, verification) =
            replay_mission(&source, trace_file, run_opts).unwrap_or_else(|error| {
                eprintln!("Replay failed: {error}");
                process::exit(1);
            });
        if as_json {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "ok": verification.ok,
                    "source": trace.source,
                    "deterministic": true,
                    "offset_ms": offset_ms,
                    "matched": verification.matched,
                    "mismatches": verification.mismatches,
                }))
                .unwrap()
            );
        } else if verification.ok {
            println!(
                "✓ Deterministic replay verified for {} ({} frames from {:.0}ms)",
                trace_file, verification.matched, offset_ms
            );
        } else {
            eprintln!("✗ Deterministic replay mismatch for {trace_file}:");
            for mismatch in &verification.mismatches {
                eprintln!("  {mismatch}");
            }
            process::exit(1);
        }
        return;
    }

    if as_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "ok": true,
                "source": trace.source,
                "deterministic": trace.deterministic,
                "offset_ms": offset_ms,
                "frames": frames,
            }))
            .unwrap()
        );
        return;
    }
    println!(
        "Replay {} ({} frames from {:.0}ms)",
        trace_file,
        frames.len(),
        offset_ms
    );
    for frame in frames.iter().take(20) {
        println!(
            "  t={:.1}ms {} {:?}",
            frame.sim_time_ms, frame.event, frame.payload
        );
    }
    if frames.len() > 20 {
        println!("  ... {} more frames", frames.len() - 20);
    }
}

fn replay_run_options(
    source_path: &str,
    config_flag: Option<&Path>,
    offset_ms: f64,
    playback_wall_clock: bool,
    replay_deterministic: bool,
) -> RunOptions {
    let path = PathBuf::from(source_path);
    let cfg: Option<Arc<spanda_config::ResolvedSystemConfig>> =
        load_system_config(&path, config_flag);
    let base = RunOptions {
        max_loop_iterations: 20,
        record_trace: replay_deterministic,
        trace_source: Some(source_path.to_string()),
        replay_from_ms: Some(offset_ms),
        replay_deterministic,
        playback_wall_clock,
        ..Default::default()
    };
    apply_system_config_to_run_options(cfg, base, &path)
}

fn resolve_trace_source(trace_file: &str, source: &str) -> String {
    if Path::new(source).is_file() {
        return source.to_string();
    }
    if let Some(parent) = Path::new(trace_file).parent() {
        let candidate = parent.join(source);
        if candidate.is_file() {
            return candidate.to_string_lossy().into_owned();
        }
    }
    source.to_string()
}
