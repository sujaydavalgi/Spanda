//! Architecture decision record generation from Spanda program structure.
//!
pub mod generate;

pub use generate::{format_adr_report, generate_adrs, AdrFormat, AdrRecord, AdrReport};
