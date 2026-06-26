//! Security assurance rollup composing threat, integrity, trust, tamper, and audit engines.

use crate::detect::{generate_tamper_check, TamperReport, TamperSeverity, TamperStatus};
use crate::integrity::{generate_integrity_report, IntegrityReport};
use serde::{Deserialize, Serialize};
use spanda_ast::nodes::Program;
use spanda_security::{security_audit, SecurityReport, SecuritySeverity};
use spanda_threat::{analyze_threat_model, ThreatReport, ThreatRisk};

/// Output format for security assurance reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SecurityAssuranceFormat {
    #[default]
    Text,
    Json,
    Markdown,
}

/// One composed section in a security assurance rollup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssuranceSection {
    pub name: String,
    pub passed: bool,
    pub score: Option<u32>,
    pub summary: String,
    pub detail_count: u32,
}

/// Rollup security posture for a mission program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssuranceReport {
    pub program: String,
    pub posture: String,
    pub trust_score: u32,
    pub sections: Vec<SecurityAssuranceSection>,
    pub recommendations: Vec<String>,
    pub passed: bool,
}

/// Compose a security assurance report from existing analysis engines.
pub fn generate_security_assurance(
    program: &Program,
    source: &str,
    source_label: &str,
) -> SecurityAssuranceReport {
    // Roll up threat modeling, integrity, tamper trust, and secure-comm audit signals.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    // - `source` — raw program source for audit-oriented security analysis
    // - `source_label` — file label for the report
    //
    // Returns:
    // Security assurance rollup with section summaries and recommendations.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = generate_security_assurance(&program, &source, "rover.sd");

    let threat = analyze_threat_model(program, source_label);
    let integrity = generate_integrity_report(program, source_label, None, None);
    let tamper = generate_tamper_check(program, source_label);
    let audit = security_audit(source).unwrap_or_default();

    let sections = vec![
        section_from_threat(&threat),
        section_from_integrity(&integrity),
        section_from_tamper(&tamper),
        section_from_audit(&audit),
    ];

    let trust_score = tamper.trust_score;
    let posture = derive_posture(&tamper, &threat, trust_score);
    let recommendations = synthesize_recommendations(&threat, &integrity, &tamper, &audit);
    let passed = sections.iter().all(|section| section.passed) && tamper.passed;

    SecurityAssuranceReport {
        program: source_label.to_string(),
        posture,
        trust_score,
        sections,
        recommendations,
        passed,
    }
}

/// Render a security assurance report for CLI output.
pub fn format_security_assurance_report(
    report: &SecurityAssuranceReport,
    format: SecurityAssuranceFormat,
) -> String {
    // Format a security assurance rollup for text, JSON, or Markdown output.
    //
    // Parameters:
    // - `report` — composed assurance report
    // - `format` — output format selector
    //
    // Returns:
    // Rendered report string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_security_assurance_report(&report, SecurityAssuranceFormat::Text);

    match format {
        SecurityAssuranceFormat::Json => {
            serde_json::to_string_pretty(report).unwrap_or_else(|error| error.to_string())
        }
        SecurityAssuranceFormat::Markdown => {
            let mut lines = vec![
                format!("# Security Assurance: {}", report.program),
                format!(
                    "**Posture:** {} · **Trust score:** {}/100 · **Result:** {}",
                    report.posture,
                    report.trust_score,
                    if report.passed { "PASS" } else { "FAIL" }
                ),
                String::new(),
                "| Section | Score | Status | Summary |".into(),
                "|---------|-------|--------|---------|".into(),
            ];
            for section in &report.sections {
                let score = section
                    .score
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "—".into());
                lines.push(format!(
                    "| {} | {} | {} | {} |",
                    section.name,
                    score,
                    if section.passed { "PASS" } else { "FAIL" },
                    section.summary
                ));
            }
            if !report.recommendations.is_empty() {
                lines.push(String::new());
                lines.push("## Recommendations".into());
                for recommendation in &report.recommendations {
                    lines.push(format!("- {recommendation}"));
                }
            }
            lines.join("\n")
        }
        SecurityAssuranceFormat::Text => {
            let mut lines = vec![
                format!("Security assurance: {}", report.program),
                format!(
                    "Posture: {} · Trust score: {}/100 · Result: {}",
                    report.posture,
                    report.trust_score,
                    if report.passed { "PASS" } else { "FAIL" }
                ),
                "Sections:".into(),
            ];
            for section in &report.sections {
                let score = section
                    .score
                    .map(|value| format!("{value}/100"))
                    .unwrap_or_else(|| "n/a".into());
                lines.push(format!(
                    "  {} — {} — {} — {} ({} findings)",
                    section.name,
                    if section.passed { "PASS" } else { "FAIL" },
                    score,
                    section.summary,
                    section.detail_count
                ));
            }
            if !report.recommendations.is_empty() {
                lines.push("Recommendations:".into());
                for recommendation in &report.recommendations {
                    lines.push(format!("  - {recommendation}"));
                }
            }
            lines.join("\n")
        }
    }
}

