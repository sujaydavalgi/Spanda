//! Digital Thread v1 — query capability-to-device trace chains.
//!
use crate::build::{build_dependency_graph, DependencyGraph, GraphEdge, GraphNode};
use serde::{Deserialize, Serialize};
use spanda_ast::nodes::Program;
use spanda_capability::{
    capability_traceability, hardware_traceability, CapabilityTraceRow, HardwareTraceRow,
};
use spanda_config::ResolvedSystemConfig;
use std::collections::{HashSet, VecDeque};

/// Filters for digital thread graph traversal.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DigitalThreadQuery {
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub device_id: Option<String>,
    #[serde(default)]
    pub node_id: Option<String>,
}

/// Device link from configuration registry into the trace graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalThreadDeviceLink {
    pub device_id: String,
    pub device_type: String,
    pub assigned_robot: Option<String>,
    pub lifecycle_state: Option<String>,
    pub related_capabilities: Vec<String>,
}

/// Digital thread query result for Control Center and SDK consumers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalThreadReport {
    pub query: DigitalThreadQuery,
    pub source: String,
    pub graph: DependencyGraph,
    pub capability_rows: Vec<CapabilityTraceRow>,
    pub hardware_rows: Vec<HardwareTraceRow>,
    pub device_links: Vec<DigitalThreadDeviceLink>,
    pub chain_summary: Vec<String>,
    pub matched_node_count: usize,
    pub matched_edge_count: usize,
}

/// Build and filter a digital thread from program AST, traceability, and device registry.
pub fn query_digital_thread(
    program: &Program,
    source: &str,
    config: Option<&ResolvedSystemConfig>,
    query: &DigitalThreadQuery,
) -> DigitalThreadReport {
    let full_graph = build_dependency_graph(program, source, config);
    let trace = capability_traceability(program);
    let hardware = hardware_traceability(program);
    let device_links = link_devices(config, &trace.capability_rows);
    let (nodes, edges) = filter_graph(&full_graph, query, &trace.capability_rows, &device_links);
    let chain_summary = summarize_chain(query, &nodes, &edges, &device_links);
    let matched_node_count = nodes.len();
    let matched_edge_count = edges.len();
    let capability_rows = filter_capability_rows(&trace.capability_rows, query);
    let hardware_rows = filter_hardware_rows(&hardware.hardware_rows, query, &capability_rows);

    DigitalThreadReport {
        query: query.clone(),
        source: source.to_string(),
        graph: DependencyGraph {
            source: full_graph.source,
            nodes,
            edges,
        },
        capability_rows,
        hardware_rows,
        device_links,
        chain_summary,
        matched_node_count,
        matched_edge_count,
    }
}

fn link_devices(
    config: Option<&ResolvedSystemConfig>,
    capability_rows: &[CapabilityTraceRow],
) -> Vec<DigitalThreadDeviceLink> {
    let Some(resolved) = config else {
        return Vec::new();
    };
    let registry = &resolved.device_registry;
    registry
        .devices
        .iter()
        .map(|device| {
            let related_capabilities = capability_rows
                .iter()
                .filter(|row| {
                    row.hardware.eq_ignore_ascii_case(&device.device_type)
                        || device
                            .assigned_robot
                            .as_deref()
                            .map(|r| row.required_by.eq_ignore_ascii_case(r))
                            .unwrap_or(false)
                })
                .map(|row| row.capability.clone())
                .collect();
            DigitalThreadDeviceLink {
                device_id: device.id.clone(),
                device_type: device.device_type.clone(),
                assigned_robot: device.assigned_robot.clone(),
                lifecycle_state: device.lifecycle_state.clone(),
                related_capabilities,
            }
        })
        .collect()
}

