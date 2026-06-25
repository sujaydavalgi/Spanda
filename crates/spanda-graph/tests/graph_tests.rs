//! Integration tests for dependency graph construction.

use spanda_graph::{build_dependency_graph, format_dependency_graph, GraphFormat};
use spanda_lexer::tokenize;
use spanda_parser::parse;

fn parse_source(source: &str) -> spanda_ast::nodes::Program {
    let tokens = tokenize(source).expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn builds_mission_capability_hardware_edges() {
    let source = r#"
hardware RoverV1 { sensors [ GPS, Lidar ]; actuators [ DifferentialDrive ]; }
robot Rover {
  uses hardware RoverV1;
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  exposes capabilities [ gps_navigation ];
  mission Patrol { requires capabilities [ gps_navigation ]; patrol; }
  safety { max_speed = 1 m/s; }
  behavior patrol() {}
}
deploy Rover to RoverV1;
"#;
    let program = parse_source(source);
    let graph = build_dependency_graph(&program, "rover.sd", None);
    assert!(graph.nodes.iter().any(|n| n.label == "Patrol"));
    assert!(graph.nodes.iter().any(|n| n.label == "gps_navigation"));
    assert!(graph.edges.iter().any(|e| e.relation == "requires"));
    assert!(graph.edges.iter().any(|e| e.relation == "deploy_to"));
}

#[test]
fn renders_mermaid_output() {
    let source = r#"
robot R { sensor g: GPS; actuator w: DifferentialDrive; behavior b() {} }
"#;
    let program = parse_source(source);
    let graph = build_dependency_graph(&program, "r.sd", None);
    let mermaid = format_dependency_graph(&graph, GraphFormat::Mermaid);
    assert!(mermaid.contains("flowchart TD"));
    assert!(mermaid.contains("-->"));
}
