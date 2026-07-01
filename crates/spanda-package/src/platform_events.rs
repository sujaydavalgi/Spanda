//! Platform event emission for package manager lifecycle hooks.
//!
use spanda_audit::platform_event::names;
use spanda_audit::PlatformEvent;
use spanda_runtime::publish_platform_event;
use serde_json::json;

/// Record `PackageVerified` after adapter verification succeeds.
pub fn record_package_verified(name: &str, version: &str, import_path: Option<&str>) {
    let event = PlatformEvent::new(
        names::PACKAGE_VERIFIED,
        "spanda-package",
        json!({
            "name": name,
            "version": version,
            "import_path": import_path,
            "verification": "adapter",
        }),
    )
    .with_entity_id(format!("package/{name}"));
    publish_platform_event(None, &event);
}

/// Record `PackageInstalled` for each dependency written to the lockfile.
pub fn record_packages_installed(packages: &[(String, String)]) {
    for (name, version) in packages {
        let event = PlatformEvent::new(
            names::PACKAGE_INSTALLED,
            "spanda-package",
            json!({
                "name": name,
                "version": version,
                "provenance": "lockfile_resolve",
            }),
        )
        .with_entity_id(format!("package/{name}"));
        publish_platform_event(None, &event);
    }
}

/// Record `PackageRemoved` when a dependency is removed from the project manifest.
pub fn record_package_removed(name: &str) {
    let event = PlatformEvent::new(
        names::PACKAGE_REMOVED,
        "spanda-package",
        json!({
            "name": name,
        }),
    )
    .with_entity_id(format!("package/{name}"));
    publish_platform_event(None, &event);
}
