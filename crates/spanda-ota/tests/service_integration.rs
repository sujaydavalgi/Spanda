//! OTA deploy service integration tests.

use spanda_driver::{build_deploy_plan, check, compile};
use spanda_ota::{
    apply_rollout, plan_rollout, validate_rollout_certification, DeployState, RolloutOptions,
    RolloutStrategy,
};

#[test]
fn ota_canary_rollout_from_example() {
    let source = include_str!("../../../examples/robotics/ota_deployment.sd");
    check(source).expect("ota example should type-check");
    let program = compile(source).expect("compile").program;
    let plan = build_deploy_plan(&program, "ota_deployment.sd", "1.0.0");
    assert!(!plan.assignments.is_empty());
    let result = plan_rollout(
        &plan,
        &RolloutOptions {
            strategy: RolloutStrategy::All,
            version: "1.0.0".into(),
            dry_run: false,
            ..Default::default()
        },
    );
    assert!(result.success);
    let canary = plan_rollout(
        &plan,
        &RolloutOptions {
            strategy: RolloutStrategy::Canary,
            canary_percent: 50,
            version: "1.1.0".into(),
            ..Default::default()
        },
    );
    assert!(canary.success);
}

#[test]
fn ota_apply_rollout_updates_state() {
    let source = include_str!("../../../examples/robotics/ota_deployment.sd");
    let program = compile(source).expect("compile").program;
    let plan = build_deploy_plan(&program, "ota.sd", "2.0.0");
    let mut state = DeployState::default();
    let rollout = plan_rollout(
        &plan,
        &RolloutOptions {
            strategy: RolloutStrategy::All,
            version: "2.0.0".into(),
            ..Default::default()
        },
    );
    apply_rollout(&mut state, &rollout);
    assert!(!state.current_version.is_empty());
}

#[test]
fn require_certify_blocks_uncertified_rollout() {
    let source = include_str!("../../../examples/robotics/ota_deployment.sd");
    let program = compile(source).expect("compile").program;
    let plan = build_deploy_plan(&program, "ota_deployment.sd", "1.0.0");
    let options = RolloutOptions {
        require_certify: true,
        ..Default::default()
    };
    assert!(validate_rollout_certification(&plan, &options).is_err());
    let result = plan_rollout(&plan, &options);
    assert!(!result.success);
    assert!(result.steps.is_empty());
}

#[test]
fn require_certify_allows_certified_program() {
    let source = include_str!("../../../examples/robotics/certified_deployment.sd");
    let program = compile(source).expect("compile").program;
    let plan = build_deploy_plan(&program, "certified_deployment.sd", "1.0.0");
    let options = RolloutOptions {
        require_certify: true,
        ..Default::default()
    };
    validate_rollout_certification(&plan, &options).expect("certified program should pass");
    let result = plan_rollout(&plan, &options);
    assert!(result.success);
    assert!(plan.certification_proof.as_ref().unwrap().passed_strict);
}
