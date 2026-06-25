//! Optional ML backend for trace spoofing analysis via HTTP endpoint.

use crate::trace::{MissionTrace, SpoofingAlert};
use serde::Deserialize;

/// Merge optional ML spoofing alerts when `SPANDA_SPOOFING_ML_ENDPOINT` is configured.
pub fn merge_ml_spoofing_alerts(trace: &MissionTrace, alerts: &mut Vec<SpoofingAlert>) {
    // POST trace JSON to an external ML endpoint and append returned alerts.
    //
    // Parameters:
    // - `trace` — mission trace under analysis
    // - `alerts` — heuristic alerts to extend in place
    //
    // Returns:
    // None (appends ML alerts when endpoint responds successfully).
    //
    // Options:
    // `SPANDA_SPOOFING_ML_ENDPOINT` — HTTP URL accepting trace JSON.
    //
    // Example:
    // merge_ml_spoofing_alerts(&trace, &mut alerts);

    let Some(endpoint) = std::env::var("SPANDA_SPOOFING_ML_ENDPOINT")
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        return;
    };
    let body = match serde_json::to_string(trace) {
        Ok(value) => value,
        Err(_) => return,
    };
    let response = match spanda_deploy_http::http_request("POST", &endpoint, Some(&body), None) {
        Ok(value) => value,
        Err(_) => return,
    };
    if !(200..300).contains(&response.status) {
        return;
    }
    let Ok(payload) = serde_json::from_str::<MlSpoofingResponse>(&response.body) else {
        return;
    };
    alerts.extend(payload.alerts);
}

#[derive(Debug, Deserialize)]
struct MlSpoofingResponse {
    #[serde(default)]
    alerts: Vec<SpoofingAlert>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::{SpoofingSeverity, TraceFrame};

    #[test]
    fn ml_merge_is_noop_without_endpoint() {
        let trace = MissionTrace {
            version: 1,
            source: "rover.sd".into(),
            deterministic: true,
            frames: vec![TraceFrame {
                sim_time_ms: 0.0,
                event: "gps".into(),
                payload: serde_json::json!({}),
            }],
        };
        let mut alerts = Vec::new();
        std::env::remove_var("SPANDA_SPOOFING_ML_ENDPOINT");
        merge_ml_spoofing_alerts(&trace, &mut alerts);
        assert!(alerts.is_empty());
    }

    #[test]
    fn ml_response_deserializes_alerts() {
        let json = r#"{"alerts":[{"sensor":"gps","severity":"high","confidence":0.9,"message":"ml","evidence":"model"}]}"#;
        let payload: MlSpoofingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(payload.alerts.len(), 1);
        assert_eq!(payload.alerts[0].severity, SpoofingSeverity::High);
    }
}
