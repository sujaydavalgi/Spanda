//! Bootstrap default provider registrations from core compatibility shims.
//!
use super::registry::ProviderRegistry;
use super::traits::TransportAdapterProvider;
use crate::comm::TransportKind;
use crate::transport::{
    DdsTransportAdapterLive, MqttTransportAdapter, Ros2TransportAdapter, TransportConfig,
    WebsocketTransportAdapterLive,
};

fn register_transport_stub(
    registry: &mut ProviderRegistry,
    package: &str,
    adapter: impl crate::transport::TransportAdapter + Send + Sync + 'static,
) {
    registry.register_transport(Box::new(TransportAdapterProvider::new(
        package,
        "project",
        adapter,
    )));
}

/// Register built-in transport shims so legacy programs work without installed packages.
pub fn bootstrap_default_providers() -> ProviderRegistry {
    bootstrap_providers_for_packages(&[])
}

/// Build a provider registry from installed official package names.
pub fn bootstrap_providers_for_packages(package_names: &[&str]) -> ProviderRegistry {
    // Build a provider registry from installed official package names.
    //
    // Parameters:
    // - `package_names` — dependency keys from `spanda.toml` / `spanda.lock`
    //
    // Returns:
    // Registry with default shims plus project-scoped transports for official packages.
    //
    // Options:
    // None.
    //
    // Example:
    // let registry = bootstrap_providers_for_packages(&["spanda-ros2"]);

    let mut registry = ProviderRegistry::new();
    registry.set_official_packages(
        package_names
            .iter()
            .map(|name| (*name).to_string())
            .collect(),
    );
    registry.grant_capability("mqtt.publish");
    registry.grant_capability("mqtt.subscribe");
    registry.grant_capability("comm.ros2.publish");
    registry.grant_capability("comm.ros2.subscribe");

    let names: std::collections::HashSet<&str> = package_names.iter().copied().collect();
    let include_all = names.is_empty();

    if include_all || names.contains("spanda-mqtt") {
        register_transport_stub(
            &mut registry,
            "spanda-mqtt",
            MqttTransportAdapter::default(),
        );
    }
    if include_all || names.contains("spanda-ros2") {
        register_transport_stub(
            &mut registry,
            "spanda-ros2",
            Ros2TransportAdapter::default(),
        );
    }
    if names.contains("spanda-dds") {
        registry.grant_capability("dds.publish");
        registry.grant_capability("dds.subscribe");
        register_transport_stub(
            &mut registry,
            "spanda-dds",
            DdsTransportAdapterLive::default(),
        );
    }
    if names.contains("spanda-ble") || names.contains("spanda-wifi") {
        registry.grant_capability("connectivity.wifi");
        registry.grant_capability("connectivity.ble");
        register_transport_stub(
            &mut registry,
            "spanda-ble",
            WebsocketTransportAdapterLive::default(),
        );
    }
    if names.contains("spanda-gps") {
        registry.grant_capability("positioning.read");
    }
    if names.contains("spanda-nav") || names.contains("spanda-nav2") {
        registry.grant_capability("navigation.plan");
    }
    if names.contains("spanda-slam") {
        registry.grant_capability("slam.localize");
        registry.grant_capability("slam.map");
    }
    if names.contains("spanda-fleet") {
        registry.grant_capability("fleet.orchestrate");
    }
    if names.contains("spanda-ota") {
        registry.grant_capability("deploy.rollout");
    }
    if names.contains("spanda-ledger") {
        registry.grant_capability("audit.append");
    }
    if names.contains("spanda-cloud") {
        registry.grant_capability("cloud.invoke");
    }

    registry
}

/// Map a transport kind to the official package that backs it when installed.
pub fn official_package_for_transport(kind: TransportKind) -> Option<&'static str> {
    match kind {
        TransportKind::Ros2 => Some("spanda-ros2"),
        TransportKind::Mqtt => Some("spanda-mqtt"),
        TransportKind::Dds => Some("spanda-dds"),
        TransportKind::Websocket => Some("spanda-ble"),
        TransportKind::Local | TransportKind::Sim => None,
    }
}

/// Connect comm-bus adapters that correspond to installed official packages.
pub fn sync_comm_bus_for_official_packages(
    comm_bus: &mut crate::transport::RoutingCommBus,
    package_names: &[String],
) {
    let config = TransportConfig::default();
    for name in package_names {
        match name.as_str() {
            "spanda-ros2" => {
                if let Some(adapter) = comm_bus.adapter_mut(TransportKind::Ros2) {
                    let _ = adapter.connect(&config);
                }
            }
            "spanda-mqtt" => {
                if let Some(adapter) = comm_bus.adapter_mut(TransportKind::Mqtt) {
                    let _ = adapter.connect(&TransportConfig {
                        broker_url: Some("mqtt://localhost:1883".into()),
                        client_id: Some("spanda".into()),
                        ..config.clone()
                    });
                }
            }
            "spanda-dds" => {
                if let Some(adapter) = comm_bus.adapter_mut(TransportKind::Dds) {
                    let _ = adapter.connect(&TransportConfig {
                        domain_id: Some(0),
                        ..config.clone()
                    });
                }
            }
            "spanda-ble" | "spanda-wifi" => {
                if let Some(adapter) = comm_bus.adapter_mut(TransportKind::Websocket) {
                    let _ = adapter.connect(&TransportConfig {
                        broker_url: Some("ws://localhost:9090".into()),
                        ..config.clone()
                    });
                }
            }
            _ => {}
        }
    }
}
