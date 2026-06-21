//! GPS/GNSS positioning and wireless connectivity types, verification, and simulation faults.
//!
use crate::foundations::{
    ConnectivityPolicyDecl, GeofenceDecl, RequiresConnectivityDecl, SimFaultDecl,
};
use crate::hardware::{CompatItem, CompatSeverity, HardwareProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Requirement level for a connectivity channel in `requires_connectivity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectivityRequirement {
    Required,
    Optional,
}

/// Positioning and navigation native type names.
pub fn positioning_types() -> &'static [&'static str] {
    &[
        "GpsFix",
        "GnssFix",
        "GeoPoint",
        "GeoFence",
        "Altitude",
        "Heading",
        "SpeedOverGround",
        "SatelliteInfo",
        "PositionAccuracy",
        "NavigationStatus",
    ]
}

/// Wireless and network connection type names.
pub fn connectivity_types() -> &'static [&'static str] {
    &[
        "WifiConnection",
        "BluetoothConnection",
        "BleConnection",
        "CellularConnection",
        "LTEConnection",
        "FourGConnection",
        "FiveGConnection",
        "EthernetConnection",
        "MeshConnection",
        "NetworkStatus",
        "SignalStrength",
        "Bandwidth",
        "Latency",
        "PacketLoss",
        "RoamingStatus",
    ]
}

/// Hardware profile connectivity option identifiers.
pub fn connectivity_options() -> &'static [&'static str] {
    &[
        "WiFi",
        "WiFi6",
        "Bluetooth",
        "Bluetooth5",
        "BLE",
        "LTE",
        "FourG",
        "4G",
        "FiveG",
        "5G",
        "Ethernet",
        "Mesh",
        "GPS",
        "GNSS",
        "Satellite",
    ]
}

/// Map a requires_connectivity key to profile connectivity tokens.
pub fn connectivity_key_to_profile_tokens(key: &str) -> Vec<&'static str> {
    match key {
        "gps" => vec!["GPS"],
        "gnss" => vec!["GNSS", "GPS"],
        "wifi" => vec!["WiFi", "WiFi6"],
        "bluetooth" => vec!["Bluetooth", "Bluetooth5", "BLE"],
        "cellular" => vec!["LTE", "FourG", "4G", "FiveG", "5G"],
        "ethernet" => vec!["Ethernet"],
        "mesh" => vec!["Mesh"],
        "satellite" => vec!["Satellite"],
        _ => vec![],
    }
}

/// Connectivity-related simulation fault names.
pub fn connectivity_faults() -> &'static [&'static str] {
    &[
        "GPSLost",
        "GpsFailure",
        "GpsDrift",
        "GpsSpoofing",
        "NetworkOutage",
        "NetworkLatencySpike",
        "WeakWifi",
        "LteOutage",
        "FiveGHandoff",
        "BluetoothDisconnect",
        "PacketLoss",
        "LatencySpike",
    ]
}

/// Security capabilities for positioning and connectivity.
pub fn connectivity_capabilities() -> &'static [&'static str] {
    &[
        "gps.read",
        "network.status",
        "wifi.connect",
        "bluetooth.scan",
        "bluetooth.pair",
        "cellular.connect",
        "network.failover",
    ]
}

fn pass(category: &str, message: impl Into<String>, line: u32, column: u32) -> CompatItem {
    CompatItem {
        category: category.into(),
        message: message.into(),
        severity: CompatSeverity::Pass,
        line,
        column,
    }
}

fn warn(category: &str, message: impl Into<String>, line: u32, column: u32) -> CompatItem {
    CompatItem {
        category: category.into(),
        message: message.into(),
        severity: CompatSeverity::Warning,
        line,
        column,
    }
}

fn error(category: &str, message: impl Into<String>, line: u32, column: u32) -> CompatItem {
    CompatItem {
        category: category.into(),
        message: message.into(),
        severity: CompatSeverity::Error,
        line,
        column,
    }
}

/// Verify `requires_connectivity` against a hardware profile's connectivity list and network metrics.
pub fn verify_requires_connectivity(
    req: &RequiresConnectivityDecl,
    profile: &HardwareProfile,
) -> Vec<CompatItem> {
    let RequiresConnectivityDecl::RequiresConnectivityDecl {
        channels,
        latency_ms_max,
        bandwidth_mbps_min,
        packet_loss_pct_max,
        span,
    } = req;
    let mut items = Vec::new();
    let line = span.start.line;
    let column = span.start.column;
    let profile_set: HashSet<String> = profile.connectivity.iter().cloned().collect();

    for (key, level) in channels {
        if *level != ConnectivityRequirement::Required {
            continue;
        }
        let tokens = connectivity_key_to_profile_tokens(key);
        if tokens.is_empty() {
            items.push(warn(
                "connectivity",
                format!("Unknown connectivity key '{key}' in requires_connectivity"),
                line,
                column,
            ));
            continue;
        }
        let satisfied = tokens.iter().any(|t| profile_set.contains(*t));
        if satisfied {
            items.push(pass(
                "connectivity",
                format!(
                    "Required connectivity '{key}' present on '{}'",
                    profile.name
                ),
                line,
                column,
            ));
        } else {
            items.push(error(
                "connectivity",
                format!(
                    "Required connectivity '{key}' not on '{}' [{}]",
                    profile.name,
                    profile.connectivity.join(", ")
                ),
                line,
                column,
            ));
        }
    }

    if let Some(min_bw) = bandwidth_mbps_min {
        match profile.network_bandwidth_mbps {
            Some(bw) if bw >= *min_bw => items.push(pass(
                "connectivity",
                format!("Bandwidth {bw} Mbps meets connectivity requirement >= {min_bw} Mbps"),
                line,
                column,
            )),
            Some(bw) => items.push(error(
                "connectivity",
                format!(
                    "Connectivity bandwidth requirement {min_bw} Mbps exceeds target {bw} Mbps"
                ),
                line,
                column,
            )),
            None => items.push(warn(
                "connectivity",
                "Target bandwidth unknown — cannot verify connectivity bandwidth requirement",
                line,
                column,
            )),
        }
    }

    if let Some(max_lat) = latency_ms_max {
        match profile.network_latency_ms {
            Some(lat) if lat <= *max_lat => items.push(pass(
                "connectivity",
                format!("Latency {lat} ms meets connectivity requirement <= {max_lat} ms"),
                line,
                column,
            )),
            Some(lat) => items.push(error(
                "connectivity",
                format!(
                    "Connectivity latency requirement {max_lat} ms exceeded by target {lat} ms"
                ),
                line,
                column,
            )),
            None => items.push(warn(
                "connectivity",
                "Target latency unknown — cannot verify connectivity latency requirement",
                line,
                column,
            )),
        }
    }

    if let Some(max_loss) = packet_loss_pct_max {
        match profile.packet_loss_pct {
            Some(loss) if loss <= *max_loss => items.push(pass(
                "connectivity",
                format!("Packet loss {loss}% meets requirement <= {max_loss}%"),
                line,
                column,
            )),
            Some(loss) => items.push(error(
                "connectivity",
                format!("Packet loss {loss}% exceeds requirement <= {max_loss}%"),
                line,
                column,
            )),
            None => items.push(warn(
                "connectivity",
                "Target packet loss unknown — cannot verify packet_loss requirement",
                line,
                column,
            )),
        }
    }

    items
}

/// Apply a connectivity or positioning simulation fault to a hardware profile.
pub fn apply_connectivity_fault(
    mut profile: HardwareProfile,
    fault: &SimFaultDecl,
) -> HardwareProfile {
    match fault.fault_type.as_str() {
        "GPSLost" | "GpsFailure" => {
            profile.sensors.retain(|s| s != "GPS" && s != "GNSS");
            profile.connectivity.retain(|c| c != "GPS" && c != "GNSS");
        }
        "GpsDrift" | "GpsSpoofing" => {}
        "NetworkOutage" | "LteOutage" => {
            profile.network_bandwidth_mbps = Some(0.0);
            profile.network_latency_ms = Some(10_000.0);
            profile.connectivity.retain(|c| {
                !matches!(
                    c.as_str(),
                    "WiFi"
                        | "WiFi6"
                        | "LTE"
                        | "FourG"
                        | "4G"
                        | "FiveG"
                        | "5G"
                        | "Ethernet"
                        | "Mesh"
                )
            });
        }
        "WeakWifi" => {
            profile.network_bandwidth_mbps = Some(1.0);
            profile.network_latency_ms = Some(500.0);
        }
        "NetworkLatencySpike" | "LatencySpike" => {
            profile.network_latency_ms = Some(2000.0);
        }
        "FiveGHandoff" => {
            profile.network_latency_ms = Some(150.0);
        }
        "BluetoothDisconnect" => {
            profile
                .connectivity
                .retain(|c| !matches!(c.as_str(), "Bluetooth" | "Bluetooth5" | "BLE"));
        }
        "PacketLoss" => {
            profile.packet_loss_pct = Some(10.0);
        }
        _ => {}
    }
    profile
}

/// Validate geofence declaration geometry.
pub fn validate_geofence(geofence: &GeofenceDecl) -> Vec<CompatItem> {
    let GeofenceDecl::GeofenceDecl {
        name,
        center_lat,
        center_lon,
        radius_m,
        span,
    } = geofence;
    let mut items = Vec::new();
    let line = span.start.line;
    let column = span.start.column;

    if !(-90.0..=90.0).contains(center_lat) {
        items.push(error(
            "geofence",
            format!("Geofence '{name}' center latitude {center_lat} out of range [-90, 90]"),
            line,
            column,
        ));
    } else if !(-180.0..=180.0).contains(center_lon) {
        items.push(error(
            "geofence",
            format!("Geofence '{name}' center longitude {center_lon} out of range [-180, 180]"),
            line,
            column,
        ));
    } else if *radius_m <= 0.0 {
        items.push(error(
            "geofence",
            format!("Geofence '{name}' radius must be positive"),
            line,
            column,
        ));
    } else {
        items.push(pass(
            "geofence",
            format!("Geofence '{name}' geometry valid"),
            line,
            column,
        ));
    }
    items
}

/// Validate connectivity failover policy link names.
pub fn validate_connectivity_policy(policy: &ConnectivityPolicyDecl) -> Vec<CompatItem> {
    let ConnectivityPolicyDecl::ConnectivityPolicyDecl {
        name,
        preferred,
        fallback,
        emergency,
        span,
        ..
    } = policy;
    let line = span.start.line;
    let column = span.start.column;
    let mut items = vec![pass(
        "connectivity_policy",
        format!("Connectivity policy '{name}' parsed: preferred={preferred}, fallback={fallback}"),
        line,
        column,
    )];
    if preferred == fallback {
        items.push(warn(
            "connectivity_policy",
            format!("Policy '{name}' preferred and fallback are the same link"),
            line,
            column,
        ));
    }
    if let Some(em) = emergency {
        if em == preferred || em == fallback {
            items.push(warn(
                "connectivity_policy",
                format!("Policy '{name}' emergency link duplicates preferred or fallback"),
                line,
                column,
            ));
        }
    }
    items
}
