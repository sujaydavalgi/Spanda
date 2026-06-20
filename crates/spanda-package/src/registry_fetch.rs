//! Download and extract registry package tarballs.
//!
//! Resolution order: local `dist/` or `.spanda/registry/` tarballs, then
//! `SPANDA_REGISTRY_URL` (supports `https://` and `file://` bases).

use crate::registry_remote::registry_base_url;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn registry_tarball_url(name: &str, version: &str) -> Option<String> {
    registry_base_url().map(|base| format!("{base}/packages/{name}/{version}"))
}

pub fn resolve_local_tarball(
    project_root: &Path,
    name: &str,
    version: &str,
) -> Option<PathBuf> {
    let mut candidates = vec![
        project_root
            .join("dist")
            .join(format!("{name}-{version}.tar.gz")),
        project_root
            .join(".spanda/registry")
            .join(format!("{name}-{version}.tar.gz")),
    ];
    if let Ok(local) = std::env::var("SPANDA_REGISTRY_LOCAL") {
        let trimmed = local.trim();
        if !trimmed.is_empty() {
            candidates.push(PathBuf::from(trimmed).join(format!("{name}-{version}.tar.gz")));
        }
    }
    candidates.into_iter().find(|path| path.is_file())
}

pub fn fetch_registry_tarball(
    project_root: &Path,
    name: &str,
    version: &str,
    dest: &Path,
) -> Result<PathBuf, String> {
    fs::create_dir_all(dest).map_err(|e| format!("create vendor dir: {e}"))?;

    if let Some(local) = resolve_local_tarball(project_root, name, version) {
        extract_tarball(&local, dest)?;
        return Ok(dest.to_path_buf());
    }

    let url = registry_tarball_url(name, version).ok_or_else(|| {
        format!(
            "no tarball for '{name}@{version}' — run spanda publish (dist/) or set SPANDA_REGISTRY_URL"
        )
    })?;
    let tarball = dest.join(format!("{name}-{version}.tar.gz"));
    fetch_url_to_file(&url, &tarball)?;
    extract_tarball(&tarball, dest)?;
    let _ = fs::remove_file(&tarball);
    Ok(dest.to_path_buf())
}

pub fn fetch_url_to_file(url: &str, output: &Path) -> Result<(), String> {
    if let Some(path) = file_url_path(url) {
        fs::copy(&path, output).map_err(|e| format!("copy {path:?} to vendor: {e}"))?;
        return Ok(());
    }
    let status = Command::new("curl")
        .args(["-fsSL", url, "-o"])
        .arg(output)
        .status()
        .map_err(|e| format!("curl failed for {url}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("curl exited with {status} for {url}"))
    }
}

pub fn file_url_path(url: &str) -> Option<PathBuf> {
    let path = url.strip_prefix("file://")?;
    if path.is_empty() {
        return None;
    }
    Some(PathBuf::from(path))
}

pub fn extract_tarball(tarball: &Path, dest: &Path) -> Result<(), String> {
    let status = Command::new("tar")
        .args(["-xzf"])
        .arg(tarball)
        .arg("-C")
        .arg(dest)
        .status()
        .map_err(|e| format!("tar extract failed: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("tar exited with {status}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tarball_url_requires_registry_env() {
        std::env::remove_var("SPANDA_REGISTRY_URL");
        assert!(registry_tarball_url("demo", "0.1.0").is_none());
    }

    #[test]
    fn tarball_url_uses_base_path() {
        std::env::set_var("SPANDA_REGISTRY_URL", "https://registry.example.com");
        assert_eq!(
            registry_tarball_url("spanda-mqtt", "1.0.0"),
            Some("https://registry.example.com/packages/spanda-mqtt/1.0.0".into())
        );
        std::env::remove_var("SPANDA_REGISTRY_URL");
    }

    #[test]
    fn file_url_path_parses_local_paths() {
        assert_eq!(
            file_url_path("file:///tmp/registry/index.json"),
            Some(PathBuf::from("/tmp/registry/index.json"))
        );
    }

    #[test]
    fn resolve_local_tarball_finds_dist_bundle() {
        let root = std::env::temp_dir().join(format!("spanda-fetch-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("dist")).unwrap();
        let bundle = root.join("dist/demo-0.1.0.tar.gz");
        fs::write(&bundle, b"not a real tar").unwrap();
        assert_eq!(
            resolve_local_tarball(&root, "demo", "0.1.0"),
            Some(bundle)
        );
        let _ = fs::remove_dir_all(&root);
    }
}