fn section_from_threat(threat: &ThreatReport) -> SecurityAssuranceSection {
    let high_risk = threat
        .assessments
        .iter()
        .filter(|assessment| assessment.risk >= ThreatRisk::High)
        .count() as u32;
    SecurityAssuranceSection {
        name: "attack_surface".into(),
        passed: threat.passed,
        score: Some(100u32.saturating_sub(threat.risk_score)),
        summary: format!(
            "risk {}/100, {} surface items, {} high/critical threats",
            threat.risk_score,
            threat.attack_surface.len(),
            high_risk
        ),
        detail_count: threat.assessments.len() as u32,
    }
}

fn section_from_integrity(integrity: &IntegrityReport) -> SecurityAssuranceSection {
    SecurityAssuranceSection {
        name: "integrity".into(),
        passed: integrity.passed,
        score: None,
        summary: format!("{} artifacts hashed", integrity.artifacts.len()),
        detail_count: integrity.artifacts.len() as u32,
    }
}

fn section_from_tamper(tamper: &TamperReport) -> SecurityAssuranceSection {
    SecurityAssuranceSection {
        name: "tamper".into(),
        passed: tamper.passed,
        score: Some(tamper.trust_score),
        summary: format!(
            "status {:?}, {} findings",
            tamper.status,
            tamper.findings.len()
        ),
        detail_count: tamper.findings.len() as u32,
    }
}

fn section_from_audit(audit: &SecurityReport) -> SecurityAssuranceSection {
    let errors = audit
        .findings
        .iter()
        .filter(|finding| finding.severity == SecuritySeverity::Error)
        .count() as u32;
    let warnings = audit
        .findings
        .iter()
        .filter(|finding| finding.severity == SecuritySeverity::Warning)
        .count() as u32;
    SecurityAssuranceSection {
        name: "secure_comm_audit".into(),
        passed: !audit.has_errors(),
        score: Some(100u32.saturating_sub(errors.saturating_mul(25))),
        summary: format!("{errors} errors, {warnings} warnings"),
        detail_count: audit.findings.len() as u32,
    }
}

fn derive_posture(tamper: &TamperReport, threat: &ThreatReport, trust_score: u32) -> String {
    if tamper.status == TamperStatus::Compromised || trust_score < 40 {
        return "compromised".into();
    }
    if tamper.status == TamperStatus::Tampered
        || threat.risk_score >= 60
        || trust_score < 70
        || tamper
            .findings
            .iter()
            .any(|finding| finding.severity >= TamperSeverity::High)
    {
        return "elevated".into();
    }
    "trusted".into()
}

fn synthesize_recommendations(
    threat: &ThreatReport,
    integrity: &IntegrityReport,
    tamper: &TamperReport,
    audit: &SecurityReport,
) -> Vec<String> {
    let mut recommendations = Vec::new();

    for mitigation in threat.mitigations.iter().take(3) {
        recommendations.push(mitigation.clone());
    }

    if !integrity.passed {
        recommendations.push(
            "Compare integrity against an approved baseline with `spanda integrity --baseline`."
                .into(),
        );
    }

    if !tamper.passed {
        recommendations.push(
            "Resolve tamper findings and re-run `spanda tamper-check` before deployment.".into(),
        );
    }

    if audit.has_errors() {
        recommendations.push(
            "Fix secure communication audit errors before enabling remote command paths.".into(),
        );
    }

    if tamper
        .findings
        .iter()
        .any(|finding| finding.severity >= TamperSeverity::Critical)
    {
        recommendations.push(
            "Escalate critical tamper signals to safe mode and require human approval.".into(),
        );
    }

    recommendations.sort();
    recommendations.dedup();
    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;

    fn parse_fixture(relative: &str) -> (Program, String) {
        let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative);
        let source =
            std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path}: {error}"));
        let tokens = tokenize(&source).expect("tokenize");
        let program = parse(tokens).expect("parse");
        (program, source)
    }

    #[test]
    fn assurance_report_composes_sections_for_readiness_rover() {
        let (program, source) = parse_fixture("../../examples/showcase/readiness/rover.sd");
        let report = generate_security_assurance(&program, &source, "rover.sd");
        assert_eq!(report.sections.len(), 4);
        assert!(report
            .sections
            .iter()
            .any(|section| section.name == "attack_surface"));
        assert!(report
            .sections
            .iter()
            .any(|section| section.name == "tamper"));
        assert!(!report.passed);
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn assurance_report_formats_as_json() {
        let (program, source) = parse_fixture("../../examples/showcase/assurance/rover.sd");
        let report = generate_security_assurance(&program, &source, "rover.sd");
        let json = format_security_assurance_report(&report, SecurityAssuranceFormat::Json);
        assert!(json.contains("\"attack_surface\""));
        assert!(json.contains("\"trust_score\""));
    }
}
