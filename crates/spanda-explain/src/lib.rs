//! Explainability reports for Spanda programs, verification, and traces.

mod explain;
mod report;

pub use explain::{
    explain_decision_trace, explain_program, explain_program_with_options, explain_readiness,
    explain_safety, explain_trace, explain_verify, ExplainProgramOptions,
};
pub use report::{format_explain_report, ExplainReport, ExplainSection};
