//! SRE rollup helpers — SLO targets and availability budgeting.
//!
use serde_json::{json, Value};

/// Target availability percent from `SPANDA_SRE_SLO_PERCENT` (default 99.0).
pub fn slo_target_percent() -> f64 {
    std::env::var("SPANDA_SRE_SLO_PERCENT")
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| *value > 0.0 && *value <= 100.0)
        .unwrap_or(99.0)
}

/// SLO status object for Control Center `/v1/sre/summary`.
pub fn slo_status(availability_percent: f64) -> Value {
    let target = slo_target_percent();
    let met = availability_percent >= target;
    json!({
        "target_percent": target,
        "met": met,
        "budget_remaining_percent": availability_percent - target,
        "env": "SPANDA_SRE_SLO_PERCENT",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slo_met_when_availability_exceeds_target() {
        std::env::set_var("SPANDA_SRE_SLO_PERCENT", "95");
        let status = slo_status(96.0);
        assert_eq!(status["met"], true);
        std::env::remove_var("SPANDA_SRE_SLO_PERCENT");
    }
}
