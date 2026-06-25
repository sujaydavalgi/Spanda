//! Integration tests for deploy agent attestation status fields.

use spanda_ota::{agent_entry_for_port, agent_status, deploy_target_key, spawn_test_agent};
use std::thread;
use std::time::Duration;

#[test]
fn agent_status_includes_attestation_from_environment() {
    std::env::set_var("SPANDA_ATTESTATION_CONTRACT", "trust.jetson");
    std::env::set_var("SPANDA_ATTESTATION_VERIFIED", "1");
    std::env::set_var("SPANDA_BOOT_STATE", "verified");

    let target = deploy_target_key("Rover", "Jetson");
    let (port, _handle) = spawn_test_agent(&target, None).expect("spawn test agent");
    thread::sleep(Duration::from_millis(50));

    let entry = agent_entry_for_port(&target, port, None);
    let status = agent_status(&entry).expect("agent status");
    assert_eq!(status.attestation_contract.as_deref(), Some("trust.jetson"));
    assert_eq!(status.attestation_verified, Some(true));
    assert_eq!(status.boot_state.as_deref(), Some("verified"));

    std::env::remove_var("SPANDA_ATTESTATION_CONTRACT");
    std::env::remove_var("SPANDA_ATTESTATION_VERIFIED");
    std::env::remove_var("SPANDA_BOOT_STATE");
}
