//! High-level run helpers from source through compile and interpreter.
//!
use spanda_ast::nodes::Program;
#[cfg(feature = "bridge")]
use spanda_bridge::default_ffi_registry;
use spanda_error::SpandaError;
use spanda_interpreter::{run_program as interpreter_run_program, RunOptions, RunResult};
use spanda_typecheck::ModuleRegistry;

use crate::compile::{compile, compile_with_registry};

/// Compile, certify (when enabled), and execute Spanda source.
pub fn run(source: &str, options: RunOptions) -> Result<RunResult, SpandaError> {
    // Description:
    //     Run.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     options: RunOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<RunResult, SpandaError>
    //         Return value from `run`.
    //
    // Example:

    //     let result = spanda_driver::run::run(source, options);

    let program = if let Some(registry) = &options.module_registry {
        compile_with_registry(source, registry)?.program
    } else {
        compile(source)?.program
    };
    run_program(&program, options)
}

/// Apply certify and FFI defaults, then execute a type-checked program.
pub fn run_program(program: &Program, options: RunOptions) -> Result<RunResult, SpandaError> {
    // Description:
    //     Run program.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     options: RunOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<RunResult, SpandaError>
    //         Return value from `run_program`.
    //
    // Example:

    //     let result = spanda_driver::run::run_program(progra, options);

    let mut options = options;
    // Wire default FFI bridges and certification gate before interpreter execution.
    //
    // Parameters:
    // - `program` — type-checked AST
    // - `options` — run options
    //
    // Returns:
    // Interpreter run result, or a certify/runtime error.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = run_program(&program, RunOptions::default())?;

    #[cfg(feature = "bridge")]
    if options.ffi_registry.is_none() {
        options.ffi_registry = Some(default_ffi_registry());
    }
    interpreter_run_program(program, options)
}

/// Type-check and run embedded module tests from source.
pub fn run_tests_with_registry(
    source: &str,
    registry: &ModuleRegistry,
) -> Result<spanda_interpreter::TestRunResult, SpandaError> {
    // Description:
    //     Run tests with registry.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     registry: &ModuleRegistry
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: Result<spanda_interpreter::TestRunResult, SpandaError>
    //         Return value from `run_tests_with_registry`.
    //
    // Example:

    //     let result = spanda_driver::run::run_tests_with_registry(source, registry);

    let program = compile_with_registry(source, registry)?.program;
    spanda_interpreter::run_tests_with_registry(&program, registry)
}
