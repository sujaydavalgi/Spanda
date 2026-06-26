//! Build dependency graphs from programs and optional resolved configuration.
//!
use serde::{Deserialize, Serialize};
use spanda_ast::foundations::{DeployDecl, HardwareDecl, MissionDecl};
use spanda_ast::nodes::{ImportDecl, Program, RobotDecl};
use spanda_capability::{capability_traceability, infer_robot_capabilities};
use spanda_config::ResolvedSystemConfig;
use std::collections::HashMap;

/// Node category in the dependency graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeKind {
    Mission,
    Robot,
    Capability,
    Hardware,
    Provider,
    Package,
    Safety,
    Sensor,
    Actuator,
}

/// A node in the dependency graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: GraphNodeKind,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

/// A directed edge between graph nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
}

/// Full dependency graph for a Spanda program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub source: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

struct GraphBuilder {
    nodes: HashMap<String, GraphNode>,
    edges: Vec<GraphEdge>,
}

impl GraphBuilder {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    fn node_id(kind: GraphNodeKind, key: &str) -> String {
        format!("{kind:?}:{key}").to_ascii_lowercase()
    }

    fn add_node(&mut self, kind: GraphNodeKind, key: &str, label: &str) -> String {
        let id = Self::node_id(kind, key);
        self.nodes.entry(id.clone()).or_insert_with(|| GraphNode {
            id: id.clone(),
            label: label.to_string(),
            kind,
            metadata: HashMap::new(),
        });
        id
    }

    fn add_edge(&mut self, from: &str, to: &str, relation: &str) {
        if from == to {
            return;
        }
        let edge = GraphEdge {
            from: from.to_string(),
            to: to.to_string(),
            relation: relation.to_string(),
        };
        if !self
            .edges
            .iter()
            .any(|e| e.from == edge.from && e.to == edge.to && e.relation == edge.relation)
        {
            self.edges.push(edge);
        }
    }

    fn finish(self, source: &str) -> DependencyGraph {
        let mut nodes: Vec<GraphNode> = self.nodes.into_values().collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        DependencyGraph {
            source: source.to_string(),
            nodes,
            edges: self.edges,
        }
    }
}

