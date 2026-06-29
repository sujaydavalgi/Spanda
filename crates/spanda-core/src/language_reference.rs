//! Language reference shim — delegates to `spanda_docs`.
pub use spanda_docs::{
    generate_cli_man_pages, list_man_pages, lookup_man_page, markdown_man_to_roff, CLI_COMMAND_NAMES,
};
pub fn generate_language_reference() -> String {
    let libs: Vec<_> = spanda_lib_registry::list_libraries().iter().map(|l| (l.id.clone(), format!("{} — {} v{}: {}", l.name, l.vendor, l.version, l.description))).collect();
    spanda_docs::generate_language_reference(spanda_runtime_host::core_type_check_host(), &libs)
}
