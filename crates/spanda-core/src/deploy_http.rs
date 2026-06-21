//! Minimal HTTP/1.1 helpers for the Spanda deploy agent protocol.

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedUrl {
    pub host: String,
    pub port: u16,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub body: String,
    pub authorization: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

pub fn parse_http_url(url: &str) -> Result<ParsedUrl, String> {
    // Parse an http://host:port/path URL for deploy agent calls.
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| format!("deploy agent URL must start with http:// (got {url})"))?;
    let (authority, path) = match rest.split_once('/') {
        Some((auth, tail)) => (auth, format!("/{tail}")),
        None => (rest, "/".into()),
    };
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => {
            let port = p
                .parse::<u16>()
                .map_err(|_| format!("invalid port in deploy agent URL '{url}'"))?;
            (h.to_string(), port)
        }
        None => (authority.to_string(), 80),
    };
    Ok(ParsedUrl { host, port, path })
}

pub fn http_request(
    method: &str,
    url: &str,
    body: Option<&str>,
    token: Option<&str>,
) -> Result<HttpResponse, String> {
    // Issue a single HTTP/1.1 request and return the response body.
    let parsed = parse_http_url(url)?;
    let payload = body.unwrap_or("");
    let mut stream = TcpStream::connect(format!("{}:{}", parsed.host, parsed.port))
        .map_err(|e| format!("connect to {}:{} failed: {e}", parsed.host, parsed.port))?;
    let mut request = format!(
        "{method} {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
        parsed.path,
        parsed.host,
        payload.len()
    );
    if let Some(token) = token {
        request.push_str(&format!("Authorization: Bearer {token}\r\n"));
    }
    request.push_str("\r\n");
    request.push_str(payload);
    stream
        .write_all(request.as_bytes())
        .map_err(|e| format!("write request failed: {e}"))?;
    stream
        .shutdown(Shutdown::Write)
        .map_err(|e| format!("shutdown request failed: {e}"))?;
    let mut raw = String::new();
    stream
        .read_to_string(&mut raw)
        .map_err(|e| format!("read response failed: {e}"))?;
    parse_http_response(&raw)
}

pub fn parse_http_response(raw: &str) -> Result<HttpResponse, String> {
    // Split an HTTP response into status code and body.
    let (head, body) = raw
        .split_once("\r\n\r\n")
        .ok_or_else(|| "invalid HTTP response".to_string())?;
    let status_line = head.lines().next().ok_or_else(|| "missing status line".to_string())?;
    let status = status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "missing HTTP status code".to_string())?
        .parse::<u16>()
        .map_err(|_| "invalid HTTP status code".to_string())?;
    Ok(HttpResponse {
        status,
        body: body.to_string(),
    })
}

pub fn parse_http_request(raw: &str) -> Result<HttpRequest, String> {
    // Parse a minimal HTTP/1.1 request for the deploy agent server.
    let (head, body) = raw
        .split_once("\r\n\r\n")
        .ok_or_else(|| "invalid HTTP request".to_string())?;
    let mut lines = head.lines();
    let request_line = lines.next().ok_or_else(|| "missing request line".to_string())?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| "missing HTTP method".to_string())?
        .to_string();
    let path = parts
        .next()
        .ok_or_else(|| "missing HTTP path".to_string())?
        .to_string();
    let mut authorization = None;
    for line in lines {
        if let Some(token) = line.strip_prefix("Authorization: Bearer ") {
            authorization = Some(token.trim().to_string());
        }
    }
    Ok(HttpRequest {
        method,
        path,
        body: body.to_string(),
        authorization,
    })
}

pub fn http_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

pub fn serve_once(listener: &TcpListener, handler: impl Fn(HttpRequest) -> HttpResponse) -> Result<(), String> {
    // Accept one HTTP connection and write the handler response.
    let (mut stream, _) = listener
        .accept()
        .map_err(|e| format!("accept failed: {e}"))?;
    let mut raw = String::new();
    stream
        .read_to_string(&mut raw)
        .map_err(|e| format!("read request failed: {e}"))?;
    let request = parse_http_request(&raw)?;
    let response = handler(request);
    let encoded = http_response(response.status, &response.body);
    stream
        .write_all(encoded.as_bytes())
        .map_err(|e| format!("write response failed: {e}"))?;
    Ok(())
}
