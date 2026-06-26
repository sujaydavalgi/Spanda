//! Configuration snapshot versioning and rollback metadata.
//!
use crate::error::{ConfigError, ConfigResult};
use crate::resolved::ResolvedSystemConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Default directory for persisted configuration snapshots.
pub fn default_snapshots_dir() -> PathBuf {
    PathBuf::from(".spanda/config-snapshots")
}

/// Metadata for a stored configuration snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigSnapshotMeta {
    pub id: String,
    pub created_at_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub project_name: String,
    pub device_count: usize,
}

/// Full snapshot including resolved configuration JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigSnapshot {
    pub meta: ConfigSnapshotMeta,
    pub resolved: ResolvedSystemConfig,
}

fn io_error(path: impl Into<PathBuf>, source: std::io::Error) -> ConfigError {
    ConfigError::Io {
        path: path.into(),
        source,
    }
}

/// Save a resolved configuration snapshot to disk.
pub fn save_config_snapshot(
    resolved: &ResolvedSystemConfig,
    dir: &Path,
    label: Option<String>,
) -> ConfigResult<ConfigSnapshotMeta> {
    fs::create_dir_all(dir).map_err(|e| io_error(dir, e))?;
    let id = format!("cfg-{}", now_ms().to_string().replace('.', ""));
    let meta = ConfigSnapshotMeta {
        id: id.clone(),
        created_at_ms: now_ms(),
        label,
        project_name: resolved.project_name().to_string(),
        device_count: resolved.device_registry.devices.len(),
    };
    let snapshot = ConfigSnapshot {
        meta: meta.clone(),
        resolved: resolved.clone(),
    };
    let path = dir.join(format!("{id}.json"));
    let text = serde_json::to_string_pretty(&snapshot).map_err(|e| ConfigError::JsonParse {
        path: path.clone(),
        source: e,
    })?;
    fs::write(&path, text).map_err(|e| io_error(&path, e))?;
    Ok(meta)
}

/// List snapshot metadata from a snapshots directory.
pub fn list_config_snapshots(dir: &Path) -> ConfigResult<Vec<ConfigSnapshotMeta>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut items = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| io_error(dir, e))? {
        let entry = entry.map_err(|e| io_error(dir, e))?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| io_error(&path, e))?;
        if let Ok(snapshot) = serde_json::from_str::<ConfigSnapshot>(&text) {
            items.push(snapshot.meta);
        }
    }
    items.sort_by(|a, b| b.created_at_ms.partial_cmp(&a.created_at_ms).unwrap());
    Ok(items)
}

/// Load a snapshot by id.
pub fn load_config_snapshot(dir: &Path, id: &str) -> ConfigResult<ConfigSnapshot> {
    let path = dir.join(format!("{id}.json"));
    let text = fs::read_to_string(&path).map_err(|e| io_error(&path, e))?;
    serde_json::from_str(&text).map_err(|e| ConfigError::JsonParse { path, source: e })
}

fn now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_missing_dir_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nested");
        assert!(list_config_snapshots(&missing).unwrap().is_empty());
    }
}
