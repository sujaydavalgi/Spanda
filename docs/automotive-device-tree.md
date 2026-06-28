# Automotive Device Tree

Device hierarchy and capability mapping for the ADAS Solution Blueprint.

**Fixture:** `examples/solutions/adas/spanda.devices.toml` · **CLI:** `spanda device-tree inspect vehicle-001 --config spanda.toml`

---

## Hierarchy

```
Vehicle (vehicle-001)
├── Compute Platform (compute-001, JetsonAutomotive)
│   ├── Front Camera          → lane_detection, traffic_sign_recognition, pedestrian_detection
│   ├── Rear Camera           → obstacle_detection, parking_assist
│   ├── Stereo Camera         → obstacle_detection, parking_assist
│   ├── Front Radar           → obstacle_detection, adaptive_speed_control
│   ├── Front LiDAR           → obstacle_detection, localization
│   ├── GPS Receiver          → localization, route_following
│   ├── IMU                   → localization
│   ├── Driver Monitor Camera → driver_monitoring
│   ├── Drive Controller      → move, stop, emergency_stop (DifferentialDrive)
│   ├── Vehicle CAN Bus       → steering_control, emergency_braking, adaptive_speed_control
│   └── Comm Gateway (MQTT)   → secure_communication, telemetry_streaming
```

---

## Capability mapping

| Logical capability | Primary device | Redundant device |
|--------------------|----------------|------------------|
| `lane_detection` | Front camera | Stereo camera |
| `obstacle_detection` | Front radar | Front LiDAR, front camera |
| `emergency_braking` | Brake ECU | — |
| `adaptive_speed_control` | Front radar + powertrain ECU | Front LiDAR |
| `steering_control` | Steering ECU | Differential drive |
| `localization` | GPS + IMU | Front LiDAR |
| `route_following` | GPS | IMU + visual odometry |
| `driver_monitoring` | Driver monitor camera | — |
| `parking_assist` | Stereo camera, rear camera | Front LiDAR |

---

## Device types

| Type | Spanda sensor/actuator | Provider (registered) |
|------|------------------------|------------------------|
| Camera | `Camera` | `spanda-opencv` |
| DepthCamera | `DepthCamera` | `spanda-opencv` |
| Radar | `Radar` | `spanda-slam` (stub) |
| Lidar | `Lidar` | `spanda-slam` |
| GPS | `GPS` | `spanda-gps` |
| IMU | `IMU` | `spanda-fusion` |
| DifferentialDrive | `DifferentialDrive` | `spanda-canbus` |
| CommunicationGateway | — | `spanda-canbus`, `spanda-mqtt` |

Planned providers (`spanda-radar`, `spanda-lidar`, `spanda-ultrasonic`, `spanda-automotive-ethernet`) are listed in `spanda.providers.toml` with `status = "experimental"` or `planned`.

Wheel speed, steering angle, brake, and tire pressure sensors attach to ECUs via CAN and surface as health-check inputs.

---

## CLI

```bash
spanda device-tree inspect vehicle-001 --config examples/solutions/adas/spanda.toml
spanda device-tree graph --config examples/solutions/adas/spanda.toml --json
```

Control Center: `GET /v1/device-tree` with `--config spanda.toml`.

---

## Application variants

Adjust the device tree per application without changing core types:

| Application | Add / remove devices | Fixture |
|-------------|----------------------|---------|
| Passenger vehicle | Full suite (default) | `applications/passenger/` → `spanda.devices.toml` |
| Commercial truck | Long-range radar, rear radar | `applications/truck/` |
| Autonomous shuttle | Geo-fenced pedestrian focus | `applications/shuttle/` |
| Mining vehicle | Ruggedized LiDAR, redundant GPS | `applications/mining/` |
| Agricultural | RTK GPS, remove highway radar | `applications/agricultural/` |
| Delivery | Urban cameras, parking assist | `applications/delivery/` |
| Airport ground | Remove driver monitor, geofence gateway | `applications/airport/` |
| Campus shuttle | Multi-camera pedestrian suite | `applications/campus/` |
| Construction | 360° LiDAR, machinery safety | `applications/construction/` |

Copy `spanda.devices.toml` and edit `[[fleet.robots.compute.devices]]` entries for each deployment.

---

## Related

- [device-tree.md](./device-tree.md) — General device tree reference
- [solutions/adas.md](./solutions/adas.md) — ADAS blueprint architecture
- [configuration.md](./configuration.md) — Cascading config layers
