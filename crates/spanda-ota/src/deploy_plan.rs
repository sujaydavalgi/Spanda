//! Deploy plan builder with embedded certification proof summary.

use spanda_ast::nodes::Program;
use spanda_certify::build_certification_proof_summary;

use crate::plan::build_deploy_plan_from_program;
use crate::CertificationProofSummary;
use crate::DeployPlan;

/// Build a deployment plan with certification proof metadata attached.
pub fn build_deploy_plan(program: &Program, program_path: &str, version: &str) -> DeployPlan {
    let mut plan = build_deploy_plan_from_program(program, program_path, version);
    let proof = build_certification_proof_summary(program, program_path);
    plan.certification_proof = Some(CertificationProofSummary {
        passed: proof.passed,
        passed_strict: proof.passed_strict,
        summary: proof.summary,
        error_count: proof.error_count,
        warning_count: proof.warning_count,
    });
    plan
}
