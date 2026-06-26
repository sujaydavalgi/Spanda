//! Chaos experiment orchestration composing recovery, readiness, health, and safety engines.

use serde::{Deserialize, Serialize};
use spanda_assurance::{simulate_failure_recovery, RecoveryStatus};
use spanda_ast::nodes::{Program, RobotDecl, SensorDecl};
use spanda_capability::{evaluate_runtime_health, HealthStatus};
use spanda_readiness::{evaluate_readiness, ReadinessOptions, ReadinessSeverity};

/// Output format for chaos reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChaosFormat {
    #[default]
    Text,
    Json,
}

/// Options for chaos experiments.
#[derive(Debug, Clone, Default)]
pub struct ChaosExperimentOptions {
    pub injections: Vec<String>,
}

/// Per-injection verification outcome.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChaosInjectionResult {
    pub injection: String,
    pub recovery_passed: bool,
    pub health_passed: bool,
    pub readiness_passed: bool,
    pub safety_passed: bool,
    pub passed: bool,
    pub details: Vec<String>,
}

/// Chaos experiment rollup for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChaosReport {
    pub program: String,
    pub injections: Vec<ChaosInjectionResult>,
    pub passed: bool,
}

/// Normalize user-facing injection labels to internal failure kinds.
pub fn normalize_injection(raw: &str) -> Option<&'static str> {
    // Map CLI aliases to canonical failure tokens used by recovery simulation.
    //
    // Parameters:
    // - `raw` — user-supplied injection label
    //
    // Returns:
    // Canonical failure kind, or `None` when unrecognized.
    //
    // Options:
    // None.
    //
    // Example:
    // let kind = normalize_injection("gps-failure");

    let normalized = raw.trim().to_ascii_lowercase().replace('_', "-");
    match normalized.as_str() {
        "gps" | "gps-failure" => Some("gps"),
        "camera" | "camera-failure" => Some("camera"),
        "lidar" | "lidar-failure" => Some("lidar"),
        "connectivity" | "connectivity-failure" | "network" | "network-failure" => {
            Some("connectivity")
        }
        "provider" | "provider-failure" => Some("provider"),
        "package" | "package-failure" => Some("package"),
        "battery" | "battery-failure" => Some("battery"),
        _ => None,
    }
}

/// Infer default chaos injections from program structure.
pub fn default_injections(program: &Program) -> Vec<String> {
    // Select sensor and connectivity failures relevant to the parsed program.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    //
    // Returns:
    // Injection labels in recommended run order.
    //
    // Options:
    // None.
    //
    // Example:
    // let kinds = default_injections(&program);

    let mut kinds = Vec::new();
    if program_has_sensor(program, "gps") || program_mentions(program, "gps") {
        kinds.push("gps-failure".into());
    }
    if program_has_sensor(program, "camera") || program_mentions(program, "camera") {
        kinds.push("camera-failure".into());
    }
    if program_has_sensor(program, "lidar") || program_mentions(program, "lidar") {
        kinds.push("lidar-failure".into());
    }
    if program_has_connectivity(program) || program_mentions(program, "connect") {
        kinds.push("connectivity-failure".into());
    }
    if program_mentions(program, "provider") {
        kinds.push("provider-failure".into());
    }
    if program_mentions(program, "package") {
        kinds.push("package-failure".into());
    }
    if program_mentions(program, "battery") {
        kinds.push("battery-failure".into());
    }
    if kinds.is_empty() {
        kinds.push("gps-failure".into());
    }
    kinds
}

/// Run chaos injections and verify recovery, health, readiness, and safety signals.
pub fn run_chaos_experiment(
    program: &Program,
    source_label: &str,
    options: &ChaosExperimentOptions,
) -> ChaosReport {
    // Execute each injection and compose assurance, readiness, health, and safety checks.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    // - `source_label` — file label for reports
    // - `options` — explicit injection list (empty uses defaults)
    //
    // Returns:
    // Chaos experiment report with per-injection pass/fail.
    //
    // Options:
    // `ChaosExperimentOptions::injections`.
    //
    // Example:
    // let report = run_chaos_experiment(&program, "rover.sd", &ChaosExperimentOptions::default());

    let labels = if options.injections.is_empty() {
        default_injections(program)
    } else {
        options.injections.clone()
    };

    let mut results = Vec::new();
    for label in labels {
        if let Some(kind) = normalize_injection(&label) {
            results.push(evaluate_injection(program, source_label, &label, kind));
        } else {
            results.push(ChaosInjectionResult {
                injection: label.clone(),
                recovery_passed: false,
                health_passed: false,
                readiness_passed: false,
                safety_passed: false,
                passed: false,
                details: vec![format!("unknown injection kind: {label}")],
            });
        }
    }

    let passed = !results.is_empty() && results.iter().all(|result| result.passed);
    ChaosReport {
        program: source_label.into(),
        injections: results,
        passed,
    }
}

/// Format a chaos report for CLI output.
pub fn format_chaos_report(report: &ChaosReport, format: ChaosFormat) -> String {
    // Render chaos results as human text or JSON.
    //
    // Parameters:
    // - `report` — chaos experiment report
    // - `format` — text or JSON output
    //
    // Returns:
    // Formatted report string.
    //
    // Options:
    // `ChaosFormat::Json` for machine-readable output.
    //
    // Example:
    // println!("{}", format_chaos_report(&report, ChaosFormat::Text));

    match format {
        ChaosFormat::Json => serde_json::to_string_pretty(report).unwrap_or_else(|e| e.to_string()),
        ChaosFormat::Text => format_chaos_text(report),
    }
}

