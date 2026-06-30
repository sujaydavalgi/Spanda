//! Platform event emission for verify-time tamper analysis.
//!
use spanda_audit::platform_event::names;
use spanda_audit::PlatformEvent;
use spanda_runtime::publish_platform_event;
use serde_json::json;

use crate::detect::{TamperReport, TamperStatus};

/// Record tamper platform events when verify-time analysis finds actionable findings.
pub fn record_tamper_platform_events(report: &TamperReport) {
    let should_emit = matches!(
        report.status,
        TamperStatus::Tampered | TamperStatus::Compromised | TamperStatus::Suspicious
    ) || !report.passed;
    if !should_emit {
        return;
    }
    let top_finding = report.findings.first().map(|f| {
        json!({
            "category": f.category,
            "severity": format!("{:?}", f.severity),
            "message": f.message,
        })
    });
    let event = PlatformEvent::new(
        names::TAMPER_DETECTED,
        "spanda-tamper",
        json!({
            "program": report.program,
            "status": format!("{:?}", report.status),
            "trust_score": report.trust_score,
            "passed": report.passed,
            "finding_count": report.findings.len(),
            "top_finding": top_finding,
        }),
    )
    .with_entity_id(format!("program/{}", report.program));
    publish_platform_event(None, &event);
}
