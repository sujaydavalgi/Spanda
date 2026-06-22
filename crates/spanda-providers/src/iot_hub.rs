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
    hub().lock().unwrap().read_modbus_register(address)
}

/// Read an OPC-UA node value from the in-memory IoT hub.
pub fn read_opcua_node(node: &str) -> Option<String> {
    hub().lock().unwrap().read_opcua_node(node)
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
