# Package ecosystem

[← Overview](./README.md)

Spanda extends through a **lean core** plus **37 official packages** under `packages/registry/`.

## Quick workflow

```bash
spanda init my_robot
spanda add spanda-ros2 --version 0.1.0
spanda install
spanda build
spanda test
```

## Categories

| Category | Examples | Import paths |
|----------|----------|--------------|
| Communication | `spanda-mqtt`, `spanda-ros2`, `spanda-dds` | `communication.mqtt`, `robotics.ros2` |
| Positioning | `spanda-gps` | `positioning.gps` |
| Vision & AI | `spanda-opencv`, `spanda-yolo`, `spanda-onnx` | `vision.opencv`, `ai.onnx` |
| Robotics | `spanda-fleet`, `spanda-nav`, `spanda-slam` | `robotics.fleet`, `navigation.slam` |
| Mission assurance | `spanda-anomaly`, `spanda-fusion`, `spanda-diagnosis`, … | `assurance.anomaly`, `assurance.fusion` |
| IoT | `spanda-modbus`, `spanda-opcua`, `spanda-zigbee` | `iot.modbus`, `iot.opcua` |

## Registry

- Hosted index: `registry/index.json` with SHA-256 + Ed25519 signatures
- Rebuild after adding scaffolds: `./scripts/build-registry.sh`
- Override URL: `SPANDA_REGISTRY_URL`

Guides: [packages.md](../packages.md) · [registry.md](../registry.md) · [official-packages.md](../official-packages.md) · [how-packages-work.md](../how-packages-work.md)
