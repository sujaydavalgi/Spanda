//! Operational policy parsing and verify-time evaluation.
//!
pub mod evaluate;

pub use evaluate::{
    evaluate_policy, format_policy_report, list_policies, PolicyEvaluationReport, PolicySeverity,
    PolicyViolation,
};
