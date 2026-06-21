//! Compatibility shim: MQTT live transport moved to `spanda-transport-mqtt`.
//!
pub use spanda_transport_mqtt::LiveMqttBridge as MqttLiveBridge;

use crate::runtime::RuntimeValue;

/// Live MQTT bridge with Spanda runtime value conversion (compatibility shim).
#[derive(Debug, Default)]
pub struct LiveMqttBridge {
    inner: spanda_transport_mqtt::LiveMqttBridge,
}

impl LiveMqttBridge {
    pub fn connect(broker_url: &str, client_id: &str) -> Result<Self, String> {
        Ok(Self {
            inner: spanda_transport_mqtt::LiveMqttBridge::connect(broker_url, client_id)?,
        })
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        self.inner.publish(topic, payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        self.inner.subscribe(topic)
    }

    pub fn receive(&self, topic: &str) -> Option<RuntimeValue> {
        self.inner
            .receive(topic)
            .map(|value| RuntimeValue::String { value })
    }
}
