//! WebSocket telemetry stream for Control Center (`/v1/stream/telemetry`).
//!
use crate::state::SharedState;
use serde::Deserialize;
use serde_json::json;
use spanda_telemetry_store::global_store;
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use tungstenite::{accept, Message, WebSocket};

/// True when the parsed HTTP request is a WebSocket upgrade for telemetry streaming.
pub fn is_telemetry_stream_upgrade(raw: &str, path: &str) -> bool {
    if path != "/v1/stream/telemetry" {
        return false;
    }
    let lower = raw.to_ascii_lowercase();
    lower.contains("upgrade: websocket") && lower.contains("connection:")
}

#[derive(Debug, Clone, Copy, Default)]
struct StreamOffsets {
    telemetry: usize,
    traces: usize,
    alerts: usize,
}

#[derive(Debug, Clone)]
struct StreamConfig {
    max_pending_frames: usize,
    heartbeat_interval_ms: u64,
    poll_interval_ms: u64,
}

impl StreamConfig {
    fn from_env() -> Self {
        Self {
            max_pending_frames: std::env::var("SPANDA_WS_MAX_PENDING_FRAMES")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(64),
            heartbeat_interval_ms: std::env::var("SPANDA_WS_HEARTBEAT_INTERVAL_MS")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(15_000),
            poll_interval_ms: std::env::var("SPANDA_WS_POLL_INTERVAL_MS")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(250),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ResumeRequest {
    #[serde(default)]
    telemetry_offset: usize,
    #[serde(default)]
    trace_offset: usize,
    #[serde(default)]
    alert_offset: usize,
}

enum ClientCommand {
    Resume(StreamOffsets),
    Ping,
}

/// Serve live telemetry, API traces, and alerts over WebSocket.
pub fn serve_telemetry_websocket(
    stream: TcpStream,
    prefix: &[u8],
    state: SharedState,
) -> Result<(), String> {
    let config = StreamConfig::from_env();
    let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));
    let prefixed = PrefixedReader {
        cursor: Cursor::new(prefix.to_vec()),
        inner: stream,
    };
    let mut websocket = accept(prefixed).map_err(|error| error.to_string())?;
    let mut pending_frames = 0usize;
    let mut backpressure_active = false;
    let mut offsets = StreamOffsets::default();
    let mut last_heartbeat = std::time::Instant::now();
    let deadline = std::time::Instant::now() + stream_duration();

    send_json(
        &mut websocket,
        &json!({
            "type": "hello",
            "version": "v1",
            "stream": "telemetry",
            "contract": {
                "resume_supported": true,
                "max_pending_frames": config.max_pending_frames,
                "heartbeat_interval_ms": config.heartbeat_interval_ms,
                "reconnect": "send {\"type\":\"resume\",\"telemetry_offset\":N,\"trace_offset\":N,\"alert_offset\":N}",
            }
        }),
        &mut pending_frames,
        config.max_pending_frames,
    )?;

    while std::time::Instant::now() < deadline {
        if let Some(command) = drain_client_messages(&mut websocket)? {
            match command {
                ClientCommand::Resume(resume) => {
                    offsets = resume;
                    send_json(
                        &mut websocket,
                        &json!({
                            "type": "resumed",
                            "telemetry_offset": offsets.telemetry,
                            "trace_offset": offsets.traces,
                            "alert_offset": offsets.alerts,
                        }),
                        &mut pending_frames,
                        config.max_pending_frames,
                    )?;
                }
                ClientCommand::Ping => {
                    send_json(
                        &mut websocket,
                        &json!({ "type": "pong" }),
                        &mut pending_frames,
                        config.max_pending_frames,
                    )?;
                }
            }
        }

        if last_heartbeat.elapsed() >= Duration::from_millis(config.heartbeat_interval_ms) {
            send_json(
                &mut websocket,
                &json!({ "type": "heartbeat", "offsets": {
                    "telemetry": offsets.telemetry,
                    "traces": offsets.traces,
                    "alerts": offsets.alerts,
                }}),
                &mut pending_frames,
                config.max_pending_frames,
            )?;
            last_heartbeat = std::time::Instant::now();
        }

        if pending_frames >= config.max_pending_frames {
            if !backpressure_active {
                backpressure_active = true;
                send_json(
                    &mut websocket,
                    &json!({ "type": "backpressure", "paused": true, "pending_frames": pending_frames }),
                    &mut pending_frames,
                    config.max_pending_frames,
                )?;
            }
            std::thread::sleep(Duration::from_millis(config.poll_interval_ms));
            pending_frames = pending_frames.saturating_sub(1);
            continue;
        }
        if backpressure_active {
            backpressure_active = false;
            send_json(
                &mut websocket,
                &json!({ "type": "backpressure", "paused": false, "pending_frames": pending_frames }),
                &mut pending_frames,
                config.max_pending_frames,
            )?;
        }

        if let Ok(guard) = state.lock() {
            let traces = guard.trace_log.list_owned();
            if traces.len() > offsets.traces {
                for record in traces.iter().skip(offsets.traces) {
                    send_json(
                        &mut websocket,
                        &json!({ "type": "trace", "record": record }),
                        &mut pending_frames,
                        config.max_pending_frames,
                    )?;
                }
                offsets.traces = traces.len();
            }
            let alerts = guard.alert_store.list_owned();
            if alerts.len() > offsets.alerts {
                for alert in alerts.iter().skip(offsets.alerts) {
                    send_json(
                        &mut websocket,
                        &json!({ "type": "alert", "alert": alert }),
                        &mut pending_frames,
                        config.max_pending_frames,
                    )?;
                }
                offsets.alerts = alerts.len();
            }
        }

        if let Ok(store) = global_store().lock() {
            let events = store.read_all().map_err(|error| error.to_string())?;
            if offsets.telemetry < events.len() {
                for event in events.iter().skip(offsets.telemetry) {
                    send_json(
                        &mut websocket,
                        &json!({ "type": "telemetry", "event": event }),
                        &mut pending_frames,
                        config.max_pending_frames,
                    )?;
                }
                offsets.telemetry = events.len();
            }
        }

        std::thread::sleep(Duration::from_millis(config.poll_interval_ms));
    }

    let _ = websocket.close(None);
    Ok(())
}

fn send_json(
    websocket: &mut WebSocket<PrefixedReader<TcpStream>>,
    value: &serde_json::Value,
    pending_frames: &mut usize,
    max_pending_frames: usize,
) -> Result<(), String> {
    if *pending_frames >= max_pending_frames {
        return Ok(());
    }
    let text = serde_json::to_string(value).map_err(|error| error.to_string())?;
    websocket
        .send(Message::Text(text))
        .map_err(|error| error.to_string())?;
    *pending_frames += 1;
    Ok(())
}

fn drain_client_messages(
    websocket: &mut WebSocket<PrefixedReader<TcpStream>>,
) -> Result<Option<ClientCommand>, String> {
    loop {
        match websocket.read() {
            Ok(Message::Close(_)) => return Err("client closed websocket".into()),
            Ok(Message::Ping(payload)) => {
                websocket
                    .send(Message::Pong(payload))
                    .map_err(|error| error.to_string())?;
            }
            Ok(Message::Pong(_)) | Ok(Message::Frame(_)) => {}
            Ok(Message::Text(text)) => {
                if let Some(command) = parse_client_command(&text) {
                    return Ok(Some(command));
                }
            }
            Ok(Message::Binary(_)) => {}
            Err(tungstenite::Error::Io(error))
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(tungstenite::Error::AlreadyClosed) => return Err("websocket closed".into()),
            Err(_) => break,
        }
    }
    Ok(None)
}

fn parse_client_command(text: &str) -> Option<ClientCommand> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let kind = value.get("type").and_then(|entry| entry.as_str())?;
    match kind {
        "resume" => {
            let resume: ResumeRequest = serde_json::from_value(value).ok()?;
            Some(ClientCommand::Resume(StreamOffsets {
                telemetry: resume.telemetry_offset,
                traces: resume.trace_offset,
                alerts: resume.alert_offset,
            }))
        }
        "ping" => Some(ClientCommand::Ping),
        _ => None,
    }
}

