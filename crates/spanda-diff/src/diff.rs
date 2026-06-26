//! Program snapshot extraction and mission diff reporting.

use serde::{Deserialize, Serialize};
use spanda_ast::assurance_decl::{ContinuityPolicyDecl, MissionPlanDecl, RecoveryPolicyDecl};
use spanda_ast::foundations::{
    DeployDecl, HardwareDecl, HealthCheckDecl, HealthPolicyDecl, KillSwitchDecl,
};
use spanda_ast::nodes::{Program, RobotDecl, SafetyBlock, SafetyRule};
use spanda_ast::robotics_decl::FleetDecl;
use spanda_capability::infer_robot_capabilities;
use std::collections::{BTreeMap, BTreeSet};

/// Mission area where a diff was detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionDiffDimension {
    Import,
    Hardware,
    Robot,
    Capability,
    Mission,
    Safety,
    Sensor,
    Actuator,
    Behavior,
    Deploy,
    HealthCheck,
    HealthPolicy,
    KillSwitch,
    RecoveryPolicy,
    ContinuityPolicy,
    Fleet,
    MissionPlan,
}

/// Kind of change between baseline and candidate programs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffChangeKind {
    Added,
    Removed,
    Modified,
}

/// Single mission diff finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionDiffChange {
    pub dimension: MissionDiffDimension,
    pub kind: DiffChangeKind,
    pub path: String,
    pub summary: String,
    pub before: Option<String>,
    pub after: Option<String>,
    pub impact: String,
}

