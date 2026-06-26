//! OTA rollout planning, state tracking, and certification gates.
//!
use crate::types::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Block rollout when `--require-certify` is set and strict proof failed.
pub fn validate_rollout_certification(
    plan: &DeployPlan,
    options: &RolloutOptions,
) -> Result<(), String> {
    // Description:
    //     Validate rollout certification.
    //
    // Inputs:
    //     plan: &DeployPlan
    //         Caller-supplied plan.
    //     options: &RolloutOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<(), String>
    //         Return value from `validate_rollout_certification`.
    //
    // Example:

    //     let result = spanda_ota::service::validate_rollout_certification(plan, options);

    if !options.require_certify {
        return Ok(());
    }
    let Some(proof) = &plan.certification_proof else {
        return Err("Deploy plan missing certification proof summary".into());
    };
    if !proof.passed_strict {
        return Err(format!(
            "Deploy blocked — strict certification proof failed: {}",
            proof.summary
        ));
    }
    Ok(())
}

fn assignment_key(robot: &str, hardware: &str) -> String {
    // Description:
    //     Assignment key.
    //
    // Inputs:
    //     robo: &str
    //         Caller-supplied robo.
    //     hardware: &str
    //         Caller-supplied hardware.
    //
    // Outputs:
    //     result: String
    //         Return value from `assignment_key`.
    //
    // Example:

    //     let result = spanda_ota::service::assignment_key(robo, hardware);

    format!("{robot}@{hardware}")
}

/// Stable deploy target key for robot/hardware pairs (`Robot@Hardware`).
pub fn deploy_target_key(robot: &str, hardware: &str) -> String {
    // Description:
    //     Deploy target key.
    //
    // Inputs:
    //     robo: &str
    //         Caller-supplied robo.
    //     hardware: &str
    //         Caller-supplied hardware.
    //
    // Outputs:
    //     result: String
    //         Return value from `deploy_target_key`.
    //
    // Example:

    //     let result = spanda_ota::service::deploy_target_key(robo, hardware);

    assignment_key(robot, hardware)
}

/// Compute a SHA-256 hex digest of a program artifact on disk.
pub fn hash_program_artifact(program_path: &str) -> Option<String> {
    // Description:
    //     Hash program artifact.
    //
    // Inputs:
    //     program_path: &str
    //         Caller-supplied program path.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `hash_program_artifact`.
    //
    // Example:

    //     let result = spanda_ota::service::hash_program_artifact(program_path);

    let path = Path::new(program_path);
    if !path.exists() {
        return None;
    }
    let bytes = fs::read(path).ok()?;
    use sha2::{Digest, Sha256};
    Some(hex::encode(Sha256::digest(bytes)))
}

/// Plan which targets receive an update under the chosen rollout strategy.
pub fn plan_rollout(plan: &DeployPlan, options: &RolloutOptions) -> RolloutResult {
    // Description:
    //     Plan rollout.
    //
    // Inputs:
    //     plan: &DeployPlan
    //         Caller-supplied plan.
    //     options: &RolloutOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: RolloutResult
    //         Return value from `plan_rollout`.
    //
    // Example:
    //     let result = spanda_ota::service::plan_rollout(plan, options);
    if validate_rollout_certification(plan, options).is_err() {
        return RolloutResult {
            strategy: options.strategy,
            version: options.version.clone(),
            dry_run: options.dry_run,
            steps: vec![],
            success: false,
        };
    }
    let total = plan.assignments.len();
    let mut steps = Vec::new();

    if total == 0 {
        return RolloutResult {
            strategy: options.strategy,
            version: options.version.clone(),
            dry_run: options.dry_run,
            steps,
            success: true,
        };
    }

    match options.strategy {
        RolloutStrategy::All => {
            for assignment in &plan.assignments {
                steps.push(RolloutStep {
                    robot_name: assignment.robot_name.clone(),
                    hardware: assignment.hardware.clone(),
                    status: if options.dry_run {
                        RolloutStepStatus::Pending
                    } else {
                        RolloutStepStatus::Deployed
                    },
                    version: options.version.clone(),
                    phase_percent: Some(100),
                });
            }
        }
        RolloutStrategy::Canary => {
            let pct = options.canary_percent.clamp(1, 100);
            let canary_count = ((total as f64 * pct as f64 / 100.0).ceil() as usize).max(1);
            for (idx, assignment) in plan.assignments.iter().enumerate() {
                let deploy = idx < canary_count;
                steps.push(RolloutStep {
                    robot_name: assignment.robot_name.clone(),
                    hardware: assignment.hardware.clone(),
                    status: if deploy {
                        if options.dry_run {
                            RolloutStepStatus::Pending
                        } else {
                            RolloutStepStatus::Deployed
                        }
                    } else {
                        RolloutStepStatus::Skipped
                    },
                    version: options.version.clone(),
                    phase_percent: Some(if deploy { pct } else { 0 }),
                });
            }
        }
        RolloutStrategy::Staged => {
            let phases = if options.staged_phases.is_empty() {
                vec![100]
            } else {
                options.staged_phases.clone()
            };
            let final_phase = *phases.last().unwrap_or(&100);
            let deploy_count = ((total as f64 * final_phase as f64 / 100.0).ceil() as usize).max(1);
            for (idx, assignment) in plan.assignments.iter().enumerate() {
                let deploy = idx < deploy_count;
                steps.push(RolloutStep {
                    robot_name: assignment.robot_name.clone(),
                    hardware: assignment.hardware.clone(),
                    status: if deploy {
                        if options.dry_run {
                            RolloutStepStatus::Pending
                        } else {
                            RolloutStepStatus::Deployed
                        }
                    } else {
                        RolloutStepStatus::Skipped
                    },
                    version: options.version.clone(),
                    phase_percent: Some(final_phase),
                });
            }
        }
        RolloutStrategy::BlueGreen => {
            let deploy_count = (total / 2).max(1);
            for (idx, assignment) in plan.assignments.iter().enumerate() {
                let deploy = idx < deploy_count;
                steps.push(RolloutStep {
                    robot_name: assignment.robot_name.clone(),
                    hardware: assignment.hardware.clone(),
                    status: if deploy {
                        if options.dry_run {
                            RolloutStepStatus::Pending
                        } else {
                            RolloutStepStatus::Deployed
                        }
                    } else {
                        RolloutStepStatus::Skipped
                    },
                    version: options.version.clone(),
                    phase_percent: Some(if deploy { 50 } else { 0 }),
                });
            }
        }
    }

    RolloutResult {
        strategy: options.strategy,
        version: options.version.clone(),
        dry_run: options.dry_run,
        success: !steps.iter().any(|s| s.status == RolloutStepStatus::Failed),
        steps,
    }
}

