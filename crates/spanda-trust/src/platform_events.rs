//! Platform event emission for entity trust evaluation.
//!
use serde_json::json;
use spanda_audit::platform_event::names;
use spanda_audit::PlatformEvent;
use spanda_runtime::publish_platform_event;

use crate::entity_trust::EntityTrustReport;

/// Record trust platform events after entity trust evaluation.
pub fn record_entity_trust_platform_events(report: &EntityTrustReport) {
    let dimensions: Vec<_> = report
        .categories
        .iter()
        .map(|c| {
            json!({
                "name": c.name,
                "score": c.score,
                "passed": c.passed,
            })
        })
        .collect();
    let event = PlatformEvent::new(
        names::TRUST_UPDATED,
        "spanda-trust",
        json!({
            "entity_type": report.entity_type,
            "trust_status": report.trust_status,
            "score": report.score,
            "passed": report.passed,
            "dimensions": dimensions,
            "sources": report.sources,
        }),
    )
    .with_entity_id(report.entity_id.clone());
    publish_platform_event(None, &event);

    if !report.passed {
        let gate_event = PlatformEvent::new(
            names::TRUST_GATE_FAILED,
            "spanda-trust",
            json!({
                "threshold": "entity_trust",
                "score": report.score,
                "trust_status": report.trust_status,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(None, &gate_event);
    }
}
