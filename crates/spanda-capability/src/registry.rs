//! Known robot capabilities and their minimum hardware/package requirements.

use serde::{Deserialize, Serialize};

/// Severity when a required capability is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerificationSeverity {
    Error,
    Warning,
    Info,
}

/// Minimum requirement for a single capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub any_of_sensors: Vec<String>,
    pub any_of_actuators: Vec<String>,
    pub any_of_connectivity: Vec<String>,
    pub required_packages: Vec<String>,
    pub required_providers: Vec<String>,
    pub required_safety_rules: Vec<String>,
    pub required_security: Vec<String>,
    pub severity: VerificationSeverity,
}

/// Full definition of a known capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityDefinition {
    pub name: String,
    pub description: String,
    pub minimum: CapabilityRequirement,
    pub optional_sensors: Vec<String>,
    pub optional_packages: Vec<String>,
}

/// Package contribution to the capability registry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageCapabilityContribution {
    pub package: String,
    pub capabilities: Vec<String>,
}

/// Built-in capability registry entries.
pub fn capability_registry() -> Vec<CapabilityDefinition> {
    vec![
        def(
            "autonomous_navigation",
            "Plan and execute paths without human intervention",
            req(
                &["Lidar", "Camera", "DepthCamera"],
                &["DifferentialDrive", "AckermannDrive"],
                &[],
                &["spanda-nav"],
                &["NavigationProvider"],
                &["max_speed", "stop_if"],
                &[],
                VerificationSeverity::Error,
            ),
            &["Radar"],
            &["spanda-slam"],
        ),
        def(
            "gps_navigation",
            "Navigate using GPS/GNSS positioning",
            req(
                &["GPS", "GNSS"],
                &["DifferentialDrive"],
                &["WiFi", "LTE", "FiveG"],
                &["spanda-gps"],
                &["PositioningProvider"],
                &[],
                &[],
                VerificationSeverity::Error,
            ),
            &[],
            &[],
        ),
        def(
            "obstacle_avoidance",
            "Detect and avoid obstacles during motion",
            req(
                &["Lidar", "DepthCamera", "Radar"],
                &["DifferentialDrive"],
                &[],
                &["spanda-nav"],
                &["NavigationProvider"],
                &["stop_if"],
                &[],
                VerificationSeverity::Error,
            ),
            &["Camera"],
            &["spanda-vision"],
        ),
        def(
            "remote_control",
            "Accept signed remote commands over network",
            req(
                &[],
                &["DifferentialDrive"],
                &["WiFi", "LTE", "FiveG", "Bluetooth"],
                &[],
                &[],
                &[],
                &["signed_commands"],
                VerificationSeverity::Error,
            ),
            &[],
            &["spanda-mqtt"],
        ),
        def(
            "telemetry_streaming",
            "Stream robot state and sensor data remotely",
            req(
                &[],
                &[],
                &["WiFi", "LTE", "FiveG", "MQTT"],
                &["spanda-mqtt"],
                &["TransportProvider"],
                &[],
                &[],
                VerificationSeverity::Warning,
            ),
            &["GPS", "Camera"],
            &["spanda-cloud"],
        ),
        def(
            "emergency_stop",
            "Immediate actuator halt via hardware or kill switch",
            req(
                &[],
                &["DifferentialDrive"],
                &[],
                &[],
                &[],
                &["emergency_stop", "kill_switch"],
                &[],
                VerificationSeverity::Error,
            ),
            &[],
            &[],
        ),
        def(
            "local_ai_inference",
            "Run AI models on onboard compute",
            req(
                &["Camera"],
                &[],
                &[],
                &[],
                &[],
                &["ai.validate"],
                &[],
                VerificationSeverity::Warning,
            ),
            &["Lidar"],
            &[],
        ),
        def(
            "vision_processing",
            "Capture and process camera frames",
            req(
                &["Camera"],
                &[],
                &[],
                &["spanda-opencv", "spanda-yolo"],
                &["VisionProvider"],
                &[],
                &[],
                VerificationSeverity::Warning,
            ),
            &[],
            &[],
        ),
        def(
            "manipulation",
            "Plan and execute arm/gripper motions",
            req(
                &["Camera"],
                &["Arm", "Gripper"],
                &[],
                &["spanda-moveit"],
                &[],
                &["max_force"],
                &[],
                VerificationSeverity::Error,
            ),
            &["ForceTorque"],
            &[],
        ),
        def(
            "fleet_coordination",
            "Coordinate multiple robots in a fleet",
            req(
                &[],
                &[],
                &["WiFi", "LTE"],
                &["spanda-fleet"],
                &["FleetProvider"],
                &[],
                &[],
                VerificationSeverity::Warning,
            ),
            &[],
            &[],
        ),
        def(
            "ota_update",
            "Over-the-air firmware and package updates",
            req(
                &[],
                &[],
                &["WiFi", "LTE"],
                &["spanda-ota"],
                &[],
                &[],
                &["signed_commands"],
                VerificationSeverity::Warning,
            ),
            &[],
            &[],
        ),
        def(
            "secure_communication",
            "Encrypted and authenticated messaging",
            req(
                &[],
                &[],
                &["WiFi", "LTE"],
                &[],
                &["CryptoProvider"],
                &[],
                &["signed_commands", "encrypted_transport"],
                VerificationSeverity::Error,
            ),
            &[],
            &[],
        ),
    ]
}

