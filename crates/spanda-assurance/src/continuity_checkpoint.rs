//! Durable mission checkpoint persistence for continuity handoffs across restarts.

use crate::continuity::MissionStateSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// On-disk checkpoint index keyed by `mission::robot`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContinuityCheckpointStore {
    pub entries: HashMap<String, MissionStateSnapshot>,
}

fn store_key(mission: &str, robot: &str) -> String {
    format!("{mission}::{robot}")
}

/// Default checkpoint store under the project `.spanda/` directory.
pub fn default_checkpoint_store_path() -> std::path::PathBuf {
    std::path::PathBuf::from(".spanda/mission-checkpoints.json")
}

/// Load persisted checkpoints from disk.
pub fn load_checkpoint_store(path: &Path) -> ContinuityCheckpointStore {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Persist checkpoint store to disk.
pub fn save_checkpoint_store(path: &Path, store: &ContinuityCheckpointStore) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(store).unwrap_or_default())
}

/// Record a snapshot for a mission/robot pair.
pub fn record_checkpoint(
    store: &mut ContinuityCheckpointStore,
    mission: &str,
    robot: &str,
    snapshot: MissionStateSnapshot,
) {
    store
        .entries
        .insert(store_key(mission, robot), snapshot);
}

/// Load a stored snapshot for a mission/robot pair.
pub fn load_checkpoint(
    store: &ContinuityCheckpointStore,
    mission: &str,
    robot: &str,
) -> Option<MissionStateSnapshot> {
    store.entries.get(&store_key(mission, robot)).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::continuity::MissionCheckpoint;
    use crate::types::MissionExecutionState;

    fn sample_snapshot(mission: &str, robot: &str, progress: f64) -> MissionStateSnapshot {
        MissionStateSnapshot {
            mission: mission.into(),
            completed_steps: vec!["navigate".into()],
            current_goal: Some("scan".into()),
            progress_percent: progress,
            checkpoints: vec![MissionCheckpoint {
                name: "checkpoint_scan".into(),
                progress_percent: progress,
                mission_state: MissionExecutionState {
                    plan: mission.into(),
                    current_step: Some("scan".into()),
                    status: "running".into(),
                },
                robot_state: format!("robot:{robot}"),
                health_state: "nominal".into(),
                safety_state: "validated".into(),
                capability_state: "matched".into(),
            }],
        }
    }

    #[test]
    fn checkpoint_store_round_trips_on_disk() {
        let path = std::env::temp_dir().join("spanda-continuity-checkpoints-test.json");
        let mut store = ContinuityCheckpointStore::default();
        record_checkpoint(
            &mut store,
            "WarehouseInventoryScan",
            "ScannerAlpha",
            sample_snapshot("WarehouseInventoryScan", "ScannerAlpha", 72.0),
        );
        save_checkpoint_store(&path, &store).expect("save checkpoints");
        let loaded = load_checkpoint_store(&path);
        let snap = load_checkpoint(&loaded, "WarehouseInventoryScan", "ScannerAlpha")
            .expect("checkpoint present");
        assert!((snap.progress_percent - 72.0).abs() < f64::EPSILON);
        let _ = std::fs::remove_file(path);
    }
}
