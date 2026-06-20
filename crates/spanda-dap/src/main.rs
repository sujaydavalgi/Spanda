use serde_json::{json, Value};
use spanda_core::{run_debug, DebugOptions, SpandaError};
use std::collections::HashSet;
use std::io::{self, BufRead, Write};

fn read_message(reader: &mut dyn BufRead) -> io::Result<Option<Value>> {
    let mut line = String::new();
    let mut content_length = 0usize;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        if let Some(rest) = line.strip_prefix("Content-Length:") {
            content_length = rest.trim().parse().unwrap_or(0);
        } else if line.trim().is_empty() && content_length > 0 {
            break;
        }
    }
    let mut body = vec![0u8; content_length];
    reader.read_exact(&mut body)?;
    Ok(Some(serde_json::from_slice(&body)?))
}

fn write_message(writer: &mut dyn Write, msg: &Value) -> io::Result<()> {
    let body = serde_json::to_string(msg)?;
    write!(writer, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
    writer.flush()
}

fn respond(writer: &mut dyn Write, req: &Value, body: Value) -> io::Result<()> {
    write_message(
        writer,
        &json!({
            "seq": req.get("seq").cloned().unwrap_or(json!(0)),
            "type": "response",
            "request_seq": req.get("seq"),
            "success": true,
            "command": req.get("command"),
            "body": body,
        }),
    )
}

pub fn serve(source: &str, reader: &mut dyn BufRead, writer: &mut dyn Write) -> io::Result<()> {
    let mut breakpoints: HashSet<u32> = HashSet::new();
    let mut running = false;

    while let Some(req) = read_message(reader)? {
        let command = req
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        match command {
            "initialize" => {
                respond(
                    writer,
                    &req,
                    json!({
                        "capabilities": {
                            "supportsConfigurationDoneRequest": true,
                            "supportsSetVariable": false,
                        }
                    }),
                )?;
            }
            "launch" => {
                running = true;
                respond(writer, &req, json!({}))?;
            }
            "setBreakpoints" => {
                breakpoints.clear();
                if let Some(bps) = req
                    .pointer("/arguments/breakpoints")
                    .and_then(|v| v.as_array())
                {
                    for bp in bps {
                        if let Some(line) = bp.get("line").and_then(|l| l.as_u64()) {
                            breakpoints.insert(line as u32);
                        }
                    }
                }
                let verified: Vec<Value> = breakpoints
                    .iter()
                    .map(|line| json!({ "verified": true, "line": line }))
                    .collect();
                respond(writer, &req, json!({ "breakpoints": verified }))?;
            }
            "configurationDone" => {
                if running {
                    let session = run_debug(
                        source,
                        DebugOptions {
                            breakpoints: breakpoints.clone(),
                            step: false,
                        },
                    )
                    .unwrap_or_else(|e: SpandaError| {
                        spanda_core::DebugSession {
                            pauses: vec![spanda_core::DebugPause {
                                line: 1,
                                reason: e.to_string(),
                            }],
                        }
                    });
                    for pause in session.pauses {
                        write_message(
                            writer,
                            &json!({
                                "type": "event",
                                "event": "stopped",
                                "body": {
                                    "reason": "breakpoint",
                                    "threadId": 1,
                                    "text": pause.reason,
                                    "line": pause.line,
                                }
                            }),
                        )?;
                    }
                }
                respond(writer, &req, json!({}))?;
            }
            "threads" => {
                respond(
                    writer,
                    &req,
                    json!({ "threads": [{ "id": 1, "name": "spanda-main" }] }),
                )?;
            }
            "stackTrace" => {
                respond(
                    writer,
                    &req,
                    json!({
                        "stackFrames": [{
                            "id": 1,
                            "name": "main",
                            "line": 1,
                            "column": 1,
                        }],
                        "totalFrames": 1,
                    }),
                )?;
            }
            "disconnect" => {
                respond(writer, &req, json!({}))?;
                break;
            }
            _ => {
                respond(writer, &req, json!({}))?;
            }
        }
    }
    Ok(())
}

fn main() {
    let source = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: spanda-dap <file.sd>");
        std::process::exit(1);
    });
    let text = std::fs::read_to_string(&source).unwrap_or_else(|e| {
        eprintln!("Error reading {source}: {e}");
        std::process::exit(1);
    });
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut stdout = io::stdout();
    if let Err(e) = serve(&text, &mut reader, &mut stdout) {
        eprintln!("DAP server error: {e}");
        std::process::exit(1);
    }
}
