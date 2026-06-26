//! Generate architecture decision records from parsed programs.

use serde::{Deserialize, Serialize};
use spanda_ast::assurance_decl::RecoveryPolicyDecl;
use spanda_ast::foundations::{DeployDecl, HealthPolicyDecl, KillSwitchDecl, MissionDecl};
use spanda_ast::nodes::{Program, RobotDecl, SafetyBlock, SafetyRule};

/// Output format for ADR reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdrFormat {
    #[default]
    Markdown,
    Json,
}

/// Single architecture decision record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdrRecord {
    pub id: String,
    pub title: String,
    pub status: String,
    pub context: String,
    pub decision: String,
    pub consequences: String,
}

/// ADR generation report for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdrReport {
    pub program: String,
    pub records: Vec<AdrRecord>,
}

/// Generate ADR records from program declarations.
pub fn generate_adrs(program: &Program, source_label: &str) -> AdrReport {
    // Derive architecture decision records from deploy, safety, and assurance declarations.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    // - `source_label` — file label
    //
    // Returns:
    // ADR report with numbered records.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = generate_adrs(&program, "rover.sd");

    let mut records = Vec::new();
    let mut counter = 1usize;

    let Program::Program {
        deployments,
        robots,
        kill_switches,
        health_policies,
        recovery_policies,
        operational_policies,
        assurance_cases,
        ..
    } = program;

    for deploy in deployments {
        let DeployDecl::DeployDecl {
            robot_name,
            targets,
            ..
        } = deploy;
        for target in targets {
            push_record(
                &mut records,
                &mut counter,
                format!("Deploy {robot_name} to {target}"),
                format!("Robot '{robot_name}' must run on compatible hardware."),
                format!("Deploy robot '{robot_name}' to hardware profile '{target}'."),
                "Hardware compatibility verification gates rollout to this target.".to_string(),
            );
        }
    }

    for robot in robots {
        let RobotDecl::RobotDecl {
            name,
            exposes_capabilities,
            safety,
            mission,
            ..
        } = robot;
        if !exposes_capabilities.is_empty() {
            push_record(
                &mut records,
                &mut counter,
                format!("Capability surface for {name}"),
                format!("Robot '{name}' advertises operational capabilities to missions and fleet peers."),
                format!(
                    "Expose capabilities: {}.",
                    exposes_capabilities.join(", ")
                ),
                "Downstream missions must declare required capabilities against this surface.".to_string(),
            );
        }
        if let Some(safety) = safety {
            if let Some(limit) = safety_max_speed(safety) {
                push_record(
                    &mut records,
                    &mut counter,
                    format!("Safety speed cap for {name}"),
                    format!(
                        "Actuator commands for '{name}' must remain within safe velocity bounds."
                    ),
                    format!("Cap linear speed at {limit:.2} m/s in robot safety block."),
                    "Verification and runtime safety checks enforce the declared cap.".to_string(),
                );
            }
        }
        if let Some(mission) = mission {
            let MissionDecl::MissionDecl {
                name: mission_name,
                duration_hours,
                required_capabilities,
                ..
            } = mission;
            let title = mission_name
                .as_deref()
                .map(|n| format!("Mission {n} on {name}"))
                .unwrap_or_else(|| format!("Mission plan for {name}"));
            let duration = duration_hours
                .map(|hours| format!("duration {hours:.2} h"))
                .unwrap_or_else(|| "unspecified duration".into());
            let capabilities = if required_capabilities.is_empty() {
                "no explicit capability requirements".into()
            } else {
                format!("requires {}", required_capabilities.join(", "))
            };
            push_record(
                &mut records,
                &mut counter,
                title,
                format!("Robot '{name}' executes an operational mission with bounded runtime."),
                format!("Mission defined with {duration}; {capabilities}."),
                "Readiness and resource estimation use mission timing for go/no-go and budgeting."
                    .to_string(),
            );
        }
    }

    for kill_switch in kill_switches {
        let KillSwitchDecl::KillSwitchDecl { name, .. } = kill_switch;
        push_record(
            &mut records,
            &mut counter,
            format!("Kill switch {name}"),
            "Operators require a deterministic emergency stop path.".to_string(),
            format!("Declare kill_switch '{name}' with critical priority handler."),
            "Health and compliance profiles may require at least one kill switch.".to_string(),
        );
    }

    for policy in health_policies {
        let HealthPolicyDecl::HealthPolicyDecl { name, .. } = policy;
        push_record(
            &mut records,
            &mut counter,
            format!("Health policy {name}"),
            "Component degradation must trigger predictable operating modes.".to_string(),
            format!("Use health_policy '{name}' to map health states to robot actions."),
            "Readiness scoring and fleet health dashboards consume health_check results."
                .to_string(),
        );
    }

    for policy in recovery_policies {
        let RecoveryPolicyDecl::RecoveryPolicyDecl { name, .. } = policy;
        push_record(
            &mut records,
            &mut counter,
            format!("Recovery policy {name}"),
            "Transient faults should be handled without unsafe actuator output.".to_string(),
            format!("Declare recovery_policy '{name}' with validated recovery actions."),
            "Chaos experiments and assurance recovery planners exercise these branches."
                .to_string(),
        );
    }

    for policy in operational_policies {
        use spanda_ast::policy_decl::OperationalPolicyDecl;
        let OperationalPolicyDecl::OperationalPolicyDecl { name, rules, .. } = policy;
        push_record(
            &mut records,
            &mut counter,
            format!("Operational policy {name}"),
            "Organization-wide operational constraints apply at verify time.".to_string(),
            format!(
                "Program policy '{name}' declares {} verify-time rule(s).",
                rules.len()
            ),
            format!("Use `spanda verify --policy {name}` to enforce the declared rule set."),
        );
    }

    if !assurance_cases.is_empty() {
        push_record(
            &mut records,
            &mut counter,
            "Assurance evidence cases".to_string(),
            "Safety and operational claims require traceable evidence artifacts.".to_string(),
            format!(
                "Maintain {} assurance_case declaration(s) with linked evidence.",
                assurance_cases.len()
            ),
            "Medical and defense compliance profiles require non-empty assurance cases."
                .to_string(),
        );
    }

    if records.is_empty() {
        push_record(
            &mut records,
            &mut counter,
            "Baseline architecture".to_string(),
            format!("Program '{source_label}' contains no deploy or policy declarations yet."),
            "Continue elaborating robots, safety rules, and deployment targets.".to_string(),
            "Re-run `spanda adr` after adding mission-critical declarations.".to_string(),
        );
    }

    AdrReport {
        program: source_label.into(),
        records,
    }
}

