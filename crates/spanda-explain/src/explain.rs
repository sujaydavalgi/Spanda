//! Static and trace explainability builders.

use spanda_ast::nodes::Program;
use spanda_config::{detect_config_drift, format_drift_lines, ResolvedSystemConfig};
use spanda_contract::verify_contract;
use spanda_decision::audit_decisions_from_trace;
use spanda_hardware::{verify_program_compatibility, VerifyOptions};
use spanda_package::evaluate_package_trust;
use spanda_readiness::{
    evaluate_deployment_gates, evaluate_readiness, evaluate_safety_coverage, generate_safety_report,
    verify_mission, DeploymentGatePolicy, ReadinessOptions,
};

use crate::report::{ExplainReport, ExplainSection};

/// Optional configuration context for program explain reports.
#[derive(Debug, Clone, Default)]
pub struct ExplainProgramOptions<'a> {
    pub system_config: Option<&'a ResolvedSystemConfig>,
    pub baseline_config: Option<&'a ResolvedSystemConfig>,
}

fn program_structure(program: &Program) -> ExplainSection {
    // Description:
    //     Summarize top-level program structure.
    //
    // Parameters:
    // - `program` — parsed program
    //
    // Returns:
    // Structure explain section.
    //
    // Options:
    // None.
    //
    // Example:
    // let section = program_structure(&program);

    let Program::Program {
        robots,
        mission_plans,
        recovery_policies,
        continuity_policies,
        fleets,
        ..
    } = program;
    ExplainSection {
        topic: "structure".into(),
        summary: format!(
            "{} robot(s), {} mission plan(s), {} recovery policy(ies), {} continuity policy(ies), {} fleet(s)",
            robots.len(),
            mission_plans.len(),
            recovery_policies.len(),
            continuity_policies.len(),
            fleets.len()
        ),
        details: robots
            .iter()
            .map(|robot| {
                let spanda_ast::nodes::RobotDecl::RobotDecl { name, .. } = robot;
                format!("robot {name}")
            })
            .collect(),
    }
}

/// Explain program structure and linked operational surfaces.
pub fn explain_program(program: &Program, source_label: &str) -> ExplainReport {
    explain_program_with_options(program, source_label, &ExplainProgramOptions::default())
}

