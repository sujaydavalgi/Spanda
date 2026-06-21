# Official Packages

First-party Spanda packages live under `packages/registry/`. Each package includes `spanda.toml`, source exports, tests, and a README.

## Connectivity and positioning

| Package | Import path | Description |
|---------|-------------|-------------|
| `spanda-gps` | `positioning.gps` | GPS/GNSS receiver adapters |
| `spanda-wifi` | `connectivity.wifi` | Wi-Fi connectivity |
| `spanda-ble` | `connectivity.ble` | Bluetooth Low Energy |
| `spanda-cellular` | `connectivity.cellular` | LTE/4G/5G cellular |

## Communication and robotics middleware

| Package | Import path | Description |
|---------|-------------|-------------|
| `spanda-mqtt` | `communication.mqtt` | MQTT pub/sub transport |
| `spanda-dds` | `communication.dds` | DDS transport |
| `spanda-ros2` | `robotics.ros2` | ROS 2 integration |

## Navigation, SLAM, manipulation

| Package | Import path | Description |
|---------|-------------|-------------|
| `spanda-nav` | `navigation.path_planning` | Path planning and navigation |
| `spanda-slam` | `navigation.slam` | SLAM localization and mapping |
| `spanda-moveit` | `manipulation.moveit` | MoveIt motion planning |

Specialized adapter packages (examples under `examples/packages/`):

- `spanda-nav2` → `navigation.nav2`
- `spanda-cartographer` → `navigation.cartographer`
- `spanda-rtabmap` → `navigation.rtabmap`

## Vision and AI

| Package | Import path | Description |
|---------|-------------|-------------|
| `spanda-opencv` | `vision.opencv` | OpenCV bindings |
| `spanda-yolo` | `vision.yolo` | YOLO object detection |
| `spanda-openai` | `ai.openai` | OpenAI LLM via Python bridge |

## Simulation

| Package | Import path | Description |
|---------|-------------|-------------|
| `spanda-gazebo` | `sim.gazebo` | Gazebo backend |
| `spanda-webots` | `sim.webots` | Webots backend |

Aliases: `spanda-sim-gazebo`, `spanda-sim-webots` (registry metadata).

## Platform services

| Package | Import path | Description |
|---------|-------------|-------------|
| `spanda-fleet` | `robotics.fleet` | Multi-robot fleet orchestration |
| `spanda-ota` | `deploy.ota` | OTA deploy and rollout |
| `spanda-maintenance` | `maintenance.health` | Predictive maintenance |
| `spanda-ledger` | `provenance.ledger` | Audit ledger anchoring |
| `spanda-cloud` | `cloud.remote` | Cloud telemetry and remote commands |

## Package layout

```
packages/registry/spanda-gps/
├── spanda.toml
├── README.md
├── src/
│   └── positioning_gps.sd
├── tests/
│   └── smoke.sd
└── examples/
```

## Adding a dependency

```toml
# spanda.toml
[dependencies]
spanda-gps = "0.1"
```

```spanda
import positioning.gps;
```

## Verify adapter metadata

```bash
spanda verify-adapter packages/registry/spanda-ros2
spanda registry info spanda-mqtt
```

## Status

Scaffold packages export minimal symbols and pass smoke tests. Live vendor backends remain in core compatibility shims until each package gains a full implementation. See [migration.md](./migration.md#lean-core-package-first-refactor).

## Related docs

- [packages.md](./packages.md) — package manager
- [provider-interfaces.md](./provider-interfaces.md) — trait contracts
- [registry.md](./registry.md) — hosted registry
