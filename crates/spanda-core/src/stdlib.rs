//! Standard library namespaces for Spanda domain types.
//!
//! Types are resolved via `type_system::resolve_type_name` and may be
//! referenced with or without the `std.<module>.` prefix.

pub use crate::type_system::std_namespaces;

/// Import paths registered for `import std.time;` style modules.
pub fn resolve_std_import(path: &str) -> bool {
    std_namespaces().contains_key(path)
}
