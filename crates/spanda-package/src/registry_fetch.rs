//! Download and extract registry package tarballs from `SPANDA_REGISTRY_URL`.

use crate::registry_remote::registry_base_url;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn registry_tarball_url(name: &str, version: &str) -> Option<String> {
    registry_base_url().map(|base| format!("{base}/packages/{name}/{version}"))
}

pub fn fetch_registry_tarball(name: &str, version: &str, dest: &Path) -> Result<PathBuf, String> {
    let url = registry_tarball_url(name, version)
        .ok_or_else(|| "SPANDA_REGISTRY_URL is not set".to_string())?;
    fs::create_dir_all(dest).map_err(|e| format!("create vendor dir: {e}"))?;
    let tarball = dest.join(format!("{name}-{version}.tar.gz"));
    download_file(&url, &tarball)?;
    extract_tarball(&tarball, dest)?;
    let _ = fs::remove_file(&tarball);
    Ok(dest.to_path_buf())
}

fn download_file(url: &str, output: &Path) -> Result<(), String> {
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

fn extract_tarball(tarball: &Path, dest: &Path) -> Result<(), String> {
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
}