/// Apply a successful rollout to persistent deploy state.
pub fn apply_rollout(state: &mut DeployState, result: &RolloutResult) {
    // Description:
    //     Apply rollout.
    //
    // Inputs:
    //     state: &mut DeployState
    //         Caller-supplied state.
    //     resul: &RolloutResult
    //         Caller-supplied resul.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_ota::service::apply_rollout(state, resul);
    if result.dry_run {
        return;
    }
    for step in &result.steps {
        if step.status != RolloutStepStatus::Deployed {
            continue;
        }
        let key = assignment_key(&step.robot_name, &step.hardware);
        if let Some(prev) = state.current_version.get(&key) {
            state.previous_version.insert(key.clone(), prev.clone());
        }
        state.current_version.insert(key, step.version.clone());
    }
    state.history.push(result.clone());
}

/// Roll back deployed targets to the previous recorded version.
pub fn rollback_targets(
    state: &mut DeployState,
    plan: &DeployPlan,
    to_previous: bool,
) -> RolloutResult {
    // Description:
    //     Rollback targets.
    //
    // Inputs:
    //     state: &mut DeployState
    //         Caller-supplied state.
    //     plan: &DeployPlan
    //         Caller-supplied plan.
    //     o_previous: bool
    //         Caller-supplied o previous.
    //
    // Outputs:
    //     result: RolloutResult
    //         Return value from `rollback_targets`.
    //
    // Example:
    //     let result = spanda_ota::service::rollback_targets(state, plan, o_previous);
    let mut steps = Vec::new();
    for assignment in &plan.assignments {
        let key = assignment_key(&assignment.robot_name, &assignment.hardware);
        let target_version = if to_previous {
            state.previous_version.get(&key).cloned()
        } else {
            state.current_version.get(&key).cloned()
        };
        let (status, version) = match target_version {
            Some(v) => (RolloutStepStatus::RolledBack, v),
            None => (RolloutStepStatus::Skipped, "unknown".into()),
        };
        if status == RolloutStepStatus::RolledBack {
            if let Some(cur) = state.current_version.get(&key) {
                state.previous_version.insert(key.clone(), cur.clone());
            }
            state.current_version.insert(key, version.clone());
        }
        steps.push(RolloutStep {
            robot_name: assignment.robot_name.clone(),
            hardware: assignment.hardware.clone(),
            status,
            version,
            phase_percent: None,
        });
    }
    let result = RolloutResult {
        strategy: RolloutStrategy::All,
        version: "rollback".into(),
        dry_run: false,
        success: steps
            .iter()
            .any(|s| s.status == RolloutStepStatus::RolledBack),
        steps,
    };
    state.history.push(result.clone());
    result
}

/// Default path for OTA state under `.spanda/deploy-state.json`.
pub fn default_state_path() -> PathBuf {
    // Description:
    //     Default state path.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: PathBuf
    //         Return value from `default_state_path`.
    //
    // Example:

    //     let result = spanda_ota::service::default_state_path();

    PathBuf::from(".spanda/deploy-state.json")
}

/// Load deploy state from disk, or return default when missing.
pub fn load_deploy_state(path: &Path) -> DeployState {
    // Description:
    //     Load deploy state.
    //
    // Inputs:
    //     path: &Path
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: DeployState
    //         Return value from `load_deploy_state`.
    //
    // Example:

    //     let result = spanda_ota::service::load_deploy_state(path);

    if !path.exists() {
        return DeployState::default();
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// Persist deploy state to disk.
pub fn save_deploy_state(path: &Path, state: &DeployState) -> Result<(), String> {
    // Description:
    //     Save deploy state.
    //
    // Inputs:
    //     path: &Path
    //         Caller-supplied path.
    //     state: &DeployState
    //         Caller-supplied state.
    //
    // Outputs:
    //     result: Result<(), String>
    //         Return value from `save_deploy_state`.
    //
    // Example:

    //     let result = spanda_ota::service::save_deploy_state(path, state);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())
}