fn filter_graph(
    graph: &DependencyGraph,
    query: &DigitalThreadQuery,
    capability_rows: &[CapabilityTraceRow],
    device_links: &[DigitalThreadDeviceLink],
) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    if query.capability.is_none() && query.device_id.is_none() && query.node_id.is_none() {
        return (graph.nodes.clone(), graph.edges.clone());
    }

    let mut seed_ids: HashSet<String> = HashSet::new();
    if let Some(node_id) = &query.node_id {
        seed_ids.insert(node_id.clone());
    }
    if let Some(capability) = &query.capability {
        seed_ids.insert(format!("capability:{capability}").to_ascii_lowercase());
        for row in capability_rows {
            if row.capability.eq_ignore_ascii_case(capability) {
                seed_ids.insert(format!("hardware:{}", row.hardware).to_ascii_lowercase());
                seed_ids.insert(format!("robot:{}", row.required_by).to_ascii_lowercase());
            }
        }
    }
    if let Some(device_id) = &query.device_id {
        if let Some(link) = device_links.iter().find(|d| d.device_id == *device_id) {
            if let Some(robot) = &link.assigned_robot {
                seed_ids.insert(format!("robot:{robot}").to_ascii_lowercase());
            }
            for capability in &link.related_capabilities {
                seed_ids.insert(format!("capability:{capability}").to_ascii_lowercase());
            }
        }
    }

    let node_map: std::collections::HashMap<String, GraphNode> = graph
        .nodes
        .iter()
        .map(|node| (node.id.clone(), node.clone()))
        .collect();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = seed_ids.into_iter().collect();
    while let Some(id) = queue.pop_front() {
        if !visited.insert(id.clone()) {
            continue;
        }
        for edge in &graph.edges {
            if edge.from == id && !visited.contains(&edge.to) {
                queue.push_back(edge.to.clone());
            }
            if edge.to == id && !visited.contains(&edge.from) {
                queue.push_back(edge.from.clone());
            }
        }
    }

    let nodes: Vec<GraphNode> = visited
        .iter()
        .filter_map(|id| node_map.get(id).cloned())
        .collect();
    let node_set: HashSet<String> = visited;
    let edges: Vec<GraphEdge> = graph
        .edges
        .iter()
        .filter(|edge| node_set.contains(&edge.from) && node_set.contains(&edge.to))
        .cloned()
        .collect();
    (nodes, edges)
}

fn summarize_chain(
    query: &DigitalThreadQuery,
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    device_links: &[DigitalThreadDeviceLink],
) -> Vec<String> {
    let mut lines = vec![format!(
        "Digital thread query: {}",
        serde_json::to_string(query).unwrap_or_else(|_| "{}".into())
    )];
    lines.push(format!(
        "Matched {} nodes, {} edges",
        nodes.len(),
        edges.len()
    ));
    for edge in edges.iter().take(12) {
        lines.push(format!("{} --{}--> {}", edge.from, edge.relation, edge.to));
    }
    if let Some(device_id) = &query.device_id {
        if let Some(link) = device_links.iter().find(|d| d.device_id == *device_id) {
            lines.push(format!(
                "Device {} ({}) → capabilities: {}",
                link.device_id,
                link.device_type,
                link.related_capabilities.join(", ")
            ));
        }
    }
    lines
}

fn filter_capability_rows(
    rows: &[CapabilityTraceRow],
    query: &DigitalThreadQuery,
) -> Vec<CapabilityTraceRow> {
    if let Some(capability) = &query.capability {
        return rows
            .iter()
            .filter(|row| row.capability.eq_ignore_ascii_case(capability))
            .cloned()
            .collect();
    }
    if query.device_id.is_some() || query.node_id.is_some() {
        return rows.to_vec();
    }
    rows.to_vec()
}

fn filter_hardware_rows(
    rows: &[HardwareTraceRow],
    _query: &DigitalThreadQuery,
    capability_rows: &[CapabilityTraceRow],
) -> Vec<HardwareTraceRow> {
    if capability_rows.is_empty() {
        return rows.to_vec();
    }
    let hardware: HashSet<String> = capability_rows
        .iter()
        .map(|row| row.hardware.clone())
        .collect();
    rows.iter()
        .filter(|row| {
            hardware.is_empty()
                || hardware
                    .iter()
                    .any(|h| h.eq_ignore_ascii_case(&row.hardware_component))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;
    use std::path::PathBuf;

    #[test]
    fn query_capability_filters_graph() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/showcase/compliance/defense_rover.sd");
        let source = std::fs::read_to_string(&path).expect("defense_rover.sd");
        let tokens = tokenize(&source).expect("tokenize");
        let program = parse(tokens).expect("parse");
        let report = query_digital_thread(
            &program,
            "defense_rover.sd",
            None,
            &DigitalThreadQuery::default(),
        );
        assert!(report.matched_node_count > 0);
        assert!(!report.chain_summary.is_empty());
    }
}
