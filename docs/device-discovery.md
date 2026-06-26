# Device Discovery

Spanda discovers devices through **provider interfaces** — core ships built-in transports; live backends ship as registry packages.

## Built-in transports

| Transport | CLI/API flag | Notes |
|-----------|--------------|-------|
| Subnet scan | `subnet` | TCP probe on common ports |
| mDNS | `mdns` | Stub in core; live in `spanda-discovery-mdns` |
| BLE | `ble` | Contract stub; package-backed |
| USB | `usb` | Enumeration stub |
| CAN | `can` | Bus scan stub |
| MQTT | `mqtt` | Broker discovery stub |
| ROS2/DDS | `ros2` | Topic discovery stub |

## CLI

```bash
spanda device discover --subnet 192.168.1.0/24
spanda device discover --transport mdns --json
```

## API

```bash
# GET (query params)
curl "http://127.0.0.1:8080/v1/discovery?transport=subnet&subnet=192.168.1.0/24"

# POST (multi-transport)
curl -X POST http://127.0.0.1:8080/v1/devices/discover \
  -H "Content-Type: application/json" \
  -d '{"subnet":"192.168.1.0/24","transports":["subnet","mdns"]}'
```

## Provider contract

Packages implement `DeviceDiscoveryTransport`:

```rust
pub trait DeviceDiscoveryTransport: Send + Sync {
    fn transport_name(&self) -> &'static str;
    fn discover(&self, options: &DiscoveryOptions) -> Result<DiscoveryTransportResult, String>;
}
```

Register packages in `spanda.providers.toml`; do not hardcode vendor logic in `spanda-config`.

## Unknown devices

Newly discovered devices enter **quarantined** or **discovered** state with `trust_level = unknown` until an operator approves trust.

## Related

- [device-pool.md](./device-pool.md)
- [device-quarantine.md](./device-quarantine.md)
- [packages.md](./packages.md)
