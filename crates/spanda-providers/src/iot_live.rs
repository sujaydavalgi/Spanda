//! Optional live Modbus TCP and OPC-UA bridge reads for IoT package dispatch.

use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Return true when live Modbus hardware reads are enabled.
pub fn live_modbus_enabled() -> bool {
    std::env::var("SPANDA_LIVE_MODBUS")
        .ok()
        .as_deref()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Return true when live OPC-UA bridge reads are enabled.
pub fn live_opcua_enabled() -> bool {
    std::env::var("SPANDA_LIVE_OPCUA")
        .ok()
        .as_deref()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Read a Modbus holding register from live TCP hardware when enabled.
pub fn read_modbus_register_live(address: u16) -> Option<f64> {
    // Read a Modbus holding register from live TCP hardware when enabled.
    //
    // Parameters:
    // - `address` — register address (40001-style values are normalized)
    //
    // Returns:
    // Register value when live read succeeds, otherwise none.
    //
    // Options:
    // - `SPANDA_LIVE_MODBUS=1`, `SPANDA_MODBUS_HOST`, `SPANDA_MODBUS_PORT`, `SPANDA_MODBUS_UNIT`
    //
    // Example:
    // let value = read_modbus_register_live(40001);

    // Skip live path when the env gate is off.
    if !live_modbus_enabled() {
        return None;
    }

    #[cfg(feature = "live-iot")]
    {
        // Prefer native Modbus TCP when the live-iot feature is enabled.
        if let Ok(value) = read_modbus_tcp(address) {
            return Some(value);
        }
    }

    // Fall back to the Python bridge when pymodbus is installed.
    read_modbus_via_python_bridge(address)
}

/// Read an OPC-UA node via the Python bridge when enabled.
pub fn read_opcua_node_live(node: &str) -> Option<String> {
    // Read an OPC-UA node via the Python bridge when enabled.
    //
    // Parameters:
    // - `node` — OPC-UA node id string
    //
    // Returns:
    // Node value when live read succeeds, otherwise none.
    //
    // Options:
    // - `SPANDA_LIVE_OPCUA=1`, `SPANDA_OPCUA_ENDPOINT`
    //
    // Example:
    // let value = read_opcua_node_live("ns=2;s=Temperature");

    // Skip live path when the env gate is off.
    if !live_opcua_enabled() {
        return None;
    }

    read_opcua_via_python_bridge(node)
}

fn live_iot_flag(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .as_deref()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Return true when live Zigbee bridge reads are enabled.
pub fn live_zigbee_enabled() -> bool {
    live_iot_flag("SPANDA_LIVE_ZIGBEE")
}

/// Return true when live LoRa bridge reads are enabled.
pub fn live_lora_enabled() -> bool {
    live_iot_flag("SPANDA_LIVE_LORA")
}

/// Return true when live Matter bridge reads are enabled.
pub fn live_matter_enabled() -> bool {
    live_iot_flag("SPANDA_LIVE_MATTER")
}

/// Return true when live CAN bus bridge reads are enabled.
pub fn live_canbus_enabled() -> bool {
    live_iot_flag("SPANDA_LIVE_CANBUS")
}

pub fn read_zigbee_attribute_live(device: &str, cluster: &str) -> Option<String> {
    if !live_zigbee_enabled() {
        return None;
    }
    read_string_via_python_bridge(
        "zigbee_read_attribute",
        vec![
            serde_json::Value::String(device.to_string()),
            serde_json::Value::String(cluster.to_string()),
        ],
    )
}

pub fn read_lora_payload_live(device_id: &str) -> Option<String> {
    if !live_lora_enabled() {
        return None;
    }
    read_string_via_python_bridge(
        "lora_read_payload",
        vec![serde_json::Value::String(device_id.to_string())],
    )
}

pub fn read_matter_cluster_live(node: &str, cluster: &str) -> Option<f64> {
    if !live_matter_enabled() {
        return None;
    }
    read_number_via_python_bridge(
        "matter_read_cluster",
        vec![
            serde_json::Value::String(node.to_string()),
            serde_json::Value::String(cluster.to_string()),
        ],
    )
}

pub fn read_canbus_frame_live(can_id: u32) -> Option<f64> {
    if !live_canbus_enabled() {
        return None;
    }
    read_number_via_python_bridge(
        "canbus_read_frame",
        vec![serde_json::Value::Number(can_id.into())],
    )
}

fn read_string_via_python_bridge(fn_name: &str, args: Vec<serde_json::Value>) -> Option<String> {
    match call_python_bridge(fn_name, args)?.get("result") {
        Some(serde_json::Value::String(text)) if !text.is_empty() => Some(text.clone()),
        _ => None,
    }
}

fn read_number_via_python_bridge(fn_name: &str, args: Vec<serde_json::Value>) -> Option<f64> {
    match call_python_bridge(fn_name, args)?.get("result") {
        Some(serde_json::Value::Number(n)) => n.as_f64(),
        Some(serde_json::Value::String(text)) => text.parse().ok(),
        _ => None,
    }
}

#[cfg(feature = "live-iot")]
fn read_modbus_tcp(address: u16) -> Result<f64, String> {
    // Read one holding register over Modbus TCP using the pure-Rust client.
    //
    // Parameters:
    // - `address` — register address
    //
    // Returns:
    // Register value as f64, or an error string.
    //
    // Options:
    // None.
    //
    // Example:
    // let value = read_modbus_tcp(40001)?;

    use modbus::{tcp, Client};

    let host = std::env::var("SPANDA_MODBUS_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("SPANDA_MODBUS_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(502);
    let unit = std::env::var("SPANDA_MODBUS_UNIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1u8);
    let zero_based = address.saturating_sub(40001);
    let endpoint = format!("{host}:{port}");
    let mut transport =
        tcp::Transport::new(&endpoint).map_err(|e| format!("modbus connect failed: {e}"))?;
    transport.set_uid(unit);
    let values = transport
        .read_holding_registers(zero_based, 1)
        .map_err(|e| format!("modbus read failed: {e}"))?;
    values
        .first()
        .copied()
        .map(f64::from)
        .ok_or_else(|| "modbus read returned no registers".into())
}

fn read_modbus_via_python_bridge(address: u16) -> Option<f64> {
    // Ask the Python IoT bridge to read a Modbus register.
    //
    // Parameters:
    // - `address` — register address
    //
    // Returns:
    // Register value when the bridge succeeds, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // let value = read_modbus_via_python_bridge(40001);

    let host = std::env::var("SPANDA_MODBUS_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("SPANDA_MODBUS_PORT").unwrap_or_else(|_| "502".into());
    let response = call_python_bridge(
        "modbus_read_register",
        vec![
            serde_json::Value::String(host),
            serde_json::Value::String(port),
            serde_json::Value::Number(address.into()),
        ],
    )?;
    match response.get("result") {
        Some(serde_json::Value::Number(n)) => n.as_f64(),
        _ => None,
    }
}

fn read_opcua_via_python_bridge(node: &str) -> Option<String> {
    // Ask the Python IoT bridge to read an OPC-UA node.
    //
    // Parameters:
    // - `node` — OPC-UA node id
    //
    // Returns:
    // Node value when the bridge succeeds, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // let value = read_opcua_via_python_bridge("ns=2;s=Temperature");

    let endpoint = std::env::var("SPANDA_OPCUA_ENDPOINT")
        .unwrap_or_else(|_| "opc.tcp://127.0.0.1:4840".into());
    let response = call_python_bridge(
        "opcua_read_node",
        vec![
            serde_json::Value::String(endpoint),
            serde_json::Value::String(node.to_string()),
        ],
    )?;
    match response.get("result") {
        Some(serde_json::Value::String(value)) => Some(value.clone()),
        Some(serde_json::Value::Number(n)) => n.as_f64().map(|v| v.to_string()),
        _ => None,
    }
}

fn call_python_bridge(fn_name: &str, args: Vec<serde_json::Value>) -> Option<serde_json::Value> {
    // Invoke `scripts/spanda_python_bridge.py` with a JSON request.
    //
    // Parameters:
    // - `fn_name` — bridge handler name
    // - `args` — handler arguments
    //
    // Returns:
    // Parsed JSON response when the bridge succeeds, otherwise none.
    //
    // Options:
    // - `SPANDA_PYTHON_BRIDGE` overrides script path
    //
    // Example:
    // let response = call_python_bridge("modbus_read_register", vec![]);

    let script = bridge_script_path()?;
    let python = std::env::var("SPANDA_PYTHON").unwrap_or_else(|_| "python3".into());
    let request = serde_json::json!({ "fn": fn_name, "args": args });
    let mut child = Command::new(python)
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    {
        let stdin = child.stdin.as_mut()?;
        let payload = serde_json::to_string(&request).ok()?;
        stdin.write_all(payload.as_bytes()).ok()?;
    }
    let mut stdout = String::new();
    child.stdout.as_mut()?.read_to_string(&mut stdout).ok()?;
    let _ = child.wait();
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    if parsed.get("ok") == Some(&serde_json::Value::Bool(true)) {
        Some(parsed)
    } else {
        None
    }
}

fn bridge_script_path() -> Option<String> {
    // Resolve the Python bridge script path from env or repo layout.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Script path when found, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // let path = bridge_script_path();

    if let Ok(path) = std::env::var("SPANDA_PYTHON_BRIDGE") {
        if std::path::Path::new(&path).is_file() {
            return Some(path);
        }
    }
    let candidates = [
        "scripts/spanda_python_bridge.py".to_string(),
        format!(
            "{}/../../scripts/spanda_python_bridge.py",
            env!("CARGO_MANIFEST_DIR")
        ),
    ];
    for candidate in candidates {
        if std::path::Path::new(&candidate).is_file() {
            return Some(candidate);
        }
    }
    std::env::current_dir()
        .ok()
        .map(|cwd| cwd.join("scripts/spanda_python_bridge.py"))
        .filter(|p| p.is_file())
        .map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_modbus_disabled_by_default() {
        std::env::remove_var("SPANDA_LIVE_MODBUS");
        assert!(!live_modbus_enabled());
        assert!(read_modbus_register_live(40001).is_none());
    }
}
