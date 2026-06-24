//! Spanda documentation generators — program API docs and language reference.
//!
mod builtin_methods;
pub mod html_docs;
pub mod language_reference;
mod man_pages;
mod program_docs;

pub use html_docs::markdown_to_html;
pub use language_reference::{generate_cli_man_pages, generate_language_reference, CLI_COMMAND_NAMES};
pub use man_pages::{list_man_pages, lookup_man_page, markdown_man_to_roff};
pub use program_docs::{
    generate_docs_for_path, generate_html, generate_json_docs, generate_markdown, DocBatchResult,
};
