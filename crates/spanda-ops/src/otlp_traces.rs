//! OTLP/JSON trace export for Jaeger and OpenTelemetry collectors.
//!
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

/// HTTP API span input for OTLP trace rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpTraceSpan {
    pub correlation_id: String,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub timestamp_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
}

/// Render Control Center API spans as OTLP/JSON (`ExportTraceServiceRequest` shape).
pub fn render_otlp_traces_json(spans: &[HttpTraceSpan]) -> String {
    let resource_spans = if spans.is_empty() {
        vec![empty_resource_spans()]
    } else {
        vec![json!({
            "resource": {
                "attributes": [
                    attr("service.name", "spanda-control-center"),
                    attr("telemetry.sdk.name", "spanda-ops"),
                ]
            },
            "scopeSpans": [{
                "scope": { "name": "spanda.control-center.http" },
                "spans": spans.iter().map(render_span).collect::<Vec<_>>(),
            }]
        })]
    };
    serde_json::to_string(&json!({ "resourceSpans": resource_spans }))
        .unwrap_or_else(|_| r#"{"resourceSpans":[]}"#.into())
}

/// Push OTLP/JSON traces to a collector (Jaeger OTLP HTTP default `/v1/traces`).
pub fn push_otlp_traces(endpoint: &str, body: &str, token: Option<&str>) -> Result<(), String> {
    let response = spanda_deploy_http::http_request("POST", endpoint, Some(body), token)?;
    if (200..300).contains(&response.status) {
        return Ok(());
    }
    Err(format!(
        "OTLP trace push failed: HTTP {} from {endpoint}",
        response.status
    ))
}

/// Resolve traces endpoint from env (`SPANDA_OTLP_TRACES_ENDPOINT`, `SPANDA_JAEGER_OTLP_ENDPOINT`, `SPANDA_OTEL_COLLECTOR_URL`, or metrics URL rewrite).
pub fn env_traces_endpoint() -> Option<String> {
    if let Ok(value) = std::env::var("SPANDA_OTLP_TRACES_ENDPOINT") {
        if !value.trim().is_empty() {
            return Some(value);
        }
    }
    if let Ok(value) = std::env::var("SPANDA_JAEGER_OTLP_ENDPOINT") {
        if !value.trim().is_empty() {
            return Some(value);
        }
    }
    if let Ok(value) = std::env::var("SPANDA_OTEL_COLLECTOR_URL") {
        if !value.trim().is_empty() {
            return Some(normalize_traces_endpoint(&value));
        }
    }
    std::env::var("SPANDA_OTLP_ENDPOINT")
        .ok()
        .map(|endpoint| endpoint.replace("/v1/metrics", "/v1/traces"))
}

fn normalize_traces_endpoint(base: &str) -> String {
    let trimmed = base.trim_end_matches('/');
    if trimmed.ends_with("/v1/traces") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/v1/traces")
    }
}

/// Summary of configured distributed trace/metrics backends.
pub fn observability_backend_summary() -> serde_json::Value {
    use crate::otlp_metrics::env_metrics_endpoint;
    serde_json::json!({
        "package": "spanda-otel-collector",
        "traces_endpoint": env_traces_endpoint(),
        "metrics_endpoint": env_metrics_endpoint(),
        "auto_push": env_trace_auto_push_enabled(),
        "collector_url_env": "SPANDA_OTEL_COLLECTOR_URL",
    })
}

/// True when `SPANDA_OTLP_TRACE_AUTO_PUSH=1`.
pub fn env_trace_auto_push_enabled() -> bool {
    std::env::var("SPANDA_OTLP_TRACE_AUTO_PUSH")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Bearer token shared with metrics OTLP push.
pub fn env_otlp_token() -> Option<String> {
    std::env::var("SPANDA_OTLP_TOKEN").ok()
}

fn empty_resource_spans() -> Value {
    json!({
        "resource": {
            "attributes": [attr("service.name", "spanda-control-center")]
        },
        "scopeSpans": [{
            "scope": { "name": "spanda.control-center.http" },
            "spans": [],
        }]
    })
}

fn render_span(span: &HttpTraceSpan) -> Value {
    let start_nano = ms_to_nano(span.timestamp_ms);
    let end_nano = span
        .duration_ms
        .map(|duration| ms_to_nano(span.timestamp_ms + duration))
        .unwrap_or(start_nano);
    let (trace_id, span_id) = ids_from_correlation(&span.correlation_id);
    let status_code = if span.status < 400 { 1 } else { 2 };
    json!({
        "traceId": trace_id,
        "spanId": span_id,
        "name": format!("{} {}", span.method, span.path),
        "kind": 2,
        "startTimeUnixNano": start_nano.to_string(),
        "endTimeUnixNano": end_nano.to_string(),
        "attributes": [
            attr("spanda.correlation_id", &span.correlation_id),
            attr("http.method", &span.method),
            attr("http.route", &span.path),
            int_attr("http.status_code", span.status as i64),
        ],
        "status": { "code": status_code },
    })
}

fn attr(key: &str, value: &str) -> Value {
    json!({
        "key": key,
        "value": { "stringValue": value }
    })
}

fn int_attr(key: &str, value: i64) -> Value {
    json!({
        "key": key,
        "value": { "intValue": value.to_string() }
    })
}

fn ids_from_correlation(correlation_id: &str) -> (String, String) {
    let digest = Sha256::digest(correlation_id.as_bytes());
    let trace_id = hex::encode(&digest[..16]);
    let span_id = hex::encode(&digest[16..24]);
    (trace_id, span_id)
}

fn ms_to_nano(ms: f64) -> u64 {
    (ms * 1_000_000.0).max(0.0) as u64
}

pub fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn otlp_traces_json_shape() {
        let body = render_otlp_traces_json(&[HttpTraceSpan {
            correlation_id: "smoke-trace-1".into(),
            method: "GET".into(),
            path: "/v1/health".into(),
            status: 200,
            timestamp_ms: 1_000.0,
            duration_ms: Some(2.5),
        }]);
        assert!(body.contains("resourceSpans"));
        assert!(body.contains("spanda-control-center"));
        assert!(body.contains("GET /v1/health"));
    }

    #[test]
    fn otel_collector_url_normalizes_traces_path() {
        std::env::set_var("SPANDA_OTEL_COLLECTOR_URL", "http://collector:4318");
        std::env::remove_var("SPANDA_OTLP_TRACES_ENDPOINT");
        std::env::remove_var("SPANDA_JAEGER_OTLP_ENDPOINT");
        assert_eq!(
            env_traces_endpoint().as_deref(),
            Some("http://collector:4318/v1/traces")
        );
    }
}