/// Format an ADR report for CLI output.
pub fn format_adr_report(report: &AdrReport, format: AdrFormat) -> String {
    match format {
        AdrFormat::Json => serde_json::to_string_pretty(report).unwrap_or_else(|e| e.to_string()),
        AdrFormat::Markdown => format_adr_markdown(report),
    }
}

fn format_adr_markdown(report: &AdrReport) -> String {
    let mut lines = vec![
        format!("# Architecture Decision Records — {}", report.program),
        String::new(),
    ];
    for record in &report.records {
        lines.push(format!("## {} — {}", record.id, record.title));
        lines.push(format!("**Status:** {}", record.status));
        lines.push(String::new());
        lines.push("### Context".into());
        lines.push(record.context.clone());
        lines.push(String::new());
        lines.push("### Decision".into());
        lines.push(record.decision.clone());
        lines.push(String::new());
        lines.push("### Consequences".into());
        lines.push(record.consequences.clone());
        lines.push(String::new());
    }
    lines.join("\n").trim_end().to_string()
}

fn push_record(
    records: &mut Vec<AdrRecord>,
    counter: &mut usize,
    title: String,
    context: String,
    decision: String,
    consequences: String,
) {
    records.push(AdrRecord {
        id: format!("ADR-{counter:03}"),
        title,
        status: "Accepted".into(),
        context,
        decision,
        consequences,
    });
    *counter += 1;
}

fn safety_max_speed(safety: &SafetyBlock) -> Option<f64> {
    let SafetyBlock::SafetyBlock { rules, .. } = safety;
    let mut limit = None;
    for rule in rules {
        if let SafetyRule::MaxSpeedRule { value, unit, .. } = rule {
            if let Some(mps) = expr_to_mps(value, *unit) {
                limit = Some(limit.map_or(mps, |current: f64| current.max(mps)));
            }
        }
    }
    limit
}

fn expr_to_mps(value: &spanda_ast::nodes::Expr, unit: spanda_ast::nodes::UnitKind) -> Option<f64> {
    use spanda_ast::nodes::{Expr, LiteralValue, UnitKind};
    let number = match value {
        Expr::UnitLiteralExpr { value, unit, .. } => {
            return match unit {
                UnitKind::MPerS => Some(*value),
                UnitKind::KmPerH => Some(*value / 3.6),
                UnitKind::Mph => Some(*value * 0.44704),
                _ => None,
            };
        }
        Expr::LiteralExpr {
            value: LiteralValue::Number(n),
            ..
        } => *n,
        _ => return None,
    };
    match unit {
        UnitKind::MPerS => Some(number),
        UnitKind::KmPerH => Some(number / 3.6),
        UnitKind::Mph => Some(number * 0.44704),
        _ => None,
    }
}