/// Package contributions to the registry.
pub fn package_contributions() -> Vec<PackageCapabilityContribution> {
    vec![
        contrib(
            "spanda-nav",
            &[
                "autonomous_navigation",
                "path_planning",
                "obstacle_avoidance",
            ],
        ),
        contrib("spanda-gps", &["gps_navigation", "geofencing"]),
        contrib("spanda-mqtt", &["telemetry_streaming", "remote_command"]),
        contrib("spanda-fleet", &["fleet_coordination"]),
        contrib("spanda-ota", &["ota_update"]),
        contrib("spanda-opencv", &["vision_processing"]),
        contrib("spanda-yolo", &["vision_processing", "local_ai_inference"]),
        contrib("spanda-moveit", &["manipulation"]),
        contrib("spanda-cloud", &["telemetry_streaming"]),
        contrib("spanda-slam", &["autonomous_navigation"]),
    ]
}

/// Look up a capability by name.
pub fn lookup_capability(name: &str) -> Option<CapabilityDefinition> {
    capability_registry().into_iter().find(|c| c.name == name)
}

/// Map sensor/actuator types to hardware-level capabilities.
pub fn sensor_capabilities(sensor_type: &str) -> Vec<&'static str> {
    match sensor_type {
        "GPS" | "GNSS" => vec!["read_location", "read_altitude", "read_heading"],
        "Camera" => vec!["capture_image", "stream_video", "detect_motion"],
        "Lidar" => vec!["scan_range", "obstacle_detection"],
        "DepthCamera" => vec!["depth_map", "obstacle_detection"],
        "Radar" => vec!["range_detection", "obstacle_detection"],
        "IMU" => vec!["read_orientation", "read_acceleration"],
        _ => vec!["read"],
    }
}

/// Map actuator types to hardware-level capabilities.
pub fn actuator_capabilities(actuator_type: &str) -> Vec<&'static str> {
    match actuator_type {
        "DifferentialDrive" | "AckermannDrive" => {
            vec!["move_forward", "rotate", "stop", "emergency_stop"]
        }
        "Arm" => vec!["move_joint", "stop", "emergency_stop"],
        "Gripper" => vec!["open", "close", "stop"],
        _ => vec!["execute", "stop"],
    }
}

fn def(
    name: &str,
    description: &str,
    minimum: CapabilityRequirement,
    optional_sensors: &[&str],
    optional_packages: &[&str],
) -> CapabilityDefinition {
    CapabilityDefinition {
        name: name.into(),
        description: description.into(),
        minimum,
        optional_sensors: optional_sensors.iter().map(|s| (*s).into()).collect(),
        optional_packages: optional_packages.iter().map(|s| (*s).into()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn req(
    sensors: &[&str],
    actuators: &[&str],
    connectivity: &[&str],
    packages: &[&str],
    providers: &[&str],
    safety: &[&str],
    security: &[&str],
    severity: VerificationSeverity,
) -> CapabilityRequirement {
    CapabilityRequirement {
        any_of_sensors: sensors.iter().map(|s| (*s).into()).collect(),
        any_of_actuators: actuators.iter().map(|s| (*s).into()).collect(),
        any_of_connectivity: connectivity.iter().map(|s| (*s).into()).collect(),
        required_packages: packages.iter().map(|s| (*s).into()).collect(),
        required_providers: providers.iter().map(|s| (*s).into()).collect(),
        required_safety_rules: safety.iter().map(|s| (*s).into()).collect(),
        required_security: security.iter().map(|s| (*s).into()).collect(),
        severity,
    }
}

fn contrib(package: &str, capabilities: &[&str]) -> PackageCapabilityContribution {
    PackageCapabilityContribution {
        package: package.into(),
        capabilities: capabilities.iter().map(|c| (*c).into()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_core_capabilities() {
        assert!(lookup_capability("gps_navigation").is_some());
        assert!(lookup_capability("emergency_stop").is_some());
        assert!(lookup_capability("nonexistent").is_none());
    }

    #[test]
    fn sensor_capability_mapping() {
        let caps = sensor_capabilities("GPS");
        assert!(caps.contains(&"read_location"));
    }
}
