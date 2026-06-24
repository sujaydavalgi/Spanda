//! Remote OTLP push helpers for one-shot and scheduled collector ingest.

use crate::error::{TelemetryStoreError, TelemetryStoreResult};
use crate::global_store;
use crate::otlp::render_otlp_json;
use std::thread;
use std::time::Duration;

/// Options for a blocking OTLP push loop (`spanda telemetry push --watch`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtlpPushOptions {
    pub endpoint: String,
    pub token: Option<String>,
    pub interval_ms: u64,
    pub once: bool,
}

/// Return true when `SPANDA_OTLP_AUTO_PUSH=1` or `SPANDA_OTLP_PUSH=1`.
pub fn env_auto_push_enabled() -> bool {
    fn truthy(name: &str) -> bool {
        std::env::var(name)
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }
    truthy("SPANDA_OTLP_AUTO_PUSH") || truthy("SPANDA_OTLP_PUSH")
}

/// Resolved OTLP collector URL from `SPANDA_OTLP_ENDPOINT`.
pub fn env_otlp_endpoint() -> Option<String> {
    std::env::var("SPANDA_OTLP_ENDPOINT").ok()
}

/// Bearer token from `SPANDA_OTLP_TOKEN`.
pub fn env_otlp_token() -> Option<String> {
    std::env::var("SPANDA_OTLP_TOKEN").ok()
}

/// Push interval for watch mode (`SPANDA_OTLP_PUSH_INTERVAL_MS`, default 30s).
pub fn env_push_interval_ms() -> u64 {
    std::env::var("SPANDA_OTLP_PUSH_INTERVAL_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(30_000)
}

/// POST OTLP/JSON metrics to a remote collector.
#[cfg(feature = "push")]
pub fn push_otlp_json(
    endpoint: &str,
    body: &str,
    token: Option<&str>,
) -> TelemetryStoreResult<()> {
    let response = spanda_deploy_http::http_request("POST", endpoint, Some(body), token)
        .map_err(|error| TelemetryStoreError::Serialization(error))?;
    if (200..300).contains(&response.status) {
        return Ok(());
    }
    Err(TelemetryStoreError::Serialization(format!(
        "HTTP {} from {endpoint}",
        response.status
    )))
}

/// Push the global persistent store snapshot to the configured OTLP endpoint.
#[cfg(feature = "push")]
pub fn push_global_store(endpoint: &str, token: Option<&str>) -> TelemetryStoreResult<()> {
    let store = global_store()
        .lock()
        .map_err(|_| TelemetryStoreError::LockPoisoned)?;
    let body = render_otlp_json(&store)?;
    push_otlp_json(endpoint, &body, token)
}

/// Push after a run session ends when auto-push env vars are set.
#[cfg(feature = "push")]
pub fn maybe_auto_push_after_session() {
    if !env_auto_push_enabled() {
        return;
    }
    let Some(endpoint) = env_otlp_endpoint() else {
        eprintln!("SPANDA_OTLP_AUTO_PUSH set but SPANDA_OTLP_ENDPOINT is missing");
        return;
    };
    let token = env_otlp_token();
    match push_global_store(&endpoint, token.as_deref()) {
        Ok(()) => eprintln!("Auto-pushed OTLP metrics to {endpoint}"),
        Err(error) => eprintln!("OTLP auto-push failed: {error}"),
    }
}

/// Run a one-shot or periodic OTLP push loop until interrupted.
#[cfg(feature = "push")]
pub fn run_otlp_push_loop(options: &OtlpPushOptions) -> TelemetryStoreResult<()> {
    loop {
        push_global_store(&options.endpoint, options.token.as_deref())?;
        if options.once {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(options.interval_ms));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_auto_push_reads_flags() {
        std::env::set_var("SPANDA_OTLP_AUTO_PUSH", "1");
        assert!(env_auto_push_enabled());
        std::env::remove_var("SPANDA_OTLP_AUTO_PUSH");
        assert!(!env_auto_push_enabled());
    }

    #[test]
    fn env_push_interval_defaults() {
        std::env::remove_var("SPANDA_OTLP_PUSH_INTERVAL_MS");
        assert_eq!(env_push_interval_ms(), 30_000);
        std::env::set_var("SPANDA_OTLP_PUSH_INTERVAL_MS", "5000");
        assert_eq!(env_push_interval_ms(), 5_000);
        std::env::remove_var("SPANDA_OTLP_PUSH_INTERVAL_MS");
    }
}