/// Explain program structure with optional configuration, drift, and gate previews.
pub fn explain_program_with_options(
    program: &Program,
    source_label: &str,
    options: &ExplainProgramOptions<'_>,
) -> ExplainReport {
    // Build a multi-section explainability report for a program.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    // - `options` — optional resolved config and baseline for drift/gates
    //
    // Returns:
    // Explainability report with structure, contract, readiness, verify, safety, and optional config sections.
    //
    // Options:
    // `ExplainProgramOptions::system_config`, `ExplainProgramOptions::baseline_config`.
    //
    // Example:
    // let report = explain_program_with_options(&program, "rover.sd", &options);

    let mut sections = vec![program_structure(program)];
    sections.push(explain_readiness(program, source_label).sections[0].clone());
    sections.push(explain_verify(program, source_label).sections[0].clone());
    sections.push(explain_safety(program, source_label).sections[0].clone());
    let contract = verify_contract(program, source_label);
    sections.push(ExplainSection {
        topic: "contract".into(),
        summary: if contract.passed {
            "Mission contract verification passed".into()
        } else {
            format!(
                "Mission contract verification failed ({} issue(s))",
                contract.issues.len()
            )
        },
        details: contract
            .checks
            .iter()
            .map(|check| format!("{}: {}", check.name, check.detail))
            .collect(),
    });
    if let Some(cfg) = options.system_config {
        let validation = &cfg.validation;
        sections.push(ExplainSection {
            topic: "configuration".into(),
            summary: if validation.passed {
                "Configuration validation passed".into()
            } else {
                format!(
                    "Configuration validation failed ({} error(s))",
                    validation.error_count()
                )
            },
            details: validation
                .findings
                .iter()
                .map(|issue| format!("[{:?}] {}", issue.severity, issue.message))
                .collect(),
        });
        let readiness_options = ReadinessOptions {
            system_config: Some(std::sync::Arc::new(cfg.clone())),
            ..ReadinessOptions::default()
        };
        let gates = evaluate_deployment_gates(
            program,
            source_label,
            &readiness_options,
            &DeploymentGatePolicy::default(),
        );
        sections.push(ExplainSection {
            topic: "deployment_gates".into(),
            summary: if gates.passed {
                "All deployment gates passed".into()
            } else {
                format!(
                    "{} deployment gate(s) failed",
                    gates.gates.iter().filter(|gate| !gate.passed).count()
                )
            },
            details: gates
                .gates
                .iter()
                .map(|gate| {
                    format!(
                        "{}: {}",
                        gate.name,
                        if gate.passed {
                            gate.message.clone()
                        } else {
                            format!("FAIL — {}", gate.message)
                        }
                    )
                })
                .collect(),
        });
        if !cfg.packages.is_empty() {
            let mut details = Vec::new();
            let mut low = 0usize;
            for package in &cfg.packages {
                let trust = evaluate_package_trust(package, None, Some(&cfg.project_root));
                if !trust.passed {
                    low += 1;
                }
                details.push(format!(
                    "{} v{} — {}/100 ({})",
                    trust.package, trust.version, trust.score, trust.tier
                ));
            }
            sections.push(ExplainSection {
                topic: "package_trust".into(),
                summary: if low == 0 {
                    "Configured packages meet trust threshold".into()
                } else {
                    format!("{low} configured package(s) below trust threshold")
                },
                details,
            });
        }
    }
    if let (Some(baseline), Some(current)) = (options.baseline_config, options.system_config) {
        let drift = detect_config_drift(baseline, current);
        let lines = format_drift_lines(&drift);
        sections.push(ExplainSection {
            topic: "drift".into(),
            summary: if drift.findings.is_empty() {
                "No configuration drift from baseline".into()
            } else {
                format!("{} configuration drift finding(s)", drift.findings.len())
            },
            details: lines,
        });
    }
    ExplainReport {
        program: source_label.into(),
        sections,
    }
}

/// Explain readiness scoring failures and blockers.
pub fn explain_readiness(program: &Program, source_label: &str) -> ExplainReport {
    // Description:
    //     Explain readiness go/no-go results in plain language.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Explainability report with readiness section.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_readiness(&program, "rover.sd");

    let report = evaluate_readiness(program, &ReadinessOptions::default());
    let summary = if report.mission_ready {
        format!(
            "Mission ready with score {}/{}",
            report.score.total, report.score.maximum
        )
    } else {
        format!(
            "Mission not ready — score {}/{}",
            report.score.total, report.score.maximum
        )
    };
    let details = report
        .issues
        .iter()
        .map(|issue| format!("[{:?}] {}", issue.severity, issue.message))
        .collect();
    ExplainReport {
        program: source_label.into(),
        sections: vec![ExplainSection {
            topic: "readiness".into(),
            summary,
            details,
        }],
    }
}

/// Explain hardware and mission verification results.
pub fn explain_verify(program: &Program, source_label: &str) -> ExplainReport {
    // Description:
    //     Explain verify compatibility and mission achievability.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Explainability report with verify section.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_verify(&program, "rover.sd");

    let hw = verify_program_compatibility(program, &VerifyOptions::default());
    let missions = verify_mission(program, None);
    let mut details = Vec::new();
    for item in &hw.items {
        if item.severity != spanda_hardware::CompatSeverity::Pass {
            details.push(format!("[{}] {}", item.category, item.message));
        }
    }
    for mission in &missions {
        if !mission.achievable {
            details.extend(mission.issues.iter().cloned());
        }
    }
    let summary = if hw.compatible && missions.iter().all(|m| m.achievable) {
        "Hardware and mission verification passed".into()
    } else {
        format!(
            "Verification reported {} hardware item(s) and {} mission issue(s)",
            details.len(),
            missions.iter().map(|m| m.issues.len()).sum::<usize>()
        )
    };
    ExplainReport {
        program: source_label.into(),
        sections: vec![ExplainSection {
            topic: "verify".into(),
            summary,
            details,
        }],
    }
}

