//! Platform event emission for readiness evaluation.
//!
use serde_json::json;
use spanda_audit::platform_event::names;
use spanda_audit::{AuditRuntime, PlatformEvent};
use spanda_runtime::publish_platform_event;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::entity_health::EntityHealthReport;
use crate::entity_readiness::EntityReadinessReport;

static HEALTH_STATUS_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReadinessSnapshot {
    readiness_status: String,
    mission_ready: bool,
    score: Option<u32>,
}

static READINESS_SNAPSHOT_CACHE: LazyLock<Mutex<HashMap<String, ReadinessSnapshot>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static MISSION_READY_CACHE: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static DEGRADED_STATE_CACHE: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Clear cached platform event transition state (tests only).
#[doc(hidden)]
pub fn reset_platform_event_caches_for_tests() {
    HEALTH_STATUS_CACHE
        .lock()
        .expect("health status cache")
        .clear();
    READINESS_SNAPSHOT_CACHE
        .lock()
        .expect("readiness snapshot cache")
        .clear();
    MISSION_READY_CACHE
        .lock()
        .expect("mission ready cache")
        .clear();
    DEGRADED_STATE_CACHE
        .lock()
        .expect("degraded state cache")
        .clear();
}

/// Clear cached health statuses (tests only).
#[doc(hidden)]
pub fn reset_health_status_cache_for_tests() {
    reset_platform_event_caches_for_tests();
}

fn health_change_reason(report: &EntityHealthReport) -> String {
    report
        .diagnostics
        .first()
        .map(|diagnostic| diagnostic.message.clone())
        .unwrap_or_else(|| "entity_health_evaluation".into())
}

fn entity_in_degraded_mode(report: &EntityHealthReport) -> bool {
    report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == "critical" || diagnostic.severity == "error")
}

fn take_health_status_transition(entity_id: &str, to: &str) -> Option<String> {
    let mut cache = HEALTH_STATUS_CACHE.lock().expect("health status cache");
    match cache.get(entity_id) {
        Some(previous) if previous == to => None,
        Some(previous) => {
            let from = previous.clone();
            cache.insert(entity_id.to_string(), to.to_string());
            Some(from)
        }
        None => {
            cache.insert(entity_id.to_string(), to.to_string());
            Some("unknown".into())
        }
    }
}

fn take_readiness_snapshot_transition(
    entity_id: &str,
    report: &EntityReadinessReport,
) -> Option<ReadinessSnapshot> {
    let current = ReadinessSnapshot {
        readiness_status: report.readiness_status.clone(),
        mission_ready: report.mission_ready,
        score: report.score,
    };
    let mut cache = READINESS_SNAPSHOT_CACHE
        .lock()
        .expect("readiness snapshot cache");
    match cache.get(entity_id) {
        Some(previous) if previous == &current => None,
        Some(previous) => {
            let from = previous.clone();
            cache.insert(entity_id.to_string(), current);
            Some(from)
        }
        None => {
            cache.insert(entity_id.to_string(), current);
            Some(ReadinessSnapshot {
                readiness_status: "unknown".into(),
                mission_ready: false,
                score: None,
            })
        }
    }
}

fn take_readiness_gate_failed_transition(entity_id: &str, mission_ready: bool) -> bool {
    if mission_ready {
        let mut cache = MISSION_READY_CACHE.lock().expect("mission ready cache");
        cache.insert(entity_id.to_string(), true);
        return false;
    }
    let mut cache = MISSION_READY_CACHE.lock().expect("mission ready cache");
    match cache.get(entity_id) {
        Some(true) => {
            cache.insert(entity_id.to_string(), false);
            true
        }
        Some(false) => false,
        None => {
            cache.insert(entity_id.to_string(), false);
            true
        }
    }
}

fn take_degraded_mode_entry(entity_id: &str, in_degraded: bool) -> bool {
    let mut cache = DEGRADED_STATE_CACHE.lock().expect("degraded state cache");
    match cache.get(entity_id) {
        Some(previous) if *previous == in_degraded => false,
        Some(_) => {
            cache.insert(entity_id.to_string(), in_degraded);
            in_degraded
        }
        None => {
            cache.insert(entity_id.to_string(), in_degraded);
            in_degraded
        }
    }
}

/// Record readiness platform events for an entity readiness report.
pub fn record_readiness_platform_event(audit: &mut AuditRuntime, report: &EntityReadinessReport) {
    if let Some(from) = take_readiness_snapshot_transition(&report.entity_id, report) {
        let event = PlatformEvent::new(
            names::READINESS_CHANGED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "from": {
                    "readiness_status": from.readiness_status,
                    "mission_ready": from.mission_ready,
                    "score": from.score,
                },
                "to": {
                    "readiness_status": report.readiness_status,
                    "mission_ready": report.mission_ready,
                    "score": report.score,
                },
                "issue_count": report.issues.len(),
                "sources": report.sources,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(Some(audit), &event);
    }

    if take_readiness_gate_failed_transition(&report.entity_id, report.mission_ready) {
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

/// Record health platform events for an entity health report.
pub fn record_entity_health_platform_events(report: &EntityHealthReport) {
    if let Some(from) = take_health_status_transition(&report.entity_id, &report.health_status) {
        let reason = health_change_reason(report);
        let event = PlatformEvent::new(
            names::HEALTH_CHANGED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "from": from,
                "to": report.health_status,
                "reason": reason,
                "diagnostic_count": report.diagnostics.len(),
                "sources": report.sources,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(None, &event);
    }

    if report.metrics.health_checks_failed > 0 {
        let failed_event = PlatformEvent::new(
            names::HEALTH_CHECK_FAILED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "failed_checks": report.metrics.health_checks_failed,
                "passed_checks": report.metrics.health_checks_passed,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(None, &failed_event);
    }

    if take_degraded_mode_entry(&report.entity_id, entity_in_degraded_mode(report)) {
        let degraded_event = PlatformEvent::new(
            names::DEGRADED_MODE_ENTERED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "trigger": "entity_health_evaluation",
                "diagnostics": report.diagnostics,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(None, &degraded_event);
    }
}
