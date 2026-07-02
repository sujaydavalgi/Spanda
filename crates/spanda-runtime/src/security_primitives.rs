//! Bus security parsing helpers without transport or security crate dependencies.
//!
use crate::security_types::{
    AuthenticationMode, BusTransportSecurity, EncryptionMode, IntegrityMode, SecureCommPolicy,
    TrustBoundaryKind,
};

/// Parse bus declaration security fields into a transport security config.
pub fn bus_security_from_fields(
    encryption: Option<&str>,
    authentication: Option<&str>,
    integrity: Option<&str>,
) -> Result<BusTransportSecurity, String> {
    Ok(BusTransportSecurity {
        encryption: parse_optional_mode(encryption, EncryptionMode::None)?,
        authentication: parse_optional_mode(authentication, AuthenticationMode::None)?,
        integrity: parse_optional_mode(integrity, IntegrityMode::None)?,
        cert_path: None,
        key_secret: None,
        key_path: None,
    })
}

fn parse_optional_mode<T>(value: Option<&str>, default: T) -> Result<T, String>
where
    T: std::str::FromStr<Err = String>,
{
    match value {
        Some(raw) => raw.parse(),
        None => Ok(default),
    }
}

/// Merge robot `secure_comm` policy over bus defaults.
pub fn effective_bus_security(
    robot: &SecureCommPolicy,
    bus: &BusTransportSecurity,
) -> BusTransportSecurity {
    BusTransportSecurity {
        encryption: if robot.encryption != EncryptionMode::None {
            robot.encryption
        } else {
            bus.encryption
        },
        authentication: if robot.authentication != AuthenticationMode::None {
            robot.authentication
        } else {
            bus.authentication
        },
        integrity: if robot.integrity != IntegrityMode::None {
            robot.integrity
        } else {
            bus.integrity
        },
        cert_path: bus.cert_path.clone(),
        key_secret: bus.key_secret.clone(),
        key_path: bus.key_path.clone(),
    }
}

/// Map a transport name to the trust boundary it typically crosses.
pub fn boundary_for_transport_name(transport: &str) -> Option<TrustBoundaryKind> {
    match transport {
        "mqtt" | "websocket" | "dds" | "ros2" => Some(TrustBoundaryKind::RobotToCloud),
        "sim" | "in_memory" => None,
        _ => Some(TrustBoundaryKind::RobotToRobot),
    }
}

/// Validate bus security against a transport name.
pub fn validate_bus_security(
    security: &BusTransportSecurity,
    transport: &str,
) -> Result<(), String> {
    if security.encryption == EncryptionMode::Required
        && security.cert_path.is_none()
        && matches!(transport, "mqtt" | "websocket" | "dds")
    {
        return Err(format!(
            "transport '{transport}' requires encryption but no certificate path is configured"
        ));
    }
    Ok(())
}

/// Resolve broker URL from declaration or environment placeholders.
pub fn resolve_broker_url(broker_url: Option<&str>) -> Option<String> {
    broker_url.map(str::to_string)
}
