//! Hardware and certification compatibility verification.
//!
use spanda_error::SpandaError;
use spanda_hardware::{
    verify_program_compatibility, CompatSeverity, CompatibilityReport, VerifyOptions,
};

use crate::compile::{compile, compile_with_registry};
use spanda_typecheck::ModuleRegistry;

pub fn verify_compatibility(
    source: &str,
    options: &VerifyOptions,
) -> Result<CompatibilityReport, SpandaError> {
    // Description:
    //     Verify compatibility.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     options: &VerifyOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<CompatibilityReport, SpandaError>
    //         Return value from `verify_compatibility`.
    //
    // Example:

    //     let result = spanda_driver::verify::verify_compatibility(source, options);

    verify_compatibility_with_registry(source, options, None)
}

pub fn verify_compatibility_with_registry(
    source: &str,
    options: &VerifyOptions,
    registry: Option<&ModuleRegistry>,
) -> Result<CompatibilityReport, SpandaError> {
    // Description:
    //     Verify compatibility with registry.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     options: &VerifyOptions
    //         Caller-supplied options.
    //     registry: Option<&ModuleRegistry>
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: Result<CompatibilityReport, SpandaError>
    //         Return value from `verify_compatibility_with_registry`.
    //
    // Example:

    //     let result = spanda_driver::verify::verify_compatibility_with_registry(source, options, registry);

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
    // Description:
    //     Verify compatibility target.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     arge: Option<&str>
    //         Caller-supplied arge.
    //
    // Outputs:
    //     result: Result<CompatibilityReport, SpandaError>
    //         Return value from `verify_compatibility_target`.
    //
    // Example:

    //     let result = spanda_driver::verify::verify_compatibility_target(source, arge);

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
