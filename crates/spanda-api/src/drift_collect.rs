//! Collect program and agent drift findings for operational drift API.
//!
use spanda_ast::nodes::Program;
use spanda_config::{
    detect_agent_drift, expected_agent_states, AgentDriftSnapshot, DriftFinding,
    ResolvedSystemConfig,
};
use spanda_fleet::{
    default_fleet_agents_path, fleet_agent_status, load_fleet_agent_registry, lookup_fleet_agent,
    FleetAgentStatusResponse,
};
use spanda_ota::{
    agent_status, default_agents_path, hash_program_artifact, load_agent_registry, lookup_agent,
    AgentStatusResponse,
};
use std::path::Path;

fn snapshot_from_deploy_status(id: &str, status: &AgentStatusResponse) -> AgentDriftSnapshot {
    AgentDriftSnapshot {
        agent_id: id.to_string(),
        target: Some(status.target.clone()),
        robot_name: status.robot_name.clone(),
        hardware_profile: status.hardware_profile.clone(),
        firmware_version: status.firmware_version.clone(),
        program_hash: status.program_hash.clone(),
        current_version: Some(status.current_version.clone()),
        packages: status.packages.clone(),
        healthy: status.healthy,
        attestation_contract: status.attestation_contract.clone(),
        attestation_verified: status.attestation_verified,
        boot_state: status.boot_state.clone(),
    }
}

fn snapshot_from_fleet_status(id: &str, status: &FleetAgentStatusResponse) -> AgentDriftSnapshot {
    AgentDriftSnapshot {
        agent_id: id.to_string(),
        target: None,
        robot_name: status.robot_name.clone(),
        hardware_profile: status.hardware_profile.clone(),
        firmware_version: status.firmware_version.clone(),
        program_hash: status.program_hash.clone(),
        current_version: None,
        packages: status.packages.clone(),
        healthy: status.healthy,
        ..AgentDriftSnapshot::default()
    }
}

/// Gather live agent drift findings for fleet and deploy agents.
pub fn collect_agent_drift_findings(
    program: &Program,
    current: &ResolvedSystemConfig,
    program_path: Option<&Path>,
) -> Vec<DriftFinding> {
    // Compare registered agents against expected deploy posture.
    //
    // Parameters:
    // - `program` — deployed program
    // - `current` — resolved configuration
    // - `program_path` — optional program file for hash computation
    //
    // Returns:
    // Agent drift findings across firmware, package, and capability dimensions.
    //
    // Options:
    // None.
    //
    // Example:
    // let findings = collect_agent_drift_findings(&program, &current, Some(path));

    let program_hash = program_path
        .and_then(|path| path.to_str())
        .and_then(hash_program_artifact);
    let expected_states = expected_agent_states(program, Some(current), program_hash.as_deref());
    let deploy_registry = load_agent_registry(&default_agents_path());
    let fleet_registry = load_fleet_agent_registry(&default_fleet_agents_path());
    let mut findings = Vec::new();
    for expected in expected_states {
        let snapshot = if let Some(entry) = lookup_agent(&deploy_registry, &expected.target_key) {
            agent_status(entry)
                .ok()
                .map(|status| snapshot_from_deploy_status(&expected.target_key, &status))
        } else if let Some(entry) = lookup_fleet_agent(&fleet_registry, &expected.robot_name) {
            fleet_agent_status(entry)
                .ok()
                .map(|status| snapshot_from_fleet_status(&expected.robot_name, &status))
        } else {
            None
        };
        if let Some(snapshot) = snapshot {
            findings.extend(detect_agent_drift(&expected, &snapshot));
        }
    }
    findings
}
