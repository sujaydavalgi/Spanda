//! Compatibility shim: WebSocket live transport moved to `spanda-transport-websocket`.
//!
use crate::runtime::RuntimeValue;

/// Live WebSocket bridge with Spanda runtime value conversion (compatibility shim).
#[derive(Debug, Default)]
pub struct LiveWebsocketBridge {
    inner: spanda_transport_websocket::LiveWebsocketBridge,
}

impl LiveWebsocketBridge {
    pub fn connect(broker_url: &str) -> Result<Self, String> {
        Ok(Self {
            inner: spanda_transport_websocket::LiveWebsocketBridge::connect(broker_url)?,
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
