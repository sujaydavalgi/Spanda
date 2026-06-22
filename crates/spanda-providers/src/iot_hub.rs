//! In-memory IoT device, telemetry, and shadow store for package dispatch stubs.

use spanda_runtime::providers::{
    Command, DeviceShadow, IoTDevice, Telemetry,
};
use spanda_runtime::value::RuntimeValue;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static IOT_HUB: OnceLock<Mutex<IotHub>> = OnceLock::new();

fn hub() -> &'static Mutex<IotHub> {
    IOT_HUB.get_or_init(|| Mutex::new(IotHub::default()))
}

/// Shared in-memory IoT state for sim and stub integrations.
#[derive(Default)]
pub struct IotHub {
    devices: HashMap<String, IoTDevice>,
    telemetry: Vec<Telemetry>,
    shadows: HashMap<String, DeviceShadow>,
    modbus_registers: HashMap<u16, f64>,
    opcua_nodes: HashMap<String, String>,
    zigbee_attributes: HashMap<String, String>,
    lora_payloads: HashMap<String, String>,
    matter_clusters: HashMap<String, f64>,
    canbus_frames: HashMap<u32, f64>,
}

impl IotHub {
    pub fn register_device(&mut self, device: IoTDevice) -> Result<(), String> {
        if device.id.is_empty() {
            return Err("device id required".into());
        }
        self.devices.insert(device.id.clone(), device);
        Ok(())
    }

    pub fn publish_telemetry(&mut self, telemetry: Telemetry) {
        self.telemetry.push(telemetry);
    }

    pub fn send_command(&mut self, command: Command) -> Result<(), String> {
        if !self.devices.contains_key(&command.device_id) {
            return Err(format!("unknown device '{}'", command.device_id));
        }
        Ok(())
    }

    pub fn update_shadow(&mut self, shadow: DeviceShadow) {
        self.shadows.insert(shadow.device_id.clone(), shadow);
    }

    pub fn read_modbus_register(&self, address: u16) -> f64 {
        self.modbus_registers.get(&address).copied().unwrap_or(0.0)
    }

    pub fn write_modbus_register(&mut self, address: u16, value: f64) {
        self.modbus_registers.insert(address, value);
    }

    pub fn read_opcua_node(&self, node: &str) -> Option<String> {
        self.opcua_nodes.get(node).cloned()
    }

    pub fn read_zigbee_attribute(&self, device: &str, cluster: &str) -> String {
        let key = format!("{device}:{cluster}");
        self.zigbee_attributes
            .get(&key)
            .cloned()
            .unwrap_or_else(|| format!("zigbee:{device}:{cluster}"))
    }

    pub fn read_lora_payload(&self, device_id: &str) -> String {
        self.lora_payloads
            .get(device_id)
            .cloned()
            .unwrap_or_else(|| format!("lora:{device_id}"))
    }

    pub fn read_matter_cluster(&self, node: &str, cluster: &str) -> f64 {
        let key = format!("{node}:{cluster}");
        self.matter_clusters.get(&key).copied().unwrap_or(1.0)
    }

    pub fn read_canbus_frame(&self, can_id: u32) -> f64 {
        self.canbus_frames.get(&can_id).copied().unwrap_or(0.0)
    }

    pub fn seed_protocol_demo(&mut self) {
        self.opcua_nodes
            .insert("ns=2;s=Temperature".into(), "22.5".into());
        self.zigbee_attributes
            .insert("sensor-1:temp".into(), "21.0".into());
        self.lora_payloads
            .insert("node-a".into(), "payload:ok".into());
        self.matter_clusters.insert("light:onoff".into(), 1.0);
        self.canbus_frames.insert(0x100, 42.0);
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn telemetry_count(&self) -> usize {
        self.telemetry.len()
    }
}

/// Register a device in the in-memory IoT hub.
pub fn register_device(device: IoTDevice) -> Result<(), String> {
    hub().lock().unwrap().register_device(device)
}

/// Publish telemetry to the in-memory IoT hub.
pub fn publish_telemetry(telemetry: Telemetry) {
    hub().lock().unwrap().publish_telemetry(telemetry);
}

/// Send a remote command through the in-memory IoT hub.
pub fn send_command(command: Command) -> Result<(), String> {
    hub().lock().unwrap().send_command(command)
}

/// Update a device shadow in the in-memory IoT hub.
pub fn update_shadow(shadow: DeviceShadow) {
    hub().lock().unwrap().update_shadow(shadow);
}

/// Read a Modbus register from the in-memory IoT hub.
pub fn read_modbus_register(address: u16) -> f64 {
    if let Some(value) = crate::iot_live::read_modbus_register_live(address) {
        return value;
    }
    hub().lock().unwrap().read_modbus_register(address)
}

/// Read an OPC-UA node value from the in-memory IoT hub.
pub fn read_opcua_node(node: &str) -> Option<String> {
    if let Some(value) = crate::iot_live::read_opcua_node_live(node) {
        return Some(value);
    }
    hub().lock().unwrap().read_opcua_node(node)
}

/// Read a CAN bus frame value from the in-memory IoT hub.
pub fn read_canbus_frame(can_id: u32) -> f64 {
    if let Some(value) = crate::iot_live::read_canbus_frame_live(can_id) {
        return value;
    }
    hub().lock().unwrap().read_canbus_frame(can_id)
}

/// Read a Zigbee attribute from the in-memory IoT hub.
pub fn read_zigbee_attribute(device: &str, cluster: &str) -> String {
    if let Some(value) = crate::iot_live::read_zigbee_attribute_live(device, cluster) {
        return value;
    }
    hub().lock().unwrap().read_zigbee_attribute(device, cluster)
}

/// Read a LoRa payload from the in-memory IoT hub.
pub fn read_lora_payload(device_id: &str) -> String {
    if let Some(value) = crate::iot_live::read_lora_payload_live(device_id) {
        return value;
    }
    hub().lock().unwrap().read_lora_payload(device_id)
}

/// Read a Matter cluster value from the in-memory IoT hub.
pub fn read_matter_cluster(node: &str, cluster: &str) -> f64 {
    if let Some(value) = crate::iot_live::read_matter_cluster_live(node, cluster) {
        return value;
    }
    hub().lock().unwrap().read_matter_cluster(node, cluster)
}

/// Seed demo protocol values for golden-path tests.
pub fn seed_protocol_demos() {
    hub().lock().unwrap().seed_protocol_demo();
}

/// Snapshot hub metrics for tests and diagnostics.
pub fn hub_stats() -> (usize, usize) {
    let hub = hub().lock().unwrap();
    (hub.device_count(), hub.telemetry_count())
}

/// Seed a default Modbus register for golden-path demos.
pub fn seed_modbus_demo_register(address: u16, value: f64) {
    hub().lock().unwrap().write_modbus_register(address, value);
}

/// Extract string argument from runtime values.
pub fn string_arg(args: &[RuntimeValue], index: usize) -> String {
    match args.get(index) {
        Some(RuntimeValue::String { value }) => value.clone(),
        _ => String::new(),
    }
}

/// Extract numeric argument from runtime values.
pub fn number_arg(args: &[RuntimeValue], index: usize) -> f64 {
    match args.get(index) {
        Some(RuntimeValue::Number { value, .. }) => *value,
        _ => 0.0,
    }
}
