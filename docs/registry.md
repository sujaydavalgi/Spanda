# Spanda Package Registry

Spanda's package registry is designed for community frameworks, drivers, adapters, and libraries. A **local stub registry** ships with the toolchain for development; a public registry will follow.

## Searching packages

```bash
spanda registry search ros2
spanda registry search navigation
spanda registry search lidar
```

## Registry packages (local stub)

| Package | Category | Import paths |
|---------|----------|--------------|
| `spanda-ros2` | ros2 | `robotics.ros2` |
| `spanda-vision` | vision | `vision.core` |
| `spanda-navigation` | navigation | `navigation.path_planning` |
| `spanda-mqtt` | mqtt | `communication.mqtt` |
| `spanda-lidar-rplidar` | sensors | `sensors.lidar.rplidar` |
| `spanda-openai` | ai | `ai.openai` |

## Planned framework packages

These are defined in the ecosystem metadata and will be published as the registry matures:

| Package | Description |
|---------|-------------|
| `spanda-ros2` | ROS 2 pub/sub, services, actions |
| `spanda-mqtt` | MQTT transport |
| `spanda-opencv` | OpenCV bindings |
| `spanda-yolo` | YOLO object detection |
| `spanda-slam` | SLAM algorithms |
| `spanda-nav` | Path planning |
| `spanda-manipulation` | Arm manipulation |
| `spanda-hri` | Human-robot interaction |
| `spanda-digital-twin` | Digital twin sync |
| `spanda-sim-gazebo` | Gazebo backend |
| `spanda-sim-webots` | Webots backend |

## Adding dependencies

From registry (local stub):

```bash
spanda add spanda-ros2 --version 0.1.0
spanda install
```

From a local path:

```bash
spanda add my-lib --path ../my-lib
```

From Git:

```bash
spanda add spanda-nav --git https://github.com/spanda/spanda-nav
```

## Dependency resolution

Resolution order:

1. **Local path** — reads `spanda.toml` from the path, locks exact version
2. **Git** — locks URL + branch/tag/rev (no fetch in foundation; metadata only)
3. **Registry** — selects highest version matching semver constraint from local stub

Run `spanda install` after changing dependencies to regenerate `spanda.lock`.

## Publishing (foundation)

```bash
spanda publish
```

Validates manifest, capabilities, hardware requirements, safety level, and license before marking the package publish-ready. A public upload endpoint is not yet available.

## Version constraints

Supported semver operators: exact (`0.1.0`), caret (`^0.1.0`), comparisons (`>=0.1.0, <1.0.0`).

The lockfile pins exact resolved versions for reproducibility.
