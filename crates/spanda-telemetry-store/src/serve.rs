//! Minimal HTTP server for Prometheus and OTLP telemetry export.

use crate::error::{TelemetryStoreError, TelemetryStoreResult};
use crate::otlp::render_otlp_json;
use crate::prometheus::render_prometheus;
use crate::store::{global_store, PersistentTelemetryStore};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

/// Options for `spanda telemetry serve`.
#[derive(Debug, Clone)]
pub struct TelemetryServeOptions {
    pub bind: String,
    pub once: bool,
    pub timeout_ms: u64,
}

impl Default for TelemetryServeOptions {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1:9090".into(),
            once: false,
            timeout_ms: 0,
        }
    }
}

/// Run the telemetry HTTP server until interrupted or `--once` is set.
pub fn run_telemetry_server(options: &TelemetryServeOptions) -> TelemetryStoreResult<()> {
    let listener = TcpListener::bind(&options.bind).map_err(|error| {
        TelemetryStoreError::Io(std::io::Error::new(
            error.kind(),
            format!("bind {} failed: {error}", options.bind),
        ))
    })?;
    eprintln!("Spanda telemetry server listening on http://{}", options.bind);
    eprintln!("  GET /metrics         Prometheus text");
    eprintln!("  GET /otlp/v1/metrics OTLP/JSON metrics");
    eprintln!("  GET /healthz         liveness");

    if options.once {
        let (mut stream, _) = listener.accept().map_err(TelemetryStoreError::Io)?;
        if options.timeout_ms > 0 {
            let _ = stream.set_read_timeout(Some(Duration::from_millis(options.timeout_ms)));
        }
        return handle_connection(&mut stream);
    }

    for connection in listener.incoming() {
        let Ok(mut stream) = connection else { continue };
        if options.timeout_ms > 0 {
            let _ = stream.set_read_timeout(Some(Duration::from_millis(options.timeout_ms)));
        }
        thread::spawn(move || {
            let _ = handle_connection(&mut stream);
        });
    }
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> TelemetryStoreResult<()> {
    let request = read_request_line(stream)?;
    let path = request
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/");
    let store = global_store().lock().unwrap();
    let response = route_request(path, &store)?;
    write_http_response(stream, response.status, response.content_type, &response.body)
}

struct HttpPayload {
    status: u16,
    content_type: &'static str,
    body: String,
}

fn route_request(path: &str, store: &PersistentTelemetryStore) -> TelemetryStoreResult<HttpPayload> {
    match path {
        "/metrics" => Ok(HttpPayload {
            status: 200,
            content_type: "text/plain; version=0.0.4; charset=utf-8",
            body: render_prometheus(store)?,
        }),
        "/otlp/v1/metrics" | "/v1/metrics" => Ok(HttpPayload {
            status: 200,
            content_type: "application/json",
            body: render_otlp_json(store)?,
        }),
        "/healthz" | "/health" => Ok(HttpPayload {
            status: 200,
            content_type: "text/plain; charset=utf-8",
            body: "ok".into(),
        }),
        _ => Ok(HttpPayload {
            status: 404,
            content_type: "text/plain; charset=utf-8",
            body: "not found".into(),
        }),
    }
}

fn read_request_line(stream: &mut TcpStream) -> TelemetryStoreResult<String> {
    let mut buf = [0u8; 4096];
    let read = stream.read(&mut buf).map_err(TelemetryStoreError::Io)?;
    let text = String::from_utf8_lossy(&buf[..read]);
    text.lines()
        .next()
        .map(str::to_string)
        .ok_or_else(|| TelemetryStoreError::Serialization("empty HTTP request".into()))
}

fn write_http_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> TelemetryStoreResult<()> {
    let status_text = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Error",
    };
    let response = format!(
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream
        .write_all(response.as_bytes())
        .map_err(TelemetryStoreError::Io)?;
    stream.flush().map_err(TelemetryStoreError::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::TelemetryEvent;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    #[test]
    fn serve_returns_prometheus_and_otlp() {
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var(
            "SPANDA_TELEMETRY_STORE_PATH",
            dir.path()
                .join("telemetry.jsonl")
                .to_string_lossy()
                .to_string(),
        );
        {
            let mut store = global_store().lock().unwrap();
            store
                .append(TelemetryEvent::Health {
                    target: "overall".into(),
                    status: "Healthy".into(),
                    timestamp_ms: 1.0,
                    session_id: None,
                })
                .unwrap();
        }

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let _ = handle_connection(&mut stream);
            }
        });

        let mut metrics = TcpStream::connect(format!("127.0.0.1:{port}")).unwrap();
        metrics
            .write_all(b"GET /metrics HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut body = String::new();
        metrics.read_to_string(&mut body).unwrap();
        assert!(body.contains("spanda_telemetry_events_total"));

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server_otlp = thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let _ = handle_connection(&mut stream);
            }
        });
        let mut otlp = TcpStream::connect(format!("127.0.0.1:{port}")).unwrap();
        otlp.write_all(b"GET /otlp/v1/metrics HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut body = String::new();
        otlp.read_to_string(&mut body).unwrap();
        assert!(body.contains("resourceMetrics"));

        server.join().unwrap();
        server_otlp.join().unwrap();
        std::env::remove_var("SPANDA_TELEMETRY_STORE_PATH");
    }
}
