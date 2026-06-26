//! Mission-trace plausibility analysis for GPS and sensor spoofing signals.

use serde::{Deserialize, Serialize};
use spanda_connectivity::runtime_sim::haversine_m;

/// Spoofing alert severity for response policy integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpoofingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// One spoofing signal with confidence score (never binary-only for critical actions).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpoofingAlert {
    pub sensor: String,
    pub severity: SpoofingSeverity,
    pub confidence: f64,
    pub message: String,
    pub evidence: String,
    pub sim_time_ms: Option<f64>,
}

/// Maximum plausible ground-robot speed for GPS jump detection (m/s).
pub const DEFAULT_MAX_GROUND_SPEED_M_S: f64 = 15.0;

/// Minimal mission trace frame for spoofing analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceFrame {
    pub sim_time_ms: f64,
    pub event: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

/// Mission trace file consumed by spoof-check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionTrace {
    pub version: u32,
    pub source: String,
    #[serde(default)]
    pub deterministic: bool,
    pub frames: Vec<TraceFrame>,
}

/// Analyze a mission trace for GPS spoofing and plausibility violations.
pub fn analyze_trace_spoofing(trace: &MissionTrace, max_speed_m_s: f64) -> Vec<SpoofingAlert> {
    // Scan trace frames for impossible GPS motion, spoof events, and degraded fix quality.
    //
    // Parameters:
    // - `trace` — deserialized mission trace
    // - `max_speed_m_s` — maximum plausible ground speed between GPS samples
    //
    // Returns:
    // Spoofing alerts sorted by simulation time.
    //
    // Options:
    // None.
    //
    // Example:
    // let alerts = analyze_trace_spoofing(&trace, DEFAULT_MAX_GROUND_SPEED_M_S);

    let mut alerts = Vec::new();
    let mut last_gps: Option<(f64, f64, f64)> = None;

    for frame in &trace.frames {
        let event_lower = frame.event.to_ascii_lowercase();

        if event_lower.contains("spoof") {
            alerts.push(SpoofingAlert {
                sensor: "gps".into(),
                severity: SpoofingSeverity::High,
                confidence: 0.95,
                message: "Explicit GPS spoofing event recorded in mission trace".into(),
                evidence: format!("event={}", frame.event),
                sim_time_ms: Some(frame.sim_time_ms),
            });
        }

        if let Some((lat, lon)) = extract_lat_lon(&frame.payload) {
            if let Some((prev_time, prev_lat, prev_lon)) = last_gps {
                let delta_s = ((frame.sim_time_ms - prev_time) / 1000.0).max(0.001);
                let distance_m = haversine_m(prev_lat, prev_lon, lat, lon);
                let speed_m_s = distance_m / delta_s;

                if speed_m_s > max_speed_m_s {
                    let confidence = (speed_m_s / max_speed_m_s).min(1.0).max(0.5);
                    alerts.push(SpoofingAlert {
                        sensor: "gps".into(),
                        severity: if speed_m_s > max_speed_m_s * 3.0 {
                            SpoofingSeverity::Critical
                        } else if speed_m_s > max_speed_m_s * 1.5 {
                            SpoofingSeverity::High
                        } else {
                            SpoofingSeverity::Medium
                        },
                        confidence,
                        message: format!(
                            "Impossible GPS movement: {distance_m:.1} m in {delta_s:.3} s ({speed_m_s:.1} m/s)"
                        ),
                        evidence: format!(
                            "from ({prev_lat:.6}, {prev_lon:.6}) to ({lat:.6}, {lon:.6})"
                        ),
                        sim_time_ms: Some(frame.sim_time_ms),
                    });
                }
            }

            last_gps = Some((frame.sim_time_ms, lat, lon));
        }

        if payload_indicates_spoofed_fix(&frame.payload) {
            alerts.push(SpoofingAlert {
                sensor: "gps".into(),
                severity: SpoofingSeverity::Medium,
                confidence: 0.75,
                message: "GPS fix quality or spoof flag indicates degraded or spoofed signal"
                    .into(),
                evidence: frame.payload.to_string(),
                sim_time_ms: Some(frame.sim_time_ms),
            });
        }

        if let Some(conflict) = detect_imu_gps_conflict(frame, last_gps) {
            alerts.push(conflict);
        }
    }

    alerts.sort_by(|left, right| {
        left.sim_time_ms
            .partial_cmp(&right.sim_time_ms)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    alerts
}

fn extract_lat_lon(payload: &serde_json::Value) -> Option<(f64, f64)> {
    // Pull WGS84 coordinates from common trace payload shapes.
    if let Some(obj) = payload.as_object() {
        if let (Some(lat), Some(lon)) = (obj.get("lat"), obj.get("lon")) {
            if let (Some(lat), Some(lon)) = (lat.as_f64(), lon.as_f64()) {
                return Some((lat, lon));
            }
        }
        if let (Some(lat), Some(lon)) = (obj.get("latitude"), obj.get("longitude")) {
            if let (Some(lat), Some(lon)) = (lat.as_f64(), lon.as_f64()) {
                return Some((lat, lon));
            }
        }
        if let Some(position) = obj.get("position").and_then(|v| v.as_object()) {
            if let (Some(lat), Some(lon)) = (position.get("lat"), position.get("lon")) {
                if let (Some(lat), Some(lon)) = (lat.as_f64(), lon.as_f64()) {
                    return Some((lat, lon));
                }
            }
        }
    }
    None
}

fn payload_indicates_spoofed_fix(payload: &serde_json::Value) -> bool {
    // Flag explicit spoof markers or critically degraded fix quality in payloads.
    let Some(obj) = payload.as_object() else {
        return false;
    };

    if obj
        .get("spoofed")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return true;
    }

    if let Some(quality) = obj.get("fix_quality").and_then(|v| v.as_str()) {
        let lower = quality.to_ascii_lowercase();
        if lower.contains("spoof") || lower == "invalid" || lower == "none" {
            return true;
        }
    }

    false
}

fn detect_imu_gps_conflict(
    frame: &TraceFrame,
    last_gps: Option<(f64, f64, f64)>,
) -> Option<SpoofingAlert> {
    // Detect IMU-reported motion disagreeing with stationary GPS when both appear in one frame.
    let event_lower = frame.event.to_ascii_lowercase();
    if !event_lower.contains("imu") {
        return None;
    }

    let Some(obj) = frame.payload.as_object() else {
        return None;
    };
    let speed = obj
        .get("speed_m_s")
        .or_else(|| obj.get("velocity_m_s"))
        .and_then(|v| v.as_f64())?;
    let Some((gps_time, _, _)) = last_gps else {
        return None;
    };

    if (frame.sim_time_ms - gps_time).abs() > 250.0 {
        return None;
    }

    if speed > 0.5 {
        return None;
    }

    Some(SpoofingAlert {
        sensor: "gps+imu".into(),
        severity: SpoofingSeverity::Medium,
        confidence: 0.7,
        message: "IMU reports near-zero motion while GPS position jumped recently".into(),
        evidence: format!(
            "imu speed={speed_m_s:.2} m/s near gps sample",
            speed_m_s = speed
        ),
        sim_time_ms: Some(frame.sim_time_ms),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_impossible_gps_jump() {
        let trace = MissionTrace {
            version: 1,
            source: "test".into(),
            deterministic: true,
            frames: vec![
                TraceFrame {
                    sim_time_ms: 0.0,
                    event: "gps_reading".into(),
                    payload: serde_json::json!({"lat": 37.7749, "lon": -122.4194}),
                },
                TraceFrame {
                    sim_time_ms: 500.0,
                    event: "gps_reading".into(),
                    payload: serde_json::json!({"lat": 37.8049, "lon": -122.4194}),
                },
            ],
        };
        let alerts = analyze_trace_spoofing(&trace, DEFAULT_MAX_GROUND_SPEED_M_S);
        assert!(!alerts.is_empty());
        assert!(alerts
            .iter()
            .any(|alert| alert.severity >= SpoofingSeverity::High));
    }

    #[test]
    fn detects_explicit_spoof_event() {
        let trace = MissionTrace {
            version: 1,
            source: "test".into(),
            deterministic: true,
            frames: vec![TraceFrame {
                sim_time_ms: 100.0,
                event: "emit gps.spoofed".into(),
                payload: serde_json::json!({"reason": "simulated"}),
            }],
        };
        let alerts = analyze_trace_spoofing(&trace, DEFAULT_MAX_GROUND_SPEED_M_S);
        assert_eq!(alerts.len(), 1);
        assert!((alerts[0].confidence - 0.95).abs() < f64::EPSILON);
    }
}
