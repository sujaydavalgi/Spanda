//! Platform event emission for OTA rollouts.
//!
use serde_json::json;
use spanda_audit::platform_event::names;
use spanda_audit::PlatformEvent;
use spanda_runtime::publish_platform_event;

use crate::types::{DeployPlan, RolloutOptions, RolloutResult, RolloutStrategy};

fn fleet_scope(plan: &DeployPlan) -> String {
    format!("ota/{}", plan.program)
}

/// Record `OtaRolloutStarted` when a rollout plan is accepted.
pub fn record_ota_rollout_started(
    plan: &DeployPlan,
    options: &RolloutOptions,
    target_count: usize,
) {
    let event = PlatformEvent::new(
        names::OTA_ROLLOUT_STARTED,
        "spanda-ota",
        json!({
            "fleet_id": fleet_scope(plan),
            "artifact": plan.version,
            "targets": target_count,
            "strategy": rollout_strategy_label(options.strategy),
            "dry_run": options.dry_run,
        }),
    )
    .with_entity_id(fleet_scope(plan));
    publish_platform_event(None, &event);
}

/// Record `OtaRolloutCompleted` when rollout planning or execution finishes.
pub fn record_ota_rollout_completed(plan: &DeployPlan, result: &RolloutResult) {
    let deployed = result
        .steps
        .iter()
        .filter(|s| {
            matches!(
                s.status,
                crate::types::RolloutStepStatus::Deployed
                    | crate::types::RolloutStepStatus::Pending
            )
        })
        .count();
    let event = PlatformEvent::new(
        names::OTA_ROLLOUT_COMPLETED,
        "spanda-ota",
        json!({
            "fleet_id": fleet_scope(plan),
            "artifact": result.version,
            "status": if result.success { "success" } else { "failed" },
            "deployed_targets": deployed,
            "step_count": result.steps.len(),
            "dry_run": result.dry_run,
        }),
    )
    .with_entity_id(fleet_scope(plan));
    publish_platform_event(None, &event);
}

fn rollout_strategy_label(strategy: RolloutStrategy) -> &'static str {
    match strategy {
        RolloutStrategy::All => "all",
        RolloutStrategy::Canary => "canary",
        RolloutStrategy::Staged => "staged",
        RolloutStrategy::BlueGreen => "blue_green",
    }
}
