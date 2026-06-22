//! Pluggable transport routing for ROS2, MQTT, DDS, and WebSocket.
//!
//! **Lean-core note:** adapter traits and wire frames live in `spanda-transport`.
//! `RoutingCommBus` lives in `spanda-transport-routing` (depends on adapter backends).
//!
pub use spanda_transport::{
    decode_wire_value, encode_wire_value, payload_string_for_service, AdapterMessage,
    StubTransportState, TransportAdapter, TransportConfig, TransportWireFrame,
};
pub use spanda_transport_dds::{DdsTransportAdapter, DdsTransportAdapterLive};
pub use spanda_transport_mqtt::MqttTransportAdapter;
pub use spanda_transport_ros2::Ros2TransportAdapter;
pub use spanda_transport_websocket::{WebsocketTransportAdapter, WebsocketTransportAdapterLive};
pub use spanda_transport_routing::RoutingCommBus;