/// Explain safety rules and coverage gaps.
pub fn explain_safety(program: &Program, source_label: &str) -> ExplainReport {
    // Description:
    //     Explain safety case and scenario coverage.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Explainability report with safety section.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_safety(&program, "rover.sd");

    let safety = generate_safety_report(program, source_label);
    let coverage = evaluate_safety_coverage(program, source_label);
    let mut details = safety.safety_rules.clone();
    details.extend(
        coverage
            .scenarios
            .iter()
            .filter(|scenario| scenario.status != spanda_readiness::SafetyCoverageStatus::Covered)
            .map(|scenario| format!("{}: {:?}", scenario.name, scenario.gaps)),
    );
    let summary = format!(
        "Safety deployable={} coverage={}%",
        safety.deployable, coverage.overall_coverage_pct
    );
    ExplainReport {
        program: source_label.into(),
        sections: vec![ExplainSection {
            topic: "safety".into(),
            summary,
            details,
        }],
    }
}

/// Explain autonomous decisions from a mission trace with full decision context.
pub fn explain_decision_trace(trace_path: &str) -> Result<ExplainReport, String> {
    // Build a multi-section explainability report from decision audit records.
    //
    // Parameters:
    // - `trace_path` — path to mission trace JSON
    //
    // Returns:
    // Explainability report or load error.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_decision_trace("mission.trace")?;

    let audit = audit_decisions_from_trace(trace_path)?;
    let mut sections = Vec::new();
    sections.push(ExplainSection {
        topic: "summary".into(),
        summary: format!("{} autonomous decision(s) in trace", audit.decision_count),
        details: audit
            .chains
            .iter()
            .filter_map(|chain| chain.mission.as_ref().map(|mission| format!("mission: {mission}")))
            .collect(),
    });
    for record in &audit.timeline.decisions {
        let mut details = vec![
            format!("reason: {}", record.reason),
            format!("source event: {}", record.source_event),
        ];
        if let Some(mission) = &record.mission {
            details.push(format!("mission: {mission}"));
        }
        if !record.evidence.fields.is_empty() {
            details.push(format!(
                "evidence: {}",
                serde_json::to_string(&record.evidence.fields).unwrap_or_default()
            ));
        }
        if !record.alternatives_considered.is_empty() {
            details.push(format!(
                "rejected alternatives: {}",
                serde_json::to_string(&record.alternatives_considered).unwrap_or_default()
            ));
        }
        if !record.safety_checks.is_empty() {
            details.push(format!(
                "safety checks: {}",
                serde_json::to_string(&record.safety_checks).unwrap_or_default()
            ));
        }
        if let Some(action) = &record.action {
            details.push(format!(
                "chosen action: {}",
                serde_json::to_string(action).unwrap_or_default()
            ));
        }
        sections.push(ExplainSection {
            topic: format!("decision/{}", record.decision_id),
            summary: format!(
                "At T+{:.0}ms the system chose '{}' because {}",
                record.timestamp_ms, record.decision, record.reason
            ),
            details,
        });
    }
    Ok(ExplainReport {
        program: trace_path.into(),
        sections,
    })
}

/// Explain decisions recorded in a mission trace.
pub fn explain_trace(trace_path: &str) -> Result<ExplainReport, String> {
    // Description:
    //     Explain autonomous decisions from a mission trace file.
    //
    // Parameters:
    // - `trace_path` — path to trace JSON
    //
    // Returns:
    // Explainability report or load error.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_trace("mission.trace")?;

    let audit = audit_decisions_from_trace(trace_path)?;
    let details = audit
        .timeline
        .decisions
        .iter()
        .map(|record| {
            format!(
                "{} @ {:.0}ms — {} ({})",
                record.decision_id, record.timestamp_ms, record.decision, record.reason
            )
        })
        .collect();
    Ok(ExplainReport {
        program: trace_path.into(),
        sections: vec![ExplainSection {
            topic: "decisions".into(),
            summary: format!("{} decision(s) in trace", audit.decision_count),
            details,
        }],
    })
}
