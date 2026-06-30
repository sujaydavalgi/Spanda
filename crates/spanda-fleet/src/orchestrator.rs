//! Distributed fleet orchestration beyond in-process `spanda fleet run`.
//!
//! Builds coordination plans from program-level `fleet` declarations and peer-robot wiring,
//! then executes a round-robin mission coordination pass across fleet members.

use crate::mesh::relay_deliveries_via_mesh;
use crate::platform_events::record_fleet_member_joined;
use crate::remote::{relay_peer_deliveries, FleetAgentRegistry};
pub use crate::types::PeerDelivery;
use serde::{Deserialize, Serialize};
use spanda_ast::comm_decl::PeerRobotDecl;
use spanda_ast::foundations::MissionDecl;
use spanda_ast::nodes::{Program, RobotDecl};
use spanda_ast::robotics_decl::FleetDecl;
use spanda_runtime::robotics::{FleetRegistry, MissionRuntime};

/// In-memory peer mesh used to simulate mission-step delivery during orchestration.
#[derive(Debug, Clone, Default)]
pub struct FleetPeerMesh {
    published: Vec<(String, Option<String>)>,
}

impl FleetPeerMesh {
    pub fn new() -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_fleet::orchestrator::new();

        Self::default()
    }

    pub fn register_robot(&mut self, _name: impl AsRef<str>) {

        // Description:

        //     Register robot.

        //

        // Inputs:

        //     &mut self: value

        //         Caller-supplied &mut self.

        //     _name: impl AsRef<str>

        //         Caller-supplied name.

        //

        // Outputs:

        //     None.

        //

        // Example:

        //     let result = spanda_fleet::orchestrator::register_robot(&mut self, _name);
    }

    pub fn publish_peer(&mut self, peer: &str, topic: &str, from_robot: &str) {
        // Description:
        //     Publish peer.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     peer: &str
        //         Caller-supplied peer.
        //     opic: &str
        //         Caller-supplied opic.
        //     from_robo: &str
        //         Caller-supplied from robo.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_fleet::orchestrator::publish_peer(&mut self, peer, opic, from_robo);

        let path = format!("/{peer}/{topic}");
        self.published.push((path, Some(from_robot.to_string())));
    }

    fn was_delivered(&self, peer: &str, topic: &str, from_robot: &str) -> bool {
        // Description:
        //     Was delivered.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     peer: &str
        //         Caller-supplied peer.
        //     opic: &str
        //         Caller-supplied opic.
        //     from_robo: &str
        //         Caller-supplied from robo.
        //
        // Outputs:
        //     result: bool
        //         Return value from `was_delivered`.
        //
        // Example:

        //     let result = spanda_fleet::orchestrator::was_delivered(&self, peer, opic, from_robo);

        let path = format!("/{peer}/{topic}");
        self.published.iter().any(|(topic_path, source_id)| {
            topic_path == &path && source_id.as_deref() == Some(from_robot)
        })
    }
}

/// Per-member coordination state during orchestration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetMemberState {
    pub robot_name: String,
    pub mission_name: Option<String>,
    pub mission_state: String,
    pub current_step: String,
    pub has_peer_link: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peer_handoffs: Vec<String>,
}

/// Orchestration report for one fleet group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetOrchestrationReport {
    pub fleet_name: String,
    pub members: Vec<FleetMemberState>,
    pub coordination_mode: String,
    pub steps_advanced: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peer_messages: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peer_deliveries: Vec<PeerDelivery>,
    #[serde(default)]
    pub remote_relayed: u32,
    #[serde(default)]
    pub remote_failed: u32,
}

/// Full orchestration result for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetOrchestrationResult {
    pub program: String,
    pub fleets: Vec<FleetOrchestrationReport>,
    pub success: bool,
}

fn robot_by_name<'a>(robots: &'a [RobotDecl], name: &str) -> Option<&'a RobotDecl> {
    robots.iter().find(|r| r.name() == name)
}

fn mission_for_robot(robot: &RobotDecl) -> Option<MissionRuntime> {
    // Description:
    //     Mission for robot.
    //
    // Inputs:
    //     robo: &RobotDecl
    //         Caller-supplied robo.
    //
    // Outputs:
    //     result: Option<MissionRuntime>
    //         Return value from `mission_for_robot`.
    //
    // Example:

    //     let result = spanda_fleet::orchestrator::mission_for_robot(robo);

    let RobotDecl::RobotDecl { mission, .. } = robot;
    let mission = mission.as_ref()?;
    let MissionDecl::MissionDecl {
        name,
        duration_hours,
        steps,
        ..
    } = mission;
    Some(MissionRuntime::new(
        name.clone(),
        steps.clone(),
        *duration_hours,
    ))
}

