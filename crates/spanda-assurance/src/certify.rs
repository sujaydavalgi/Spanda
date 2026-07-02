//! Deploy certification runtime gate for mission execution.
//!
use spanda_ast::nodes::Program;
use spanda_certify::{certification_runtime_enabled_from_env, enforce_certification_runtime};
use spanda_error::SpandaError;

/// Enforce deploy certification proof before interpreter execution when requested.
pub fn enforce_runtime_certification(program: &Program, enforce: bool) -> Result<(), SpandaError> {
    if enforce || certification_runtime_enabled_from_env() {
        enforce_certification_runtime(program, true)?;
    }
    Ok(())
}
