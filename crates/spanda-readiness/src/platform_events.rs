//! Platform event emission for readiness evaluation.
//!
use spanda_audit::platform_event::names;
use spanda_audit::{AuditRuntime, PlatformEvent};
use spanda_runtime::publish_platform_event;
use serde_json::json;

use crate::entity_readiness::EntityReadinessReport;

/// Record readiness platform events for an entity readiness report.
pub fn record_readiness_platform_event(
    audit: &mut AuditRuntime,
    report: &EntityReadinessReport,
) {
    let event = PlatformEvent::new(
        names::READINESS_CHANGED,
        "spanda-readiness",
        json!({
            "entity_type": report.entity_type,
            "readiness_status": report.readiness_status,
            "mission_ready": report.mission_ready,
            "score": report.score,
            "issue_count": report.issues.len(),
            "sources": report.sources,
        }),
    )
    .with_entity_id(report.entity_id.clone());
    publish_platform_event(Some(audit), &event);

    if !report.mission_ready {
        let gate_event = PlatformEvent::new(
            names::READINESS_GATE_FAILED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "score": report.score,
                "issues": report.issues,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(Some(audit), &gate_event);
    }
}
