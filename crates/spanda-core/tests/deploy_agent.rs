//! Deploy agent certification enforcement tests.

use spanda_core::{
    agent_entry_for_port, agent_rollout, build_deploy_bundle, build_deploy_plan, compile,
    spawn_test_agent_with_options,
};
use std::thread;
use std::time::Duration;

#[test]
fn agent_rejects_rollout_without_strict_proof_when_required() {
    let target = "RoverProgram@JetsonOrin".to_string();
    let (port, _handle) =
        spawn_test_agent_with_options(&target, None, true).expect("spawn test agent");
    thread::sleep(Duration::from_millis(50));
    let entry = agent_entry_for_port(&target, port, None);

    let source = include_str!("../../../examples/robotics/ota_deployment.sd");
    let program = compile(source).expect("compile").program;
    let plan = build_deploy_plan(&program, "ota_deployment.sd", "2.0.0");
    let bundle = build_deploy_bundle(&plan);
    let rollout = agent_rollout(&entry, &bundle, plan.certification_proof.as_ref());
    assert!(rollout.is_err(), "agent should reject uncertified rollout");
}

#[test]
fn agent_accepts_rollout_with_strict_proof_when_required() {
    let target = "CertifiedRover@IndustrialController".to_string();
    let (port, _handle) =
        spawn_test_agent_with_options(&target, None, true).expect("spawn test agent");
    thread::sleep(Duration::from_millis(50));
    let entry = agent_entry_for_port(&target, port, None);

    let source = include_str!("../../../examples/robotics/certified_deployment.sd");
    let program = compile(source).expect("compile").program;
    let plan = build_deploy_plan(&program, "certified_deployment.sd", "1.0.0");
    let proof = plan.certification_proof.as_ref().expect("proof summary");
    assert!(proof.passed_strict);
    let bundle = build_deploy_bundle(&plan);
    let rollout = agent_rollout(&entry, &bundle, Some(proof)).expect("request");
    assert!(rollout.ok);
    assert_eq!(rollout.version, "1.0.0");
}
