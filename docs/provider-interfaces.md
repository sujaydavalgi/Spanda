# Provider Interfaces

Spanda Core defines generic provider traits in `crates/spanda-core/src/providers/`. Official packages implement these traits and register with `ProviderRegistry` at runtime.

## Trait overview

| Trait | Purpose | Example package |
|-------|---------|-----------------|
| `SensorProvider` | Read sensor samples | `spanda-gps`, sensor driver packages |
| `ActuatorProvider` | Execute motion and e-stop | Robot driver packages |
| `ConnectivityProvider` | Wi-Fi, BLE, cellular state | `spanda-wifi`, `spanda-ble`, `spanda-cellular` |
| `PositioningProvider` | GPS/GNSS fixes | `spanda-gps` |
| `TransportProvider` | Pub/sub, services, actions | `spanda-mqtt`, `spanda-ros2`, `spanda-dds` |
| `CryptoProvider` | Hash, sign, verify | Delegates to `spanda-security` |
| `NavigationProvider` | Path planning, Nav2 | `spanda-nav` |
| `SlamProvider` | Localization and mapping | `spanda-slam` |
| `VisionProvider` | Detection, classification, embed | `spanda-opencv`, `spanda-yolo` |
| `FleetProvider` | Multi-robot orchestration | `spanda-fleet` |
| `SimulationProvider` | Physics sim backends | `spanda-gazebo`, `spanda-webots` |
| `MaintenanceProvider` | Health metrics and reports | `spanda-maintenance` |
| `LedgerProvider` | Audit anchoring | `spanda-ledger` |
| `CloudProvider` | Upload and remote invoke | `spanda-cloud` |
| `RosProvider` | ROS node lifecycle | `spanda-ros2` |
| `HalProvider` | I2C/SPI/GPIO/UART | Board-specific HAL packages |

## Legacy trait mapping

Existing traits remain; provider traits unify the extension surface:

| Legacy trait | Provider trait |
|--------------|----------------|
| `TransportAdapter` | `TransportProvider` (via `TransportAdapterProvider`) |
| `RobotBackend` | `SensorProvider` + `ActuatorProvider` |
| `HalBackend` | `HalProvider` |
| `AiProvider` | `VisionProvider` (re-exported from `providers::traits`) |

## TransportAdapter bridge

Wrap any existing `TransportAdapter` without rewriting:

```rust
use spanda_core::providers::{ProviderRegistry, TransportAdapterProvider};
use spanda_core::transport::{MqttTransportAdapter, TransportConfig};

let mut registry = ProviderRegistry::new();
registry.register_transport(Box::new(TransportAdapterProvider::new(
    "spanda-mqtt",
    "default",
    MqttTransportAdapter::default(),
)));

if let Some(transport) = registry.transport("spanda-mqtt::default") {
    transport.connect(&TransportConfig::default())?;
}
```

## Provider metadata

Each provider exposes `ProviderMetadata`:

- `id` — `{ package, name }` stable key
- `description` — human-readable summary
- `safety_level` — `Experimental`, `Development`, or `Production`
- `capabilities_required` — runtime capability tokens
- `hardware_requirements` — profile tokens

## Registry API

```rust
use spanda_core::providers::ProviderRegistry;

let mut registry = ProviderRegistry::new();
registry.grant_capability("mqtt.publish");
registry.register_transport(/* ... */);
registry.list_transports(); // Vec<ProviderId>
```

## Package integration

In `.sd` packages, declare capabilities in `spanda.toml`:

```toml
[capabilities]
required = ["mqtt.publish", "mqtt.subscribe"]

[safety]
level = "development"
```

Use `spanda verify-adapter` to validate `[adapter]` sections against expected metadata.

## Related docs

- [lean-core.md](./lean-core.md) — architecture overview
- [official-packages.md](./official-packages.md) — package catalog
- [packages.md](./packages.md) — manifest schema
