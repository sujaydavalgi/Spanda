# ADAS & Autonomous Driving — Official Solution Blueprint

Demonstrates how Spanda supports **Safety · Verification · Readiness · Assurance · Diagnosis · Recovery · Trust · Mission Continuity · Explainability · Traceability** for intelligent vehicles — built entirely on existing platform capabilities. No automotive-specific logic in the core language.

**Profile:** ISO 26262 template (`iso26262`) · **Compliance:** engineering template, not legal certification.

---

## Quick start

```bash
cd examples/solutions/adas
spanda install
spanda check src/highway_drive.sd
spanda verify src/highway_drive.sd --profile iso26262 --capabilities --traceability --json
spanda readiness src/highway_drive.sd --profile iso26262 --config spanda.toml --json
spanda sim src/highway_drive.sd --record
spanda replay src/highway_drive.trace --deterministic
spanda diagnose src/highway_drive.sd src/highway_drive.trace
spanda explain src/highway_drive.trace
spanda compliance report src/highway_drive.sd --profile iso26262
spanda control-center serve --config spanda.toml --program src/highway_drive.sd
```

One command: `spanda demo adas` · Smoke: `./scripts/adas_smoke.sh`

---

## Blueprint layout

```
adas/
├── README.md                    # This file
├── spanda.toml                  # Package deps, hardware targets, capability requirements
├── spanda.devices.toml          # Automotive device hierarchy
├── spanda.readiness.toml        # Readiness thresholds (ISO 26262 aligned)
├── spanda.assurance.toml        # Assurance evidence configuration
├── spanda.security.toml         # Security and trust policies
├── spanda.providers.toml        # Optional automotive protocol packages (future)
└── src/
    ├── highway_drive.sd         # Primary reference program
    └── highway_drive.trace      # Golden replay fixture
```

---

## ADAS function examples

| Directory | Function | Demonstrates |
|-----------|----------|--------------|
| [`lane_keeping/`](./lane_keeping/) | Lane Keeping Assist | `lane_detection`, steering control, readiness gate |
| [`adaptive_cruise/`](./adaptive_cruise/) | Adaptive Cruise Control | `adaptive_speed_control`, obstacle detection |
| [`automatic_emergency_braking/`](./automatic_emergency_braking/) | Automatic Emergency Braking | `emergency_braking`, safety validation, audit trail |
| [`sensor_failure_recovery/`](./sensor_failure_recovery/) | Sensor failure recovery | Mission continuity, degraded mode, self-healing |
| [`driver_takeover/`](./driver_takeover/) | Driver takeover | Continuity framework, driver monitoring |

---

## Supported applications

Reference configurations for nine vehicle classes (see [docs/solutions/adas.md](../../../docs/solutions/adas.md#applications)):

Passenger vehicles · Commercial trucks · Autonomous shuttles · Mining vehicles · Agricultural vehicles · Delivery vehicles · Airport ground vehicles · Campus mobility · Construction equipment

---

## Sensor ecosystem

Modeled through hardware profiles and device-tree capability definitions — not core language extensions:

| Sensor | Device type | Capabilities |
|--------|-------------|--------------|
| Front camera | Camera | `lane_detection`, `traffic_sign_recognition`, `pedestrian_detection` |
| Stereo camera | DepthCamera | `obstacle_detection`, `parking_assist` |
| Radar | Radar | `obstacle_detection`, `adaptive_speed_control` |
| LiDAR | Lidar | `obstacle_detection`, `localization` |
| Ultrasonic | Ultrasonic | `parking_assist` |
| GPS/GNSS | GPS | `localization`, `route_following` |
| IMU | IMU | `localization` |
| Driver monitoring camera | Camera | `driver_monitoring` |

---

## Vehicle capabilities

Logical capabilities verified through the existing capability framework:

`lane_detection` · `obstacle_detection` · `emergency_braking` · `adaptive_speed_control` · `steering_control` · `localization` · `route_following` · `driver_monitoring` · `parking_assist`

---

## Simulation scenarios

Record traces for replay and diagnosis:

```bash
spanda sim lane_keeping/lane_keeping.sd --record --fault camera_obstructed
spanda sim sensor_failure_recovery/camera_failure.sd --record
spanda replay src/highway_drive.trace --deterministic
```

Scenarios: heavy rain, snow, fog, night driving, camera/radar/LiDAR failure, GPS spoofing, CAN bus failure, emergency vehicle encounter.

---

## Documentation

| Guide | Topic |
|-------|-------|
| [docs/solutions/adas.md](../../../docs/solutions/adas.md) | Architecture, workflows, applications |
| [docs/automotive-device-tree.md](../../../docs/automotive-device-tree.md) | Device hierarchy and capability mapping |
| [docs/adas-readiness.md](../../../docs/adas-readiness.md) | Pre-drive readiness gates |
| [docs/adas-assurance.md](../../../docs/adas-assurance.md) | Assurance evidence bundles |
| [docs/adas-security.md](../../../docs/adas-security.md) | CAN intrusion, OTA, spoofing |
| [docs/adas-replay.md](../../../docs/adas-replay.md) | Collision, takeover, recovery replay |

---

## Control Center

Launch with the ADAS blueprint config:

```bash
spanda control-center serve --config spanda.toml --program src/highway_drive.sd
```

Open the **ADAS** tab for vehicle health, sensor health, readiness, trust score, alerts, takeover requests, OTA status, replay viewer, and assurance reports.

---

## Related

- [automotive_rover.sd](../../showcase/compliance/automotive_rover.sd) — ISO 26262 compliance showcase
- [compliance-profiles.md](../../../docs/compliance-profiles.md) — Profile definitions
- [mission-continuity.md](../../../docs/mission-continuity.md) — Continuity framework
- [provider-interfaces.md](../../../docs/provider-interfaces.md) — Optional automotive protocol packages
