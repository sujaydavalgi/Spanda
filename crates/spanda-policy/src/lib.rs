//! Operational policy parsing and verify-time evaluation.
//!
pub mod evaluate;
pub mod readiness;
pub mod runtime;

pub use evaluate::{
    evaluate_policy, evaluate_policy_with_options, format_policy_report, list_policies,
    PolicyEvaluationReport, PolicySeverity, PolicyViolation,
};
pub use readiness::{
    apply_policy_to_readiness, evaluate_readiness_with_policy, operational_policy_gate,
};
pub use runtime::{
    build_runtime_policy_monitor, check_runtime_policy_motion, RuntimePolicyMonitor,
    RuntimePolicyViolation,
};