fn stream_duration() -> Duration {
    std::env::var("SPANDA_WS_STREAM_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(5))
}

struct PrefixedReader<R: Read> {
    cursor: Cursor<Vec<u8>>,
    inner: R,
}

impl<R: Read> Read for PrefixedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let from_prefix = self.cursor.read(buf)?;
        if from_prefix > 0 {
            return Ok(from_prefix);
        }
        self.inner.read(buf)
    }
}

impl<R: Read + Write> Write for PrefixedReader<R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_upgrade_request() {
        let raw = "GET /v1/stream/telemetry HTTP/1.1\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\r\n";
        assert!(is_telemetry_stream_upgrade(raw, "/v1/stream/telemetry"));
        assert!(!is_telemetry_stream_upgrade(raw, "/v1/health"));
    }

    #[test]
    fn parses_resume_command() {
        let command = parse_client_command(
            r#"{"type":"resume","telemetry_offset":3,"trace_offset":1,"alert_offset":2}"#,
        )
        .expect("resume");
        match command {
            ClientCommand::Resume(offsets) => {
                assert_eq!(offsets.telemetry, 3);
                assert_eq!(offsets.traces, 1);
                assert_eq!(offsets.alerts, 2);
            }
            ClientCommand::Ping => panic!("expected resume"),
        }
    }
}
