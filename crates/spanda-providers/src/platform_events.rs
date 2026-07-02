//! Platform event emission when official packages bootstrap providers.
//!
use serde_json::json;
use spanda_audit::platform_event::names;
use spanda_audit::PlatformEvent;
use spanda_runtime::publish_platform_event;

/// Record `PackageInstalled` events for each bootstrapped official package.
pub fn record_packages_installed(package_names: &[&str]) {
    for name in package_names {
        let event = PlatformEvent::new(
            names::PACKAGE_INSTALLED,
            "spanda-providers",
            json!({
                "name": name,
                "provenance": "official_bootstrap",
            }),
        )
        .with_entity_id(format!("package/{name}"));
        publish_platform_event(None, &event);
    }
}
