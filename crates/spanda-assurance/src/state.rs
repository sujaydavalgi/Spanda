//! State estimation analysis from state_estimator declarations.
//!
use crate::types::{BeliefState, Confidence, SensorFusionState, StateEstimate};
use spanda_ast::assurance_decl::StateEstimatorDecl;
use spanda_ast::nodes::Program;
use spanda_runtime::fusion::preview_fusion_inputs;

/// State estimation assurance report.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StateAssuranceReport {
    pub estimators: Vec<SensorFusionState>,
    pub belief: BeliefState,
    pub issues: Vec<String>,
    pub passed: bool,
}

/// Evaluate state estimator declarations and synthesize belief state.
pub fn evaluate_state_assurance(program: &Program) -> StateAssuranceReport {
    let estimators = extract_sensor_fusion(program);
    let belief = build_belief_state(program);
    let issues = validate_state_estimators(program);
    StateAssuranceReport {
        estimators,
        belief,
        issues: issues.clone(),
        passed: issues.is_empty(),
    }
}

/// Extract state estimators and build weighted fusion previews.
pub fn extract_sensor_fusion(program: &Program) -> Vec<SensorFusionState> {
    let Program::Program {
        state_estimators, ..
    } = program;
    state_estimators
        .iter()
        .map(|decl| {
            let StateEstimatorDecl::StateEstimatorDecl {
                name,
                inputs,
                output_type,
                ..
            } = decl;
            let preview = preview_fusion_inputs(program, inputs, output_type);
            SensorFusionState {
                estimator: name.clone(),
                inputs: inputs.clone(),
                fused: Some(StateEstimate {
                    name: output_type.clone(),
                    value: preview.summary,
                    confidence: Confidence(preview.confidence),
                    sources: preview.sources,
                }),
            }
        })
        .collect()
}

/// Aggregate estimates into a belief state.
pub fn build_belief_state(program: &Program) -> BeliefState {
    let fusion = extract_sensor_fusion(program);
    BeliefState {
        estimates: fusion.into_iter().filter_map(|f| f.fused).collect(),
    }
}

/// Validate state estimator declarations.
pub fn validate_state_estimators(program: &Program) -> Vec<String> {
    let mut issues = Vec::new();
    let Program::Program {
        state_estimators, ..
    } = program;
    for decl in state_estimators {
        let StateEstimatorDecl::StateEstimatorDecl { name, inputs, .. } = decl;
        if inputs.is_empty() {
            issues.push(format!("State estimator '{name}' has no inputs"));
        }
    }
    issues
}
