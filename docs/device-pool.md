# Device Pool

The **Device Pool** is Spanda's central inventory for physical hardware across robots, fleets, and swarms.

## Device categories

Track robots, sensors, actuators, accessories, compute modules, controllers, gateways, modems, cameras, GPS/GNSS, BLE devices, CAN devices, and ROS2/DDS/MQTT endpoints.

## Lifecycle states

| State | Meaning |
|-------|---------|
| `discovered` | Seen on the network or bus; not yet trusted |
| `quarantined` | Unknown or failed trust — operator approval required |
| `verified` | Identity and trust validated |
| `assigned` | Mapped to a robot/fleet |
| `active` | Operational (alias: `healthy`) |
| `degraded` | Partial function; monitor closely |
| `offline` | Not reachable |
| `failed` | Hard failure |
| `retired` | Removed from active pool |

## Configuration

Declare devices in `spanda.devices.toml` (hierarchy) and/or `spanda.network-devices.toml` (flat `[[devices]]`).

```toml
[[devices]]
id = "gps-ublox-001"
type = "GPS"
logical_name = "gps"
serial = "UBX-12345"
lifecycle_state = "active"
assigned_robot = "rover-001"
trust_level = "verified"
```

## CLI

```bash
spanda device discover [--subnet 192.168.1.0/24] [--transport mdns]
spanda device inspect gps-ublox-001
spanda device provision gps-ublox-001 --robot rover-001
spanda device assign gps-ublox-001 --robot rover-001 --logical gps
spanda device unassign gps-ublox-001
spanda device quarantine unknown-cam-01
spanda device retire gps-old-001
```

## API

- `GET /v1/devices` — pool entries and summary
- `GET /v1/devices/{id}` — single device record
- `PATCH /v1/devices/{id}` — update lifecycle (Bearer auth)

## Related

- [device-provisioning.md](./device-provisioning.md)
- [device-discovery.md](./device-discovery.md)
- [control-center.md](./control-center.md)
- [configuration.md](./configuration.md)
