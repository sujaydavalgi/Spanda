//! Operational drift rollup across enterprise dimensions.
//!
use crate::drift::{detect_config_drift, ConfigDriftReport, DriftDimension, DriftFinding};
use crate::resolved::ResolvedSystemConfig;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Enterprise drift dimensions for Control Center and SDK consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalDriftDimension {
    Configuration,
    Firmware,
    Package,
    Provider,
    Capability,
    Policy,
    Safety,
}

/// Rollup of drift findings by operational dimension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalDriftReport {
    pub passed: bool,
    pub findings: Vec<DriftFinding>,
    pub by_dimension: BTreeMap<String, u32>,
    pub dimensions_checked: Vec<String>,
}

/// Detect drift between baseline and current resolved configuration.
pub fn detect_operational_drift(
    baseline: &ResolvedSystemConfig,
    current: &ResolvedSystemConfig,
) -> OperationalDriftReport {
    let mut report = detect_config_drift(baseline, current);
    append_policy_safety_drift(baseline, current, &mut report);
    rollup_operational(report)
}

fn append_policy_safety_drift(
    baseline: &ResolvedSystemConfig,
    current: &ResolvedSystemConfig,
    report: &mut ConfigDriftReport,
) {
    let base_assurance = crate::assurance_policy(baseline);
    let cur_assurance = crate::assurance_policy(current);
    if base_assurance.minimum_score != cur_assurance.minimum_score {
        report.push(DriftFinding {
            dimension: DriftDimension::Configuration,
            severity: crate::drift::DriftSeverity::High,
            message: format!(
                "assurance.minimum_score changed: {} -> {}",
                base_assurance.minimum_score, cur_assurance.minimum_score
            ),
            path: Some("assurance.minimum_score".into()),
        });
    }
    let base_mission = crate::mission_policy(baseline);
    let cur_mission = crate::mission_policy(current);
    if base_mission.required_capabilities != cur_mission.required_capabilities {
        report.push(DriftFinding {
            dimension: DriftDimension::Program,
            severity: crate::drift::DriftSeverity::Medium,
            message: "mission required_capabilities changed".into(),
            path: Some("mission.required_capabilities".into()),
        });
    }
}

fn rollup_operational(report: ConfigDriftReport) -> OperationalDriftReport {
    let mut by_dimension: BTreeMap<String, u32> = BTreeMap::new();
    for finding in &report.findings {
        let dim = map_dimension(finding.dimension);
        *by_dimension
            .entry(operational_dimension_name(dim).to_string())
            .or_insert(0) += 1;
    }
    let dimensions_checked: Vec<String> = [
        OperationalDriftDimension::Configuration,
        OperationalDriftDimension::Firmware,
        OperationalDriftDimension::Package,
        OperationalDriftDimension::Provider,
        OperationalDriftDimension::Capability,
        OperationalDriftDimension::Policy,
        OperationalDriftDimension::Safety,
    ]
    .iter()
    .map(|d| operational_dimension_name(*d).to_string())
    .collect();
    OperationalDriftReport {
        passed: report.passed,
        findings: report.findings,
        by_dimension,
        dimensions_checked,
    }
}

fn operational_dimension_name(dim: OperationalDriftDimension) -> &'static str {
    match dim {
        OperationalDriftDimension::Configuration => "configuration",
        OperationalDriftDimension::Firmware => "firmware",
        OperationalDriftDimension::Package => "package",
        OperationalDriftDimension::Provider => "provider",
        OperationalDriftDimension::Capability => "capability",
        OperationalDriftDimension::Policy => "policy",
        OperationalDriftDimension::Safety => "safety",
    }
}

fn map_dimension(dim: DriftDimension) -> OperationalDriftDimension {
    match dim {
        DriftDimension::Firmware => OperationalDriftDimension::Firmware,
        DriftDimension::Package => OperationalDriftDimension::Package,
        DriftDimension::Provider => OperationalDriftDimension::Provider,
        DriftDimension::Program | DriftDimension::Hardware | DriftDimension::Mapping => {
            OperationalDriftDimension::Capability
        }
        DriftDimension::Fleet | DriftDimension::Device | DriftDimension::Configuration => {
            OperationalDriftDimension::Configuration
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::ConfigResolver;
    use std::path::PathBuf;

    #[test]
    fn identical_configs_pass() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/warehouse");
        let resolved = ConfigResolver::new()
            .resolve_from_dir(&root)
            .expect("warehouse fixture");
        let report = detect_operational_drift(&resolved, &resolved);
        assert!(report.passed);
        assert_eq!(report.dimensions_checked.len(), 7);
    }
}
