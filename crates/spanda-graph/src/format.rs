//! Render dependency graphs as JSON, Mermaid, DOT, or text.
//!
use crate::build::{DependencyGraph, GraphNodeKind};
use serde_json;

/// Output format for dependency graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphFormat {
    Json,
    Mermaid,
    Dot,
    Text,
}

impl GraphFormat {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "mermaid" => Self::Mermaid,
            "dot" | "graphviz" => Self::Dot,
            "text" | "txt" => Self::Text,
            _ => Self::Json,
        }
    }
}

/// Format a dependency graph for CLI or CI output.
pub fn format_dependency_graph(graph: &DependencyGraph, format: GraphFormat) -> String {
    // Render a dependency graph in the requested output format.
    //
    // Parameters:
    // - `graph` — built dependency graph
    // - `format` — json, mermaid, dot, or text
    //
    // Returns:
    // Formatted graph string.
    //
    // Options:
    // None.
    //
    // Example:
    // let out = format_dependency_graph(&graph, GraphFormat::Mermaid);

    match format {
        GraphFormat::Json => serde_json::to_string_pretty(graph).unwrap_or_else(|_| "{}".into()),
        GraphFormat::Mermaid => format_mermaid(graph),
        GraphFormat::Dot => format_dot(graph),
        GraphFormat::Text => format_text(graph),
    }
}

fn format_mermaid(graph: &DependencyGraph) -> String {
    let mut out = String::from("flowchart TD\n");
    for node in &graph.nodes {
        out.push_str(&format!(
            "  {}[\"{}\"]\n",
            mermaid_id(&node.id),
            escape_label(&node.label),
        ));
    }
    for edge in &graph.edges {
        out.push_str(&format!(
            "  {} -->|{}| {}\n",
            mermaid_id(&edge.from),
            escape_label(&edge.relation),
            mermaid_id(&edge.to)
        ));
    }
    out
}

fn format_dot(graph: &DependencyGraph) -> String {
    let mut out = String::from("digraph spanda {\n  rankdir=TB;\n  node [fontsize=10];\n");
    for node in &graph.nodes {
        let color = match node.kind {
            GraphNodeKind::Mission => "lightblue",
            GraphNodeKind::Capability => "lightyellow",
            GraphNodeKind::Hardware => "lightgray",
            GraphNodeKind::Provider => "lightgreen",
            GraphNodeKind::Package => "honeydew",
            GraphNodeKind::Safety => "mistyrose",
            GraphNodeKind::Robot => "white",
            GraphNodeKind::Sensor | GraphNodeKind::Actuator => "azure",
        };
        out.push_str(&format!(
            "  \"{}\" [label=\"{}\\n{:?}\" fillcolor=\"{}\" style=filled];\n",
            dot_id(&node.id),
            escape_dot(&node.label),
            node.kind,
            color
        ));
    }
    for edge in &graph.edges {
        out.push_str(&format!(
            "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
            dot_id(&edge.from),
            dot_id(&edge.to),
            escape_dot(&edge.relation)
        ));
    }
    out.push_str("}\n");
    out
}

fn format_text(graph: &DependencyGraph) -> String {
    let mut out = format!("Dependency graph: {}\n", graph.source);
    out.push_str(&format!("Nodes: {}\n", graph.nodes.len()));
    for node in &graph.nodes {
        out.push_str(&format!("  [{:?}] {}\n", node.kind, node.label));
    }
    out.push_str(&format!("Edges: {}\n", graph.edges.len()));
    for edge in &graph.edges {
        out.push_str(&format!(
            "  {} --{}--> {}\n",
            edge.from, edge.relation, edge.to
        ));
    }
    out
}

fn mermaid_id(id: &str) -> String {
    id.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn dot_id(id: &str) -> String {
    id.replace('"', "\\\"")
}

fn escape_label(s: &str) -> String {
    s.replace('"', "'")
}

fn escape_dot(s: &str) -> String {
    s.replace('"', "\\\"")
}
