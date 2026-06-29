//! Hardware compatibility verification for compiled Spanda programs.
//!
use spanda_driver::{compile, compile_with_registry};
use spanda_error::SpandaError;
use spanda_hardware::{
    verify_program_compatibility, CompatSeverity, CompatibilityReport, VerifyOptions,
};
use spanda_typecheck::ModuleRegistry;

pub fn verify_compatibility(
    source: &str,
    options: &VerifyOptions,
) -> Result<CompatibilityReport, SpandaError> {
    verify_compatibility_with_registry(source, options, None)
}

pub fn verify_compatibility_with_registry(
    source: &str,
    options: &VerifyOptions,
    registry: Option<&ModuleRegistry>,
) -> Result<CompatibilityReport, SpandaError> {
    let program = if let Some(registry) = registry {
        compile_with_registry(source, registry)?.program
    } else {
        compile(source)?.program
    };
    let mut report = verify_program_compatibility(&program, options);
    report.compatible = !report
        .items
        .iter()
        .any(|item| item.severity == CompatSeverity::Error);
    Ok(report)
}

pub fn verify_compatibility_target(
    source: &str,
    target: Option<&str>,
) -> Result<CompatibilityReport, SpandaError> {
    verify_compatibility(
        source,
        &VerifyOptions {
            target: target.map(str::to_string),
            all_targets: false,
            simulate: false,
            strict_certify: false,
        },
    )
}
