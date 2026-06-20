//! Source formatter for Spanda programs.
//!
//! Applies AST-aware pretty printing when parsing succeeds; falls back to
//! whitespace normalization (trim trailing spaces, ensure final newline).

use crate::error::SpandaError;
use crate::pretty::pretty_print_program;

pub fn format_source(source: &str) -> String {
    // Format Spanda source, using the AST formatter when possible.
    //
    // Parameters:
    //
    // - `source` — Raw program text.
    //
    // Returns:
    //
    // Formatted source string. On parse failure, returns whitespace-normalized input
    // (never panics or returns an error).
    //
    // Example:
    //
    // use spanda_core::format::format_source;
    // let input = "module m;\nexport fn f(x:Int)->Int{return x;}\n";
    // let out = format_source(input);

    // assert!(out.contains("export fn f(x: Int) -> Int"));
    match format_ast(source) {
        Ok(formatted) => formatted,
        Err(_) => normalize_whitespace(source),
    }
}

pub fn format_ast(source: &str) -> Result<String, SpandaError> {
    // Format Spanda source via parse + pretty print (strict mode).
    //
    // Parameters:
    //
    // - `source` — Program text that must parse successfully.
    //
    // Returns:
    //
    // Pretty-printed source, or [`SpandaError`] if lexing or parsing fails.
    //
    // Example:
    //
    // use spanda_core::format::format_ast;
    // let input = "module m;\nexport fn f() -> Int { return 0; }\n";
    // let out = format_ast(input).unwrap();

    // assert!(out.contains("export fn f()"));
    let tokens = crate::lexer::tokenize(source)?;
    let program = crate::parser::parse(tokens)?;
    Ok(pretty_print_program(source, &program))
}

fn normalize_whitespace(source: &str) -> String {
    // Normalize whitespace.
    //
    // Parameters:
    // - `source` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::format::normalize_whitespace(source);

    // Start the generated output buffer.
    let mut out = String::new();

    // Handle each input line.
    for line in source.lines() {
        out.push_str(line.trim_end());
        out.push('\n');
    }

    // Repeat while out.ends with("\n\n").
    while out.ends_with("\n\n") {
        out.pop();
    }

    // Take the branch when ends with is false.
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_trailing_whitespace_and_adds_final_newline() {
        // Trims trailing whitespace and adds final newline.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::format::trims_trailing_whitespace_and_adds_final_newline();

        let input = "robot R {  \n  actuator wheels: DifferentialDrive; \n}\n\n";
        let formatted = format_source(input);
        assert!(formatted.ends_with('\n'));
        assert!(!formatted.contains("  \n"));
    }

    #[test]
    fn ast_format_normalizes_module_function() {
        // Ast format normalizes module function.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::format::ast_format_normalizes_module_function();

        let input = "module m;\nexport fn f(x:Int)->Int{return x;}\n";
        let formatted = format_source(input);
        assert!(formatted.contains("export fn f(x: Int) -> Int"));
    }
}
