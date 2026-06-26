//! Rule-based improvement suggestions from readiness and policy analysis.

use serde::{Deserialize, Serialize};
use spanda_ast::nodes::Program;
use spanda_policy::list_policies;
use spanda_readiness::{audit_program, evaluate_readiness, ReadinessOptions, ReadinessSeverity};

/// Severity for a suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// One actionable improvement suggestion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggestion {
    pub category: String,
    pub severity: SuggestSeverity,
    pub message: String,
    pub recommendation: String,
}

/// Suggestion report for a parsed program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestReport {
    pub program: String,
    pub suggestions: Vec<Suggestion>,
    pub passed: bool,
}

/// Suggest improvements for a parsed program using static analysis only.
pub fn suggest_program(program: &Program, source_label: &str) -> SuggestReport {
    // Compose readiness, audit, and policy-gap suggestions without external LLM calls.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Suggestion report ordered by severity.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = suggest_program(&program, "rover.sd");

    let mut suggestions = Vec::new();
    let readiness = evaluate_readiness(program, &ReadinessOptions::default());
    for issue in readiness.issues {
        suggestions.push(Suggestion {
            category: "readiness".into(),
            severity: readiness_severity(issue.severity),
            message: issue.message,
            recommendation: "Address readiness blocker before deploy".into(),
        });
    }
    let audit = audit_program(program, source_label);
    for finding in audit.findings {
        suggestions.push(Suggestion {
            category: finding.category,
            severity: readiness_severity(finding.severity),
            message: finding.message,
            recommendation: "Resolve safety audit finding".into(),
        });
    }
    if list_policies(program).is_empty() {
        suggestions.push(Suggestion {
            category: "policy".into(),
            severity: SuggestSeverity::Medium,
            message: "No operational policy block declared".into(),
            recommendation:
                "Add `policy Name { ... }` and verify with `spanda verify --policy Name`".into(),
        });
    }
    suggestions.sort_by(|left, right| right.severity.cmp(&left.severity));
    let passed = !suggestions.iter().any(|item| {
        matches!(
            item.severity,
            SuggestSeverity::Critical | SuggestSeverity::High
        )
    });
    SuggestReport {
        program: source_label.into(),
        suggestions,
        passed,
    }
}

/// Format a suggestion report for CLI output.
pub fn format_suggest_report(report: &SuggestReport, json: bool) -> String {
    // Render suggestion report as JSON or human-readable text.
    //
    // Parameters:
    // - `report` — suggestion report
    // - `json` — emit JSON when true
    //
    // Returns:
    // Formatted output string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_suggest_report(&report, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|error| error.to_string());
    }
    let mut lines = vec![
        format!("Suggestions for {}", report.program),
        if report.passed {
            "Result: PASS (no critical/high suggestions)".into()
        } else {
            "Result: REVIEW (critical/high suggestions present)".into()
        },
    ];
    if report.suggestions.is_empty() {
        lines.push("No suggestions.".into());
    } else {
        for item in &report.suggestions {
            lines.push(format!(
                "  [{:?}] {} — {}\n    → {}",
                item.severity, item.category, item.message, item.recommendation
            ));
        }
    }
    lines.join("\n")
}

fn readiness_severity(severity: ReadinessSeverity) -> SuggestSeverity {
    match severity {
        ReadinessSeverity::Critical => SuggestSeverity::Critical,
        ReadinessSeverity::High => SuggestSeverity::High,
        ReadinessSeverity::Medium => SuggestSeverity::Medium,
        ReadinessSeverity::Low => SuggestSeverity::Low,
        ReadinessSeverity::Info => SuggestSeverity::Info,
    }
}