fn evaluate_injection(
    program: &Program,
    _source_label: &str,
    label: &str,
    kind: &str,
) -> ChaosInjectionResult {
    let recovery = simulate_failure_recovery(program, kind);
    let readiness = evaluate_readiness(
        program,
        &ReadinessOptions {
            simulate: true,
            include_runtime: true,
            inject_health_faults: true,
            ..ReadinessOptions::default()
        },
    );
    let health = evaluate_runtime_health(&runtime_faults(kind), &[], program);

    let recovery_passed = recovery.passed;
    let readiness_passed = readiness.mission_ready
        || recovery.readiness.recovery_ready
        || !readiness_has_errors(&readiness);
    let health_passed = !matches!(
        health.overall,
        HealthStatus::Critical | HealthStatus::Unsafe | HealthStatus::Failed
    );
    let safety_passed = !recovery
        .results
        .iter()
        .any(|result| result.status == RecoveryStatus::Unsafe)
        && recovery.safe_actions.iter().all(|action| action.approved);
    let passed = recovery_passed && readiness_passed && health_passed && safety_passed;

    let mut details = Vec::new();
    details.push(format!(
        "recovery: {} (plans={}, success_rate={:.0}%)",
        if recovery_passed { "pass" } else { "fail" },
        recovery.plans.len(),
        recovery.assurance.success_rate * 100.0
    ));
    details.push(format!(
        "readiness: {} (mission_ready={}, score {}/{})",
        if readiness_passed { "pass" } else { "fail" },
        readiness.mission_ready,
        readiness.score.total,
        readiness.score.maximum
    ));
    details.push(format!(
        "health: {} (overall={:?}, checks={})",
        if health_passed { "pass" } else { "fail" },
        health.overall,
        health.checks.len()
    ));
    details.push(format!(
        "safety: {} (unsafe_results={}, approved_actions={}/{})",
        if safety_passed { "pass" } else { "fail" },
        recovery
            .results
            .iter()
            .filter(|result| result.status == RecoveryStatus::Unsafe)
            .count(),
        recovery
            .safe_actions
            .iter()
            .filter(|action| action.approved)
            .count(),
        recovery.safe_actions.len()
    ));

    ChaosInjectionResult {
        injection: label.into(),
        recovery_passed,
        health_passed,
        readiness_passed,
        safety_passed,
        passed,
        details,
    }
}

fn format_chaos_text(report: &ChaosReport) -> String {
    let mut lines = vec![
        format!("Chaos experiment: {}", report.program),
        format!("Result: {}", if report.passed { "PASS" } else { "FAIL" }),
        String::new(),
    ];

    for result in &report.injections {
        lines.push(format!(
            "Injection: {} — {}",
            result.injection,
            if result.passed { "PASS" } else { "FAIL" }
        ));
        lines.push(format!(
            "  recovery={} health={} readiness={} safety={}",
            pass_label(result.recovery_passed),
            pass_label(result.health_passed),
            pass_label(result.readiness_passed),
            pass_label(result.safety_passed),
        ));
        for detail in &result.details {
            lines.push(format!("  - {detail}"));
        }
        lines.push(String::new());
    }

    lines.join("\n").trim_end().to_string()
}

fn pass_label(passed: bool) -> &'static str {
    if passed {
        "pass"
    } else {
        "fail"
    }
}

fn readiness_has_errors(readiness: &spanda_readiness::ReadinessReport) -> bool {
    readiness.issues.iter().any(|issue| {
        matches!(
            issue.severity,
            ReadinessSeverity::Critical | ReadinessSeverity::High
        )
    })
}

fn runtime_faults(kind: &str) -> Vec<String> {
    match kind {
        "gps" => vec!["gps.failed".into(), "gps".into()],
        "camera" => vec!["camera.failed".into(), "camera".into()],
        "lidar" => vec!["lidar.failed".into(), "lidar".into()],
        "connectivity" => vec!["network.disconnected".into(), "connectivity".into()],
        "provider" => vec!["provider.failed".into(), "provider".into()],
        "package" => vec!["package.failed".into(), "package".into()],
        "battery" => vec!["battery.low".into(), "battery".into()],
        other => vec![format!("{other}.failed")],
    }
}

fn program_has_sensor(program: &Program, needle: &str) -> bool {
    let Program::Program { robots, .. } = program;
    robots.iter().any(|robot| match robot {
        RobotDecl::RobotDecl { sensors, .. } => sensors.iter().any(|sensor| match sensor {
            SensorDecl::SensorDecl { sensor_type, .. } => {
                sensor_type.eq_ignore_ascii_case(needle)
                    || sensor_type.to_ascii_lowercase().contains(needle)
            }
        }),
    })
}

fn program_has_connectivity(program: &Program) -> bool {
    let Program::Program {
        robots,
        connectivity_policies,
        requires_connectivity,
        ..
    } = program;
    !connectivity_policies.is_empty()
        || requires_connectivity.is_some()
        || robots.iter().any(|robot| {
            matches!(
                robot,
                RobotDecl::RobotDecl {
                    requires_connectivity: Some(_),
                    ..
                }
            )
        })
}

fn program_mentions(program: &Program, needle: &str) -> bool {
    let serialized = serde_json::to_string(program).unwrap_or_default();
    serialized.to_ascii_lowercase().contains(needle)
}
