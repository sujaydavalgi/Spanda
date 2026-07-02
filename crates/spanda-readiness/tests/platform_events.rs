//! Entity health and readiness platform event emission tests.

use spanda_audit::platform_event::names;
use spanda_audit::AuditRuntime;
use spanda_readiness::{
    record_entity_health_platform_events, record_readiness_platform_event,
    reset_platform_event_caches_for_tests, EntityHealthDiagnostic, EntityHealthMetrics,
    EntityHealthReport, EntityReadinessFinding, EntityReadinessReport,
};
use spanda_runtime::platform_event_runtime::{set_platform_event_runtime, PlatformEventRuntime};
use std::sync::{Arc, Mutex};

struct CapturePlatformEvents {
    events: Mutex<Vec<String>>,
}

impl PlatformEventRuntime for CapturePlatformEvents {
    fn record_platform_event(&self, event: &spanda_audit::PlatformEvent) {
        self.events
            .lock()
            .unwrap()
            .push(event.event_type.as_str().to_string());
    }
}

fn sample_health_report(
    entity_id: &str,
    health_status: &str,
    severity: &str,
) -> EntityHealthReport {
    EntityHealthReport {
        entity_id: entity_id.into(),
        entity_type: "robot".into(),
        health_status: health_status.into(),
        lifecycle_state: "active".into(),
        diagnostics: vec![EntityHealthDiagnostic {
            category: "health".into(),
            severity: severity.into(),
            message: format!("Entity health is {health_status}"),
        }],
        metrics: EntityHealthMetrics::default(),
        children_checked: 0,
        sources: vec!["entity_snapshot".into()],
    }
}

fn sample_readiness_report(
    entity_id: &str,
    readiness_status: &str,
    mission_ready: bool,
    score: Option<u32>,
) -> EntityReadinessReport {
    EntityReadinessReport {
        entity_id: entity_id.into(),
        entity_type: "robot".into(),
        readiness_status: readiness_status.into(),
        mission_ready,
        score,
        issues: if mission_ready {
            Vec::new()
        } else {
            vec![EntityReadinessFinding {
                factor: "device_pool".into(),
                severity: "error".into(),
                message: "Device blocked".into(),
            }]
        },
        capabilities: Vec::new(),
        children_checked: 0,
        sources: vec!["entity_snapshot".into()],
    }
}

fn count_event(events: &[String], name: &str) -> usize {
    events
        .iter()
        .filter(|event_type| event_type.as_str() == name)
        .count()
}

#[test]
fn platform_event_transitions_emit_once_per_state_change() {
    let capture = Arc::new(CapturePlatformEvents {
        events: Mutex::new(Vec::new()),
    });
    let _ = set_platform_event_runtime(Arc::clone(&capture) as Arc<dyn PlatformEventRuntime>);

    reset_platform_event_caches_for_tests();
    record_entity_health_platform_events(&sample_health_report("rover-001", "healthy", "warning"));
    record_entity_health_platform_events(&sample_health_report("rover-001", "healthy", "warning"));
    record_entity_health_platform_events(&sample_health_report("rover-001", "degraded", "warning"));
    let events = capture.events.lock().unwrap();
    assert_eq!(count_event(&events, names::HEALTH_CHANGED), 2);
    drop(events);

    reset_platform_event_caches_for_tests();
    let start = capture.events.lock().unwrap().len();
    record_entity_health_platform_events(&sample_health_report("rover-002", "degraded", "error"));
    record_entity_health_platform_events(&sample_health_report("rover-002", "degraded", "error"));
    record_entity_health_platform_events(&sample_health_report("rover-002", "healthy", "warning"));
    record_entity_health_platform_events(&sample_health_report(
        "rover-002",
        "degraded",
        "critical",
    ));
    let events = capture.events.lock().unwrap();
    let section = &events[start..];
    assert_eq!(count_event(section, names::DEGRADED_MODE_ENTERED), 2);
    drop(events);

    reset_platform_event_caches_for_tests();
    let start = capture.events.lock().unwrap().len();
    let mut audit = AuditRuntime::new("ReadinessEventTest", vec![]);
    record_readiness_platform_event(
        &mut audit,
        &sample_readiness_report("rover-003", "ready", true, Some(90)),
    );
    record_readiness_platform_event(
        &mut audit,
        &sample_readiness_report("rover-003", "ready", true, Some(90)),
    );
    record_readiness_platform_event(
        &mut audit,
        &sample_readiness_report("rover-003", "blocked", false, Some(40)),
    );
    let events = capture.events.lock().unwrap();
    let section = &events[start..];
    assert_eq!(count_event(section, names::READINESS_CHANGED), 2);
    assert_eq!(count_event(section, names::READINESS_GATE_FAILED), 1);
}