pub fn peer_handoffs(member_name: &str, step: &str, peer_robots: &[PeerRobotDecl]) -> Vec<String> {
    // Description:
    //     Peer handoffs.
    //
    // Inputs:
    //     ember_name: &str
    //         Caller-supplied ember name.
    //     step: &str
    //         Caller-supplied step.
    //     peer_robots: &[PeerRobotDecl]
    //         Caller-supplied peer robots.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `peer_handoffs`.
    //
    // Example:

    //     let result = spanda_fleet::orchestrator::peer_handoffs(ember_name, step, peer_robots);

    if step.is_empty() || peer_robots.is_empty() {
        return Vec::new();
    }
    peer_robots
        .iter()
        .map(|peer| {
            let PeerRobotDecl::PeerRobotDecl { name, .. } = peer;
            format!("{member_name}->{name}:step={step}")
        })
        .collect()
}

pub fn deliver_peer_steps(
    mesh: &mut FleetPeerMesh,
    from_robot: &str,
    step: &str,
    peer_robots: &[PeerRobotDecl],
) -> Vec<PeerDelivery> {
    // Description:
    //     Deliver peer steps.
    //
    // Inputs:
    //     esh: &mut FleetPeerMesh
    //         Caller-supplied esh.
    //     from_robo: &str
    //         Caller-supplied from robo.
    //     step: &str
    //         Caller-supplied step.
    //     peer_robots: &[PeerRobotDecl]
    //         Caller-supplied peer robots.
    //
    // Outputs:
    //     result: Vec<PeerDelivery>
    //         Return value from `deliver_peer_steps`.
    //
    // Example:

    //     let result = spanda_fleet::orchestrator::deliver_peer_steps(esh, from_robo, step, peer_robots);

    if step.is_empty() || peer_robots.is_empty() {
        return Vec::new();
    }
    let mut deliveries = Vec::new();
    for peer in peer_robots {
        let PeerRobotDecl::PeerRobotDecl { name, .. } = peer;
        mesh.register_robot(name);
        mesh.publish_peer(name, "mission_step", from_robot);
        let delivered = mesh.was_delivered(name, "mission_step", from_robot);
        deliveries.push(PeerDelivery {
            from_robot: from_robot.to_string(),
            to_robot: name.clone(),
            topic: "mission_step".into(),
            step: step.to_string(),
            delivered,
        });
    }
    deliveries
}

/// Build fleet registry from program declarations.
pub fn fleet_registry_from_program(program: &Program) -> FleetRegistry {
    // Description:
    //     Fleet registry from program.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: FleetRegistry
    //         Return value from `fleet_registry_from_program`.
    //
    // Example:

    //     let result = spanda_fleet::orchestrator::fleet_registry_from_program(progra);

    let Program::Program { fleets, .. } = program;
    let mut registry = FleetRegistry::default();
    for fleet in fleets {
        let FleetDecl::FleetDecl { name, members, .. } = fleet;
        registry.register(name, members.clone());
    }
    registry
}