/// Mission diff report between two programs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionDiffReport {
    pub baseline: String,
    pub candidate: String,
    pub changes: Vec<MissionDiffChange>,
    pub added: u32,
    pub removed: u32,
    pub modified: u32,
    pub has_deploy_impact: bool,
    pub has_safety_impact: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RobotSnapshot {
    hardware: Option<String>,
    capabilities: BTreeSet<String>,
    sensors: BTreeSet<String>,
    actuators: BTreeSet<String>,
    behaviors: BTreeSet<String>,
    mission: Option<String>,
    safety: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProgramSnapshot {
    imports: BTreeSet<String>,
    hardware: BTreeMap<String, String>,
    robots: BTreeMap<String, RobotSnapshot>,
    deployments: BTreeSet<String>,
    health_checks: BTreeSet<String>,
    health_policies: BTreeSet<String>,
    kill_switches: BTreeSet<String>,
    recovery_policies: BTreeSet<String>,
    continuity_policies: BTreeSet<String>,
    fleets: BTreeSet<String>,
    mission_plans: BTreeSet<String>,
}

/// Diff two parsed programs and produce a change-impact report.
pub fn diff_programs(
    baseline: &Program,
    candidate: &Program,
    baseline_label: &str,
    candidate_label: &str,
) -> MissionDiffReport {
    // Compare structured snapshots from baseline and candidate missions.
    //
    // Parameters:
    // - `baseline` — approved or older program
    // - `candidate` — newer or proposed program
    // - `baseline_label` — display label for baseline file
    // - `candidate_label` — display label for candidate file
    //
    // Returns:
    // Mission diff report with categorized changes and impact flags.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = diff_programs(&base, &next, "rover-v1.sd", "rover-v2.sd");

    let left = snapshot_program(baseline);
    let right = snapshot_program(candidate);
    let mut changes = Vec::new();

    diff_string_sets(
        MissionDiffDimension::Import,
        "import",
        &left.imports,
        &right.imports,
        "provider import",
        &mut changes,
    );
    diff_named_summaries(
        MissionDiffDimension::Hardware,
        "hardware",
        &left.hardware,
        &right.hardware,
        "hardware profile",
        &mut changes,
    );
    diff_named_summaries(
        MissionDiffDimension::Robot,
        "robot",
        &robot_names(&left.robots),
        &robot_names(&right.robots),
        "robot declaration",
        &mut changes,
    );
    diff_robots(&left.robots, &right.robots, &mut changes);
    diff_string_sets(
        MissionDiffDimension::Deploy,
        "deploy",
        &left.deployments,
        &right.deployments,
        "deployment binding",
        &mut changes,
    );
    diff_string_sets(
        MissionDiffDimension::HealthCheck,
        "health_check",
        &left.health_checks,
        &right.health_checks,
        "health check",
        &mut changes,
    );
    diff_string_sets(
        MissionDiffDimension::HealthPolicy,
        "health_policy",
        &left.health_policies,
        &right.health_policies,
        "health policy",
        &mut changes,
    );
    diff_string_sets(
        MissionDiffDimension::KillSwitch,
        "kill_switch",
        &left.kill_switches,
        &right.kill_switches,
        "kill switch",
        &mut changes,
    );
    diff_string_sets(
        MissionDiffDimension::RecoveryPolicy,
        "recovery_policy",
        &left.recovery_policies,
        &right.recovery_policies,
        "recovery policy",
        &mut changes,
    );
    diff_string_sets(
        MissionDiffDimension::ContinuityPolicy,
        "continuity_policy",
        &left.continuity_policies,
        &right.continuity_policies,
        "continuity policy",
        &mut changes,
    );
    diff_string_sets(
        MissionDiffDimension::Fleet,
        "fleet",
        &left.fleets,
        &right.fleets,
        "fleet declaration",
        &mut changes,
    );
    diff_string_sets(
        MissionDiffDimension::MissionPlan,
        "mission_plan",
        &left.mission_plans,
        &right.mission_plans,
        "mission plan",
        &mut changes,
    );

    let added = changes
        .iter()
        .filter(|c| c.kind == DiffChangeKind::Added)
        .count() as u32;
    let removed = changes
        .iter()
        .filter(|c| c.kind == DiffChangeKind::Removed)
        .count() as u32;
    let modified = changes
        .iter()
        .filter(|c| c.kind == DiffChangeKind::Modified)
        .count() as u32;
    let has_deploy_impact = changes.iter().any(|c| {
        matches!(
            c.dimension,
            MissionDiffDimension::Deploy
                | MissionDiffDimension::Hardware
                | MissionDiffDimension::Capability
        )
    });
    let has_safety_impact = changes.iter().any(|c| {
        matches!(
            c.dimension,
            MissionDiffDimension::Safety | MissionDiffDimension::KillSwitch
        )
    });
    MissionDiffReport {
        baseline: baseline_label.into(),
        candidate: candidate_label.into(),
        changes,
        added,
        removed,
        modified,
        has_deploy_impact,
        has_safety_impact,
    }
}

fn snapshot_program(program: &Program) -> ProgramSnapshot {
    let Program::Program {
        imports,
        hardware_profiles,
        deployments,
        health_checks,
        health_policies,
        kill_switches,
        recovery_policies,
        continuity_policies,
        fleets,
        mission_plans,
        robots,
        ..
    } = program;

    ProgramSnapshot {
        imports: imports
            .iter()
            .map(|import| {
                let spanda_ast::nodes::ImportDecl::ImportDecl { path, .. } = import;
                path.clone()
            })
            .collect(),
        hardware: hardware_profiles
            .iter()
            .map(|profile| {
                let HardwareDecl::HardwareDecl {
                    name,
                    sensors,
                    actuators,
                    connectivity,
                    ..
                } = profile;
                (
                    name.clone(),
                    format!(
                        "sensors=[{}] actuators=[{}] connectivity=[{}]",
                        sensors.join(", "),
                        actuators.join(", "),
                        connectivity.join(", ")
                    ),
                )
            })
            .collect(),
        robots: robots
            .iter()
            .map(|robot| {
                let RobotDecl::RobotDecl { name, .. } = robot;
                (name.clone(), snapshot_robot(robot))
            })
            .collect(),
        deployments: deployments
            .iter()
            .map(|deploy| {
                let DeployDecl::DeployDecl {
                    robot_name,
                    targets,
                    ..
                } = deploy;
                format!("{robot_name} -> {}", targets.join(", "))
            })
            .collect(),
        health_checks: health_checks
            .iter()
            .map(|check| {
                let HealthCheckDecl::HealthCheckDecl { name, .. } = check;
                name.clone()
            })
            .collect(),
        health_policies: health_policies
            .iter()
            .map(|policy| {
                let HealthPolicyDecl::HealthPolicyDecl { name, .. } = policy;
                name.clone()
            })
            .collect(),
        kill_switches: kill_switches
            .iter()
            .map(|ks| {
                let KillSwitchDecl::KillSwitchDecl { name, .. } = ks;
                name.clone()
            })
            .collect(),
        recovery_policies: recovery_policies
            .iter()
            .map(|policy| {
                let RecoveryPolicyDecl::RecoveryPolicyDecl { name, .. } = policy;
                name.clone()
            })
            .collect(),
        continuity_policies: continuity_policies
            .iter()
            .map(|policy| {
                let ContinuityPolicyDecl::ContinuityPolicyDecl { name, .. } = policy;
                name.clone()
            })
            .collect(),
        fleets: fleets
            .iter()
            .map(|fleet| {
                let FleetDecl::FleetDecl { name, .. } = fleet;
                name.clone()
            })
            .collect(),
        mission_plans: mission_plans
            .iter()
            .map(|plan| {
                let MissionPlanDecl::MissionPlanDecl { name, .. } = plan;
                name.clone()
            })
            .collect(),
    }
}

fn snapshot_robot(robot: &RobotDecl) -> RobotSnapshot {
    let RobotDecl::RobotDecl {
        uses_hardware,
        exposes_capabilities,
        sensors,
        actuators,
        behaviors,
        mission,
        safety,
        ..
    } = robot;

    RobotSnapshot {
        hardware: uses_hardware.clone(),
        capabilities: exposes_capabilities.iter().cloned().collect(),
        sensors: sensors
            .iter()
            .map(|sensor| {
                let spanda_ast::nodes::SensorDecl::SensorDecl { name, .. } = sensor;
                name.clone()
            })
            .collect(),
        actuators: actuators
            .iter()
            .map(|actuator| {
                let spanda_ast::nodes::ActuatorDecl::ActuatorDecl { name, .. } = actuator;
                name.clone()
            })
            .collect(),
        behaviors: behaviors
            .iter()
            .map(|behavior| {
                let spanda_ast::nodes::BehaviorDecl::BehaviorDecl { name, .. } = behavior;
                name.clone()
            })
            .collect(),
        mission: mission.as_ref().map(mission_summary),
        safety: safety.as_ref().map(safety_summary),
    }
}

fn mission_summary(mission: &spanda_ast::foundations::MissionDecl) -> String {
    let spanda_ast::foundations::MissionDecl::MissionDecl {
        name,
        required_capabilities,
        steps,
        ..
    } = mission;
    format!(
        "name={} caps=[{}] steps=[{}]",
        name.as_deref().unwrap_or("unnamed"),
        required_capabilities.join(", "),
        steps.join(", ")
    )
}

fn safety_summary(safety: &SafetyBlock) -> String {
    let SafetyBlock::SafetyBlock { rules, .. } = safety;
    rules
        .iter()
        .map(|rule| match rule {
            SafetyRule::MaxSpeedRule { name, .. } => format!("max_speed:{name}"),
            SafetyRule::StopIfRule { .. } => "stop_if".into(),
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn robot_names(robots: &BTreeMap<String, RobotSnapshot>) -> BTreeMap<String, String> {
    robots
        .keys()
        .map(|name| (name.clone(), name.clone()))
        .collect()
}

fn diff_robots(
    left: &BTreeMap<String, RobotSnapshot>,
    right: &BTreeMap<String, RobotSnapshot>,
    changes: &mut Vec<MissionDiffChange>,
) {
    for name in left.keys().chain(right.keys()).collect::<BTreeSet<_>>() {
        let path = format!("robot/{name}");
        match (left.get(name), right.get(name)) {
            (Some(l), Some(r)) => {
                if l.hardware != r.hardware {
                    push_modified(
                        MissionDiffDimension::Robot,
                        &path,
                        "robot hardware profile changed",
                        l.hardware.as_deref(),
                        r.hardware.as_deref(),
                        "Re-run hardware verify and readiness",
                        changes,
                    );
                }
                diff_string_sets(
                    MissionDiffDimension::Capability,
                    &format!("{path}/capability"),
                    &l.capabilities,
                    &r.capabilities,
                    "robot capability",
                    changes,
                );
                diff_string_sets(
                    MissionDiffDimension::Sensor,
                    &format!("{path}/sensor"),
                    &l.sensors,
                    &r.sensors,
                    "robot sensor",
                    changes,
                );
                diff_string_sets(
                    MissionDiffDimension::Actuator,
                    &format!("{path}/actuator"),
                    &l.actuators,
                    &r.actuators,
                    "robot actuator",
                    changes,
                );
                diff_string_sets(
                    MissionDiffDimension::Behavior,
                    &format!("{path}/behavior"),
                    &l.behaviors,
                    &r.behaviors,
                    "robot behavior",
                    changes,
                );
                if l.mission != r.mission {
                    push_modified(
                        MissionDiffDimension::Mission,
                        &format!("{path}/mission"),
                        "robot mission definition changed",
                        l.mission.as_deref(),
                        r.mission.as_deref(),
                        "Re-run mission verify and readiness",
                        changes,
                    );
                }
                if l.safety != r.safety {
                    push_modified(
                        MissionDiffDimension::Safety,
                        &format!("{path}/safety"),
                        "robot safety rules changed",
                        l.safety.as_deref(),
                        r.safety.as_deref(),
                        "Re-run safety audit and safety-coverage",
                        changes,
                    );
                }
            }
            _ => {}
        }
    }
}

fn diff_string_sets(
    dimension: MissionDiffDimension,
    path_prefix: &str,
    left: &BTreeSet<String>,
    right: &BTreeSet<String>,
    label: &str,
    changes: &mut Vec<MissionDiffChange>,
) {
    for value in left.difference(right) {
        changes.push(MissionDiffChange {
            dimension,
            kind: DiffChangeKind::Removed,
            path: format!("{path_prefix}/{value}"),
            summary: format!("removed {label} `{value}`"),
            before: Some(value.clone()),
            after: None,
            impact: default_impact(dimension),
        });
    }
    for value in right.difference(left) {
        changes.push(MissionDiffChange {
            dimension,
            kind: DiffChangeKind::Added,
            path: format!("{path_prefix}/{value}"),
            summary: format!("added {label} `{value}`"),
            before: None,
            after: Some(value.clone()),
            impact: default_impact(dimension),
        });
    }
}

fn diff_named_summaries(
    dimension: MissionDiffDimension,
    path_prefix: &str,
    left: &BTreeMap<String, String>,
    right: &BTreeMap<String, String>,
    label: &str,
    changes: &mut Vec<MissionDiffChange>,
) {
    for key in left.keys().chain(right.keys()).collect::<BTreeSet<_>>() {
        match (left.get(key), right.get(key)) {
            (None, Some(after)) => changes.push(MissionDiffChange {
                dimension,
                kind: DiffChangeKind::Added,
                path: format!("{path_prefix}/{key}"),
                summary: format!("added {label} `{key}`"),
                before: None,
                after: Some(after.clone()),
                impact: default_impact(dimension),
            }),
            (Some(before), None) => changes.push(MissionDiffChange {
                dimension,
                kind: DiffChangeKind::Removed,
                path: format!("{path_prefix}/{key}"),
                summary: format!("removed {label} `{key}`"),
                before: Some(before.clone()),
                after: None,
                impact: default_impact(dimension),
            }),
            (Some(before), Some(after)) if before != after => push_modified(
                dimension,
                &format!("{path_prefix}/{key}"),
                &format!("modified {label} `{key}`"),
                Some(before),
                Some(after),
                &default_impact(dimension),
                changes,
            ),
            _ => {}
        }
    }
}

fn push_modified(
    dimension: MissionDiffDimension,
    path: &str,
    summary: &str,
    before: Option<&str>,
    after: Option<&str>,
    impact: &str,
    changes: &mut Vec<MissionDiffChange>,
) {
    changes.push(MissionDiffChange {
        dimension,
        kind: DiffChangeKind::Modified,
        path: path.into(),
        summary: summary.into(),
        before: before.map(str::to_string),
        after: after.map(str::to_string),
        impact: impact.into(),
    });
}

fn default_impact(dimension: MissionDiffDimension) -> String {
    match dimension {
        MissionDiffDimension::Deploy | MissionDiffDimension::Hardware => {
            "Re-run verify, readiness, and deploy gate".into()
        }
        MissionDiffDimension::Safety | MissionDiffDimension::KillSwitch => {
            "Re-run safety audit and safety-coverage".into()
        }
        MissionDiffDimension::Capability | MissionDiffDimension::Mission => {
            "Re-run capability traceability and mission verify".into()
        }
        MissionDiffDimension::RecoveryPolicy | MissionDiffDimension::ContinuityPolicy => {
            "Re-run recovery-coverage and continuity checks".into()
        }
        _ => "Review change impact before deploy".into(),
    }
}

/// Format a mission diff report for CLI output.
pub fn format_mission_diff(report: &MissionDiffReport, json: bool) -> String {
    // Serialize or pretty-print a mission diff report.
    //
    // Parameters:
    // - `report` — mission diff report
    // - `json` — emit JSON when true
    //
    // Returns:
    // Formatted report string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_mission_diff(&report, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|e| e.to_string());
    }
    let mut lines = vec![
        format!("Mission diff: {} -> {}", report.baseline, report.candidate),
        format!(
            "Changes: {} added, {} removed, {} modified",
            report.added, report.removed, report.modified
        ),
    ];
    if report.has_deploy_impact {
        lines.push("Deploy impact: yes".into());
    }
    if report.has_safety_impact {
        lines.push("Safety impact: yes".into());
    }
    if report.changes.is_empty() {
        lines.push("No differences detected.".into());
        return lines.join("\n");
    }
    for change in &report.changes {
        lines.push(format!(
            "  [{:?}/{:?}] {} — {}",
            change.kind, change.dimension, change.summary, change.impact
        ));
    }
    lines.join("\n")
}

/// Enrich diff with inferred capability rows from both programs.
pub fn diff_programs_with_capabilities(
    baseline: &Program,
    candidate: &Program,
    baseline_label: &str,
    candidate_label: &str,
) -> MissionDiffReport {
    let mut report = diff_programs(baseline, candidate, baseline_label, candidate_label);
    let left_caps = capability_sets(baseline);
    let right_caps = capability_sets(candidate);
    for (robot, left) in &left_caps {
        let path = format!("robot/{robot}/inferred_capability");
        let right = right_caps.get(robot).cloned().unwrap_or_default();
        diff_string_sets(
            MissionDiffDimension::Capability,
            &path,
            left,
            &right,
            "inferred capability",
            &mut report.changes,
        );
    }
    for robot in right_caps.keys() {
        if !left_caps.contains_key(robot) {
            diff_string_sets(
                MissionDiffDimension::Capability,
                &format!("robot/{robot}/inferred_capability"),
                &BTreeSet::new(),
                right_caps.get(robot).unwrap(),
                "inferred capability",
                &mut report.changes,
            );
        }
    }
    report.added = report
        .changes
        .iter()
        .filter(|c| c.kind == DiffChangeKind::Added)
        .count() as u32;
    report.removed = report
        .changes
        .iter()
        .filter(|c| c.kind == DiffChangeKind::Removed)
        .count() as u32;
    report.modified = report
        .changes
        .iter()
        .filter(|c| c.kind == DiffChangeKind::Modified)
        .count() as u32;
    report.has_deploy_impact = report.changes.iter().any(|c| {
        matches!(
            c.dimension,
            MissionDiffDimension::Deploy
                | MissionDiffDimension::Hardware
                | MissionDiffDimension::Capability
        )
    });
    report
}

fn capability_sets(program: &Program) -> BTreeMap<String, BTreeSet<String>> {
    infer_robot_capabilities(program)
        .into_iter()
        .map(|row| {
            let mut caps = row.declared;
            caps.extend(row.inferred);
            (row.robot, caps.into_iter().collect())
        })
        .collect()
}
