//! Type-check host wiring for language reference metadata.
//!
use spanda_typecheck::{self, MethodSig, TypeCheckHost};

/// Return the built-in method map for all types, keyed by type name then method name.
///
/// Parameters:
/// - `host` — type-check host supplying domain-specific validation hooks
///
/// Returns:
/// HashMap of type_name → HashMap of method_name → MethodSig.
///
/// Options:
/// None.
///
/// Example:
/// let methods = BUILTIN_METHODS(host);
#[allow(non_snake_case)]
pub fn BUILTIN_METHODS(
    host: &dyn TypeCheckHost,
) -> std::collections::HashMap<String, std::collections::HashMap<String, MethodSig>> {
    // Delegate to typecheck with the injected host.
    spanda_typecheck::BUILTIN_METHODS(host)
}