/// Build a mission-to-packages dependency graph for a parsed program.
pub fn build_dependency_graph(
    program: &Program,
    source: &str,
    config: Option<&ResolvedSystemConfig>,
) -> DependencyGraph {
    // Assemble nodes and edges from AST, capability traceability, and config.
    //
    // Parameters:
    // - `program` — parsed `.sd` program
    // - `source` — source path label for the graph metadata
    // - `config` — optional resolved system configuration
    //
    // Returns:
    // Dependency graph with mission → capability → hardware → provider/package edges.
    //
    // Options:
    // None.
    //
    // Example:
    // let graph = build_dependency_graph(&program, "rover.sd", cfg.as_deref());

    let mut builder = GraphBuilder::new();
    let Program::Program {
        hardware_profiles,
        robots,
        imports,
        kill_switches,
        ..
    } = program;

    for profile in hardware_profiles {
        let HardwareDecl::HardwareDecl { name, .. } = profile;
        builder.add_node(GraphNodeKind::Hardware, name, name);
    }

    for import in imports {
        let ImportDecl::ImportDecl { path, .. } = import;
        let provider = path.split('.').next().unwrap_or(path);
        builder.add_node(GraphNodeKind::Provider, provider, provider);
        builder.add_node(GraphNodeKind::Package, path, path);
        let provider_id = GraphBuilder::node_id(GraphNodeKind::Provider, provider);
        let package_id = GraphBuilder::node_id(GraphNodeKind::Package, path);
        builder.add_edge(&package_id, &provider_id, "provided_by");
    }

    if let Some(cfg) = config {
        for provider in &cfg.providers {
            builder.add_node(GraphNodeKind::Provider, provider, provider);
        }
        for package in &cfg.packages {
            builder.add_node(GraphNodeKind::Package, package, package);
        }
        for device in &cfg.device_registry.devices {
            if let Some(provider) = &device.provider {
                let provider_id = builder.add_node(GraphNodeKind::Provider, provider, provider);
                let device_id = builder.add_node(GraphNodeKind::Hardware, &device.id, &device.id);
                builder.add_edge(&device_id, &provider_id, "uses_provider");
            }
        }
    }

    for robot in robots {
        let RobotDecl::RobotDecl {
            name,
            sensors,
            actuators,
            uses_hardware,
            exposes_capabilities,
            mission,
            ..
        } = robot;
        let robot_id = builder.add_node(GraphNodeKind::Robot, name, name);
        if let Some(hw) = uses_hardware {
            let hw_id = builder.add_node(GraphNodeKind::Hardware, hw, hw);
            builder.add_edge(&robot_id, &hw_id, "uses_hardware");
        }
        for cap in exposes_capabilities {
            let cap_id = builder.add_node(GraphNodeKind::Capability, cap, cap);
            builder.add_edge(&robot_id, &cap_id, "exposes");
        }
        if let Some(MissionDecl::MissionDecl {
            name: mission_name,
            required_capabilities,
            ..
        }) = mission
        {
            let label = mission_name.as_deref().unwrap_or("mission");
            let mission_id = builder.add_node(GraphNodeKind::Mission, label, label);
            builder.add_edge(&robot_id, &mission_id, "runs");
            for cap in required_capabilities {
                let cap_id = builder.add_node(GraphNodeKind::Capability, cap, cap);
                builder.add_edge(&mission_id, &cap_id, "requires");
            }
        }
        for sensor in sensors {
            let spanda_ast::nodes::SensorDecl::SensorDecl {
                name: sensor_name,
                sensor_type,
                ..
            } = sensor;
            let sensor_id = builder.add_node(GraphNodeKind::Sensor, sensor_name, sensor_name);
            builder.add_edge(&robot_id, &sensor_id, "has_sensor");
            let hw_id = builder.add_node(GraphNodeKind::Hardware, sensor_type, sensor_type);
            builder.add_edge(&sensor_id, &hw_id, "device_type");
        }
        for actuator in actuators {
            let spanda_ast::nodes::ActuatorDecl::ActuatorDecl {
                name: actuator_name,
                actuator_type,
                ..
            } = actuator;
            let actuator_id =
                builder.add_node(GraphNodeKind::Actuator, actuator_name, actuator_name);
            builder.add_edge(&robot_id, &actuator_id, "has_actuator");
            let hw_id = builder.add_node(GraphNodeKind::Hardware, actuator_type, actuator_type);
            builder.add_edge(&actuator_id, &hw_id, "device_type");
        }
        let safety_id = builder.add_node(GraphNodeKind::Safety, name, &format!("safety:{name}"));
        builder.add_edge(&robot_id, &safety_id, "protected_by");
    }

    if !kill_switches.is_empty() {
        let safety_id = builder.add_node(GraphNodeKind::Safety, "kill_switch", "kill switch");
        for robot in robots {
            let RobotDecl::RobotDecl { name, .. } = robot;
            let robot_id = GraphBuilder::node_id(GraphNodeKind::Robot, name);
            builder.add_edge(&robot_id, &safety_id, "kill_switch");
        }
    }

    let trace = capability_traceability(program);
    for row in trace.capability_rows {
        let cap_id = builder.add_node(GraphNodeKind::Capability, &row.capability, &row.capability);
        if !row.hardware.is_empty() {
            let hw_id = builder.add_node(GraphNodeKind::Hardware, &row.hardware, &row.hardware);
            builder.add_edge(&cap_id, &hw_id, "requires_hardware");
        }
        if !row.package.is_empty() {
            for package in row.package.split('+').filter(|p| !p.is_empty()) {
                let pkg_id = builder.add_node(GraphNodeKind::Package, package, package);
                builder.add_edge(&cap_id, &pkg_id, "requires_package");
            }
        }
        if !row.provider.is_empty() {
            for provider in row.provider.split('+').filter(|p| !p.is_empty()) {
                let provider_id = builder.add_node(GraphNodeKind::Provider, provider, provider);
                builder.add_edge(&cap_id, &provider_id, "requires_provider");
            }
        }
    }

    for report in infer_robot_capabilities(program) {
        for row in report.rows {
            let cap_id =
                builder.add_node(GraphNodeKind::Capability, &row.capability, &row.capability);
            for component in &row.required_components {
                let hw_id = builder.add_node(GraphNodeKind::Hardware, component, component);
                builder.add_edge(&cap_id, &hw_id, "requires_component");
            }
        }
    }

    let Program::Program { deployments, .. } = program;
    for deploy in deployments {
        let DeployDecl::DeployDecl {
            robot_name,
            targets,
            ..
        } = deploy;
        let robot_id = GraphBuilder::node_id(GraphNodeKind::Robot, robot_name);
        for target in targets {
            let hw_id = builder.add_node(GraphNodeKind::Hardware, target, target);
            builder.add_edge(&robot_id, &hw_id, "deploy_to");
        }
    }

    builder.finish(source)
}
