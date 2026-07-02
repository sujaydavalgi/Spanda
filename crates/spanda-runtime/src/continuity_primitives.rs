//! AST-only continuity helpers shared between runtime and assurance layers.
//!
use crate::continuity_types::{
    ContinuityCheckpointStore, ContinuityPolicySpec, ContinuityTrigger, MissionStateSnapshot,
};
use spanda_ast::nodes::Program;
use std::path::Path;

/// Extract continuity policies from program declarations.
pub fn extract_continuity_policies(program: &Program) -> Vec<ContinuityPolicySpec> {
    // Extract continuity policies from program declarations.
    //
    // Parameters:
    // - `program` — parsed `.sd` program
    //
    // Returns:
    // Continuity policy specs from `continuity_policy` blocks.
    //
    // Options:
    // None.
    //
    // Example:
    // let policies = extract_continuity_policies(&program);

    let Program::Program {
        continuity_policies,
        ..
    } = program;

    continuity_policies
        .iter()
        .map(|decl| {
            let spanda_ast::assurance_decl::ContinuityPolicyDecl::ContinuityPolicyDecl {
                name,
                branches,
                ..
            } = decl;
            ContinuityPolicySpec {
                name: name.clone(),
                triggers: branches
                    .iter()
                    .map(|b| (b.condition.clone(), b.actions.clone()))
                    .collect(),
            }
        })
        .collect()
}

/// Map a runtime fault or health event to a continuity trigger.
pub fn issue_to_continuity_trigger(issue: &str) -> Option<ContinuityTrigger> {
    // Map a runtime fault or health event to a continuity trigger.
    //
    // Parameters:
    // - `issue` — runtime fault or health label
    //
    // Returns:
    // Continuity trigger when mappable, otherwise None.
    //
    // Options:
    // None.
    //
    // Example:
    // let trigger = issue_to_continuity_trigger("battery critical");

    let lower = issue.to_ascii_lowercase();
    if lower.contains("swarm") {
        Some(ContinuityTrigger::SwarmMemberLost)
    } else if lower.contains("fleet") {
        Some(ContinuityTrigger::FleetMemberOffline)
    } else if lower.contains("battery") {
        Some(ContinuityTrigger::BatteryCritical)
    } else if lower.contains("disconnect") || lower.contains("offline") || lower.contains("camera")
    {
        Some(ContinuityTrigger::DeviceDisconnected)
    } else if lower.contains("degraded") || lower.contains("gps") {
        Some(ContinuityTrigger::RobotDegraded)
    } else if lower.contains("capability") {
        Some(ContinuityTrigger::HardwareCapabilityLost)
    } else if lower.contains("communication") || lower.contains("comm") {
        Some(ContinuityTrigger::CommunicationInterrupted)
    } else if lower.contains("robot") || lower.contains("critical") || lower.contains("failed") {
        Some(ContinuityTrigger::RobotFailed)
    } else {
        None
    }
}

/// Return true when the program declares `continuity_policy` for the trigger.
pub fn program_has_continuity_for_trigger(program: &Program, trigger: ContinuityTrigger) -> bool {
    // Return true when the program declares continuity policy for the trigger.
    //
    // Parameters:
    // - `program` — parsed `.sd` program
    // - `trigger` — continuity trigger to match
    //
    // Returns:
    // True when a policy branch matches the trigger.
    //
    // Options:
    // None.
    //
    // Example:
    // let covered = program_has_continuity_for_trigger(&program, ContinuityTrigger::RobotFailed);

    let trigger_key = trigger_condition_key(trigger);
    extract_continuity_policies(program).iter().any(|policy| {
        policy
            .triggers
            .iter()
            .any(|(condition, _)| condition_matches_trigger(condition, trigger_key))
    })
}

/// Default checkpoint store under the project `.spanda/` directory.
pub fn default_checkpoint_store_path() -> std::path::PathBuf {
    std::path::PathBuf::from(".spanda/mission-checkpoints.json")
}

/// Load persisted checkpoints from disk.
pub fn load_checkpoint_store(path: &Path) -> ContinuityCheckpointStore {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Persist checkpoint store to disk.
pub fn save_checkpoint_store(
    path: &Path,
    store: &ContinuityCheckpointStore,
) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        path,
        serde_json::to_string_pretty(store).unwrap_or_default(),
    )
}

/// Record a snapshot for a mission/robot pair.
pub fn record_checkpoint(
    store: &mut ContinuityCheckpointStore,
    mission: &str,
    robot: &str,
    snapshot: MissionStateSnapshot,
) {
    store.entries.insert(store_key(mission, robot), snapshot);
}

/// Load a stored snapshot for a mission/robot pair.
pub fn load_checkpoint(
    store: &ContinuityCheckpointStore,
    mission: &str,
    robot: &str,
) -> Option<MissionStateSnapshot> {
    store.entries.get(&store_key(mission, robot)).cloned()
}

/// Parse continuity trigger from CLI string.
pub fn parse_trigger(s: &str) -> ContinuityTrigger {
    match s.to_lowercase().as_str() {
        "robot_degraded" | "degraded" => ContinuityTrigger::RobotDegraded,
        "device_disconnected" | "disconnect" => ContinuityTrigger::DeviceDisconnected,
        "fleet_member_offline" | "fleet_offline" => ContinuityTrigger::FleetMemberOffline,
        "swarm_member_lost" | "swarm_lost" => ContinuityTrigger::SwarmMemberLost,
        "communication_interrupted" | "comm_lost" => ContinuityTrigger::CommunicationInterrupted,
        "battery_critical" | "battery" => ContinuityTrigger::BatteryCritical,
        "hardware_capability_lost" | "capability_lost" => ContinuityTrigger::HardwareCapabilityLost,
        _ => ContinuityTrigger::RobotFailed,
    }
}

fn store_key(mission: &str, robot: &str) -> String {
    format!("{mission}::{robot}")
}

fn trigger_condition_key(trigger: ContinuityTrigger) -> &'static str {
    match trigger {
        ContinuityTrigger::RobotFailed => "robot.failed",
        ContinuityTrigger::RobotDegraded => "robot.degraded",
        ContinuityTrigger::DeviceDisconnected => "device.disconnected",
        ContinuityTrigger::FleetMemberOffline => "fleet.failed",
        ContinuityTrigger::SwarmMemberLost => "swarm.failed",
        ContinuityTrigger::CommunicationInterrupted => "communication.interrupted",
        ContinuityTrigger::BatteryCritical => "battery.critical",
        ContinuityTrigger::HardwareCapabilityLost => "hardware.capability_lost",
    }
}

fn condition_matches_trigger(condition: &str, trigger_key: &str) -> bool {
    let c = condition.to_lowercase();
    let t = trigger_key.to_lowercase();
    c == t || c.contains(&t.replace('.', "")) || t.contains(&c.replace('.', ""))
}
