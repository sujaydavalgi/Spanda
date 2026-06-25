//! Runtime tamper analysis from mission trace security and capability events.

use crate::detect::{TamperFinding, TamperReport, TamperSeverity, TamperStatus};
use serde::{Deserialize, Serialize};

/// One recorded simulation frame for runtime tamper analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceFrame {
    pub sim_time_ms: f64,
    pub event: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

/// Mission trace file consumed by runtime tamper-check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionTrace {
    pub version: u32,
    pub source: String,
    #[serde(default)]
    pub deterministic: bool,
    pub frames: Vec<TraceFrame>,
}

/// Analyze a mission trace for runtime tamper and intrusion signals.
pub fn generate_runtime_tamper_check(trace: &MissionTrace, source_label: &str) -> TamperReport {
    // Scan trace frames for capability denials, unauthorized actions, and tamper events.
    //
    // Parameters:
    // - `trace` — deserialized mission trace
    // - `source_label` — file label for the report
    //
    // Returns:
    // Tamper report with runtime findings and trust score.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = generate_runtime_tamper_check(&trace, "mission.trace");

    let mut findings = Vec::new();

    for frame in &trace.frames {
        collect_frame_findings(frame, &mut findings);
    }

    let trust_score = compute_runtime_trust_score(&findings);
    let status = derive_runtime_status(&findings, trust_score);
    let passed = !findings.iter().any(|finding| {
        matches!(
            finding.severity,
            TamperSeverity::High | TamperSeverity::Critical
        )
    });

    TamperReport {
        program: source_label.into(),
        status,
        trust_score,
        findings,
        passed,
    }
}

fn collect_frame_findings(frame: &TraceFrame, findings: &mut Vec<TamperFinding>) {
    let event_lower = frame.event.to_ascii_lowercase();

    if event_indicates_tamper(&event_lower) {
        findings.push(runtime_finding(
            "runtime_event",
            severity_for_event(&event_lower),
            format!("Runtime tamper signal in trace event `{event}`", event = frame.event),
            Some(frame.payload.to_string()),
            frame.sim_time_ms,
        ));
    }

    if let Some(obj) = frame.payload.as_object() {
        if payload_indicates_denial(obj) {
            let kind = obj
                .get("kind")
                .or_else(|| obj.get("event"))
                .and_then(|value| value.as_str())
                .unwrap_or("denied");
            findings.push(runtime_finding(
                "capability_monitor",
                TamperSeverity::High,
                format!("Unauthorized or denied runtime action: {kind}"),
                Some(frame.payload.to_string()),
                frame.sim_time_ms,
            ));
        }

        if payload_indicates_tamper(obj) {
            findings.push(runtime_finding(
                "runtime_payload",
                TamperSeverity::Critical,
                "Trace payload reports tamper or compromise".into(),
                Some(frame.payload.to_string()),
                frame.sim_time_ms,
            ));
        }
    }
}

fn event_indicates_tamper(event: &str) -> bool {
    [
        "tamper",
        "intrusion",
        "capability_denied",
        "agent_capability_denied",
        "unauthorized",
        "privilege_escalation",
        "runtime_injection",
        "integrity_violation",
    ]
    .iter()
    .any(|needle| event.contains(needle))
}

fn payload_indicates_denial(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
    if obj
        .get("denied")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return true;
    }

    if obj
        .get("unauthorized")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return true;
    }

    matches!(
        obj.get("kind")
            .or_else(|| obj.get("event"))
            .and_then(|value| value.as_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some(kind) if kind.contains("capability_denied")
            || kind.contains("agent_capability_denied")
            || kind.contains("unauthorized")
    )
}

fn payload_indicates_tamper(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
    if obj
        .get("tampered")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return true;
    }

    matches!(
        obj.get("status")
            .and_then(|value| value.as_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("tampered" | "compromised")
    )
}

fn severity_for_event(event: &str) -> TamperSeverity {
    if event.contains("critical") || event.contains("compromise") || event.contains("injection") {
        TamperSeverity::Critical
    } else if event.contains("denied") || event.contains("unauthorized") || event.contains("intrusion")
    {
        TamperSeverity::High
    } else {
        TamperSeverity::Medium
    }
}

fn runtime_finding(
    category: &str,
    severity: TamperSeverity,
    message: String,
    evidence: Option<String>,
    sim_time_ms: f64,
) -> TamperFinding {
    TamperFinding {
        category: category.into(),
        severity,
        message,
        evidence: evidence.map(|detail| format!("sim_time_ms={sim_time_ms:.1} {detail}")),
        line: None,
    }
}

fn compute_runtime_trust_score(findings: &[TamperFinding]) -> u32 {
    let mut score = 100u32;
    for finding in findings {
        let penalty = match finding.severity {
            TamperSeverity::Info => 2,
            TamperSeverity::Low => 5,
            TamperSeverity::Medium => 12,
            TamperSeverity::High => 25,
            TamperSeverity::Critical => 40,
        };
        score = score.saturating_sub(penalty);
    }
    score
}

fn derive_runtime_status(findings: &[TamperFinding], trust_score: u32) -> TamperStatus {
    if findings
        .iter()
        .any(|finding| finding.severity == TamperSeverity::Critical)
    {
        return TamperStatus::Compromised;
    }
    if findings
        .iter()
        .any(|finding| finding.severity == TamperSeverity::High)
    {
        return TamperStatus::Tampered;
    }
    if trust_score < 70 {
        return TamperStatus::Suspicious;
    }
    if findings.is_empty() {
        TamperStatus::Trusted
    } else {
        TamperStatus::Suspicious
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_trace_passes_runtime_tamper_check() {
        let trace = MissionTrace {
            version: 1,
            source: "ok.sd".into(),
            deterministic: true,
            frames: vec![TraceFrame {
                sim_time_ms: 0.0,
                event: "topic_publish".into(),
                payload: serde_json::json!({"topic": "/telemetry"}),
            }],
        };
        let report = generate_runtime_tamper_check(&trace, "ok.trace");
        assert!(report.passed);
        assert_eq!(report.status, TamperStatus::Trusted);
    }

    #[test]
    fn capability_denial_fails_runtime_tamper_check() {
        let trace = MissionTrace {
            version: 1,
            source: "intrusion.sd".into(),
            deterministic: true,
            frames: vec![TraceFrame {
                sim_time_ms: 120.0,
                event: "security_audit".into(),
                payload: serde_json::json!({
                    "kind": "agent_capability_denied",
                    "agent": "Intruder",
                    "action": "execute"
                }),
            }],
        };
        let report = generate_runtime_tamper_check(&trace, "intrusion.trace");
        assert!(!report.passed);
        assert!(matches!(
            report.status,
            TamperStatus::Tampered | TamperStatus::Compromised
        ));
    }
}
