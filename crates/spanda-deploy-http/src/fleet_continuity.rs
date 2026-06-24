//! Fleet mesh continuity / takeover HTTP client types and coordinator POST helper.
//!
use crate::{http_request, parse_http_url};
use serde::{Deserialize, Serialize};

/// Takeover command posted to the fleet mesh coordinator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetContinuityRequest {
    pub failed_robot: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mission: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_robot: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}

/// Result of broadcasting a takeover command to fleet agents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetContinuityResponse {
    pub ok: bool,
    pub relayed: u32,
    pub failed: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Post a takeover command to a running fleet mesh coordinator.
pub fn relay_continuity_via_mesh(
    mesh_url: &str,
    request: &FleetContinuityRequest,
    token: Option<&str>,
) -> Result<FleetContinuityResponse, String> {
    let parsed = parse_http_url(mesh_url)?;
    let url = format!(
        "{}://{}:{}/v1/fleet/continuity",
        parsed.scheme, parsed.host, parsed.port
    );
    let payload = serde_json::to_string(request).map_err(|e| e.to_string())?;
    let response = http_request("POST", &url, Some(&payload), token)?;
    if response.status >= 400 {
        return Err(format!(
            "fleet mesh continuity HTTP {}: {}",
            response.status, response.body
        ));
    }
    serde_json::from_str(&response.body).map_err(|e| format!("invalid fleet continuity JSON: {e}"))
}
