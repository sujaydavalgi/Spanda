//! Static security capability catalogue for the type checker.
//!
//! This module contains the authoritative list of known platform capabilities so
//! that `spanda-typecheck` can validate `@require_capability` declarations without
//! depending on the `spanda-security` platform-services crate.

/// Return the full list of known capability identifiers.
///
/// Parameters:
/// None.
///
/// Returns:
/// Static slice of capability identifier strings.
///
/// Options:
/// None.
///
/// Example:
/// assert!(known_capabilities().contains(&"network.outbound"));
pub fn known_capabilities() -> &'static [&'static str] {
    // Return the static list of known capability strings mirrored from spanda-security.
    &[
        "network.outbound",
        "network.inbound",
        "camera.read",
        "lidar.read",
        "imu.read",
        "gps.read",
        "network.status",
        "wifi.connect",
        "bluetooth.scan",
        "bluetooth.pair",
        "cellular.connect",
        "network.failover",
        "motion.propose",
        "actuator.execute",
        "actuator.execute.safe",
        "serial.port",
        "storage.read",
        "storage.write",
        "ai.inference",
        "ros2.publish",
        "ros2.subscribe",
        "audit.write",
        "audit.read",
        "identity.sign",
        "identity.verify",
        "identity.read",
        "ledger.anchor",
        "crypto.encrypt",
        "crypto.decrypt",
        "crypto.sign",
        "crypto.verify",
        "secret.read",
        "secure_topic.publish",
        "secure_topic.subscribe",
        "positioning.read",
        "mqtt.publish",
        "mqtt.subscribe",
        "connectivity.wifi",
        "connectivity.ble",
        "connectivity.cellular",
        "navigation.plan",
        "fleet.orchestrate",
        "slam.localize",
        "slam.map",
        "deploy.rollout",
        "deploy.rollback",
        "dds.publish",
        "dds.subscribe",
        "ai.invoke",
        "vision.detect",
        "simulation.step",
        "cloud.invoke",
        "audit.append",
        "maintenance.health",
        "manipulation.plan",
    ]
}

/// Return true if `cap` is a recognised platform capability identifier.
///
/// Parameters:
/// - `cap` — capability string to look up
///
/// Returns:
/// true if the capability is in the known list.
///
/// Options:
/// None.
///
/// Example:
/// assert!(is_known_capability("network.outbound"));
pub fn is_known_capability(cap: &str) -> bool {
    // Check membership in the static capability list.
    known_capabilities().contains(&cap)
}
