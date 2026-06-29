//! Static package import path catalogue for the type checker.
//!
//! This module contains the authoritative list of known import paths (builtin, std,
//! adapter, and registry) so that `spanda-typecheck` can validate `import` declarations
//! without depending on the `spanda-package` core-platform crate.

/// Built-in language import paths (stdlib and core modules).
pub fn builtin_import_paths() -> &'static [&'static str] {
    // Return the static list of built-in import paths.
    &[
        "sensors.lidar",
        "sensors.camera",
        "sensors.imu",
        "motion.drive",
        "motion.arm",
        "navigation.planning",
        "navigation.path_planning",
        "navigation.localize",
        "navigation.slam",
        "safety.validate",
        "ai.reasoning",
        "ai.openai",
        "robotics.ros2",
        "communication.mqtt",
        "vision.opencv",
        "vision.yolo",
        "vision.core",
        "manipulation.grasp",
        "hri.dialogue",
        "twin.sync",
        "sim.gazebo",
        "sim.webots",
        "ledger.mock",
        "provenance.core",
        "identity.core",
        "supply_chain.trace",
        "std.core",
        "std.time",
        "std.units",
        "std.spatial",
        "std.math",
        "std.collections",
        "std.result",
        "std.io",
        "std.log",
        "std.ai",
        "std.robotics",
        "std.sensors",
        "std.actuators",
        "std.safety",
        "std.communication",
        "std.hardware",
        "std.sim",
        "std.twin",
        "std.hri",
        "std.security",
        "std.audit",
        "std.crypto",
        "std.network",
    ]
}

/// Adapter framework import paths (nav2, slam, ROS2, AI, etc.).
pub fn adapter_import_paths() -> &'static [&'static str] {
    // Return the static list of adapter framework import paths.
    &[
        "ai.anthropic",
        "ai.onnx",
        "ai.openai",
        "alerting.escalation",
        "alerting.pagerduty",
        "alerting.slack",
        "alerting.teams",
        "assurance.anomaly",
        "assurance.continuity",
        "assurance.diagnosis",
        "assurance.evidence",
        "assurance.fusion",
        "assurance.knowledge",
        "assurance.mission",
        "assurance.prognostics",
        "assurance.resilience",
        "audit.siem",
        "automotive.ethernet",
        "automotive.lin",
        "automotive.uds",
        "automotive.v2x",
        "cloud.remote",
        "communication.dds",
        "communication.mqtt",
        "connectivity.ble",
        "connectivity.cellular",
        "connectivity.lte",
        "connectivity.wifi",
        "deploy.ota",
        "discovery.ble",
        "discovery.cellular",
        "discovery.mdns",
        "discovery.serial",
        "discovery.tls",
        "discovery.usb",
        "discovery.wifi",
        "hri.dialogue",
        "hri.eye",
        "hri.gesture",
        "hri.voice",
        "iot.canbus",
        "iot.command",
        "iot.device",
        "iot.lora",
        "iot.matter",
        "iot.modbus",
        "iot.opcua",
        "iot.shadow",
        "iot.telemetry",
        "iot.zigbee",
        "maintenance.health",
        "manipulation.grasp",
        "manipulation.moveit",
        "navigation.cartographer",
        "navigation.nav2",
        "navigation.path_planning",
        "navigation.rtabmap",
        "navigation.slam",
        "observability.grafana",
        "observability.otel",
        "positioning.gps",
        "provenance.ledger",
        "robotics.fleet",
        "robotics.ros2",
        "security.audit",
        "sensors.lidar",
        "sensors.radar",
        "sensors.ultrasonic",
        "sim.gazebo",
        "sim.webots",
        "spatial.arcore",
        "spatial.arkit",
        "spatial.hololens",
        "spatial.magic_leap",
        "spatial.openxr",
        "spatial.vision_pro",
        "trust.jetson",
        "trust.pi",
        "twin.sync",
        "vision.detectron",
        "vision.opencv",
        "vision.yolo",
        "wearable.bodycam",
        "wearable.industrial",
        "wearable.smartwatch",
    ]
}

/// Registry package import paths (official and community packages).
pub fn registry_import_paths() -> &'static [&'static str] {
    // Return the static list of registry import paths.
    &[
        "robotics.ros2",
        "vision.core",
        "navigation.path_planning",
        "communication.mqtt",
        "sensors.lidar.rplidar",
        "ai.openai",
        "vision.opencv",
        "vision.yolo",
        "navigation.slam",
        "navigation.nav2",
        "navigation.cartographer",
        "navigation.rtabmap",
        "ledger.mock",
        "identity.core",
        "provenance.core",
        "supply_chain.trace",
        "python.torch",
        "python.opencv",
        "cpp.ros2",
    ]
}

/// Return true if `path` is a known package import path.
///
/// Parameters:
/// - `path` — import path to look up
///
/// Returns:
/// true if the path is in any of the known import path sets.
///
/// Options:
/// None.
///
/// Example:
/// assert!(resolve_package_import("sensors.lidar"));
pub fn resolve_package_import(path: &str) -> bool {
    // Check each catalogue tier in order of specificity.
    if builtin_import_paths().contains(&path) {
        return true;
    }
    if adapter_import_paths().contains(&path) {
        return true;
    }
    registry_import_paths().contains(&path)
}