/// Orchestrate fleet members by advancing missions in round-robin order.
pub fn orchestrate_fleets(program: &Program, program_path: &str) -> FleetOrchestrationResult {
    // Description:
    //     Orchestrate fleets.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     program_path: &str
    //         Caller-supplied program path.
    //
    // Outputs:
    //     result: FleetOrchestrationResult
    //         Return value from `orchestrate_fleets`.
    //
    // Example:

    //     let result = spanda_fleet::orchestrator::orchestrate_fleets(progra, program_path);

    let Program::Program { fleets, robots, .. } = program;
    let mut reports = Vec::new();

    for fleet in fleets {
        let FleetDecl::FleetDecl { name, members, .. } = fleet;
        let mut member_states = Vec::new();
        let mut steps_advanced = 0u32;
        let mut peer_messages = Vec::new();
        let mut peer_deliveries = Vec::new();
        let mut mesh = FleetPeerMesh::new();

        for member_name in members {
            record_fleet_member_joined(name, member_name);
            mesh.register_robot(member_name);
        }

        for member_name in members {
            let Some(robot) = robot_by_name(robots, member_name) else {
                member_states.push(FleetMemberState {
                    robot_name: member_name.clone(),
                    mission_name: None,
                    mission_state: "MissingRobot".into(),
                    current_step: String::new(),
                    has_peer_link: false,
                    peer_handoffs: Vec::new(),
                });
                continue;
            };
            let RobotDecl::RobotDecl {
                peer_robots,
                mission,
                ..
            } = robot;
            let mut runtime = mission_for_robot(robot);
            let (mission_name, mission_state, current_step) = if let Some(ref mut m) = runtime {
                m.start();
                let step = m.advance().unwrap_or_default();
                if !step.is_empty() {
                    steps_advanced += 1;
                }
                (m.name.clone(), m.state.as_str().to_string(), step)
            } else {
                (None, "NoMission".into(), String::new())
            };
            let handoffs = peer_handoffs(member_name, &current_step, peer_robots);
            peer_messages.extend(handoffs.clone());
            peer_deliveries.extend(deliver_peer_steps(
                &mut mesh,
                member_name,
                &current_step,
                peer_robots,
            ));
            let _ = mission;
            member_states.push(FleetMemberState {
                robot_name: member_name.clone(),
                mission_name,
                mission_state,
                current_step,
                has_peer_link: !peer_robots.is_empty(),
                peer_handoffs: handoffs,
            });
        }

        let has_peer_link = member_states.iter().any(|m| m.has_peer_link);
        let coordination_mode = if !peer_deliveries.is_empty() {
            "peer_mesh_mission".into()
        } else if has_peer_link {
            "peer_round_robin_mission".into()
        } else {
            "round_robin_mission".into()
        };
        reports.push(FleetOrchestrationReport {
            fleet_name: name.clone(),
            members: member_states,
            coordination_mode,
            steps_advanced,
            peer_messages,
            peer_deliveries,
            remote_relayed: 0,
            remote_failed: 0,
        });
    }

    let success = reports
        .iter()
        .all(|r| r.members.iter().all(|m| m.mission_state != "MissingRobot"));

    FleetOrchestrationResult {
        program: program_path.to_string(),
        fleets: reports,
        success,
    }
}

/// Orchestrate fleets and relay peer deliveries to registered remote fleet agents.
pub fn orchestrate_fleets_remote(
    program: &Program,
    program_path: &str,
    registry: &FleetAgentRegistry,
) -> FleetOrchestrationResult {
    // Description:
    //     Orchestrate fleets remote.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     program_path: &str
    //         Caller-supplied program path.
    //     registry: &FleetAgentRegistry
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: FleetOrchestrationResult
    //         Return value from `orchestrate_fleets_remote`.
    //
    // Example:

    //     let result = spanda_fleet::orchestrator::orchestrate_fleets_remote(progra, program_path, registry);

    let mut result = orchestrate_fleets(program, program_path);
    let mut success = result.success;
    for fleet in &mut result.fleets {
        let (relayed, failed) = relay_peer_deliveries(&fleet.peer_deliveries, registry);
        fleet.remote_relayed = relayed;
        fleet.remote_failed = failed;
        if relayed > 0 {
            fleet.coordination_mode = "distributed_peer_mesh".into();
        }
        if failed > 0 {
            success = false;
        }
    }
    result.success = success;
    result
}

/// Orchestrate fleets and relay peer deliveries through a mesh coordinator.
pub fn orchestrate_fleets_mesh(
    program: &Program,
    program_path: &str,
    mesh_url: &str,
    token: Option<&str>,
) -> FleetOrchestrationResult {
    // Description:
    //     Orchestrate fleets mesh.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     program_path: &str
    //         Caller-supplied program path.
    //     mesh_url: &str
    //         Caller-supplied mesh url.
    //     token: Option<&str>
    //         Caller-supplied token.
    //
    // Outputs:
    //     result: FleetOrchestrationResult
    //         Return value from `orchestrate_fleets_mesh`.
    //
    // Example:

    //     let result = spanda_fleet::orchestrator::orchestrate_fleets_mesh(progra, program_path, esh_url, oken);

    let mut result = orchestrate_fleets(program, program_path);
    let mut success = result.success;
    for fleet in &mut result.fleets {
        if fleet.peer_deliveries.is_empty() {
            continue;
        }
        match relay_deliveries_via_mesh(mesh_url, &fleet.peer_deliveries, token) {
            Ok(resp) => {
                fleet.remote_relayed = resp.relayed;
                fleet.remote_failed = resp.failed;
                if resp.relayed > 0 {
                    fleet.coordination_mode = "distributed_peer_mesh".into();
                }
                if resp.failed > 0 {
                    success = false;
                }
            }
            Err(_) => {
                fleet.remote_failed = fleet.peer_deliveries.len() as u32;
                success = false;
            }
        }
    }
    result.success = success;
    result
}
