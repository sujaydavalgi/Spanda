//! Static dependency graph construction and rendering for Spanda programs.
//!
pub mod build;
pub mod format;

pub use build::{build_dependency_graph, DependencyGraph, GraphEdge, GraphNode, GraphNodeKind};
pub use format::{format_dependency_graph, GraphFormat};
