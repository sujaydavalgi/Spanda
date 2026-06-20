# spanda.toml Reference

The `spanda.toml` manifest describes a Spanda package — its metadata, dependencies, hardware requirements, capabilities, and safety level.

## Minimal manifest

```toml
[package]
name = "my_robot"
version = "0.1.0"
description = "My first Spanda robot"
license = "Apache-2.0"
```

## `[package]`

| Field | Required | Description |
|-------|----------|-------------|
| `name` | yes | Package identifier (snake_case) |
| `version` | yes | Semver version (e.g. `0.1.0`) |
| `description` | no | Human-readable summary |
| `license` | no | SPDX license identifier |
| `authors` | no | List of author strings |

## `[dependencies]`

Map of package names to version constraints or source tables.

```toml
[dependencies]
spanda-ros2 = "0.1.0"
spanda-vision = ">=0.1.0, <0.2.0"
local-lib = { path = "../local-lib" }
spanda-nav = { git = "https://github.com/spanda/spanda-nav", branch = "main" }
```

Version constraints follow [SemVer](https://semver.org/) (`^`, `>=`, exact).

## `[dev-dependencies]`

Same format as `[dependencies]`, used only for development and testing.

## `[hardware]`

```toml
[hardware]
targets = ["RoverV1", "JetsonOrin"]
```

Lists hardware profiles this package is designed for. Validated against built-in profiles from `spanda verify`.

## `[capabilities]`

```toml
[capabilities]
uses = ["network.outbound", "camera.read", "lidar.read"]
required = ["motion.propose", "actuator.execute.safe"]
```

| Field | Description |
|-------|-------------|
| `uses` | Capabilities the package consumes at runtime |
| `required` | Capabilities the consuming application must grant |

Known capabilities include: `network.outbound`, `network.inbound`, `camera.read`, `lidar.read`, `imu.read`, `gps.read`, `motion.propose`, `actuator.execute`, `actuator.execute.safe`, `serial.port`, `storage.read`, `storage.write`, `ai.inference`, `ros2.publish`, `ros2.subscribe`.

## `[requires_hardware]`

```toml
[requires_hardware]
memory = ">=2GB"
storage = ">=512MB"
gpu = ">=1 TOPS"
sensors = ["Camera", "Lidar"]
actuators = ["DriveUnit"]
```

Integrates with the hardware compatibility verifier. Memory/storage accept `MB` or `GB` suffixes with optional `>=` prefix. GPU accepts `>=N TOPS`.

## `[safety]`

```toml
[safety]
level = "experimental"
requires_review = true
can_control_actuators = false
```

| Level | `can_control_actuators` default | `requires_review` default |
|-------|--------------------------------|---------------------------|
| `experimental` | false | true |
| `simulation_only` | false | true |
| `hardware_safe` | true | false |
| `certified` | true | false |

## `[adapter]`

For driver/adapter packages:

```toml
[adapter]
provides = ["LidarAdapter", "Topic<LidarScan>"]
requires = ["serial.port", "lidar.read"]
```

## `categories`

Top-level array of package categories:

```toml
categories = ["robotics", "sensors", "navigation"]
```

Valid values: `ai`, `robotics`, `vision`, `navigation`, `manipulation`, `simulation`, `ros2`, `mqtt`, `hardware`, `sensors`, `actuators`, `digital-twin`, `safety`, `hri`, `testing`.

## `license_compat`

Optional list of compatible licenses for dependency resolution:

```toml
license_compat = ["Apache-2.0", "MIT"]
```

## Lockfile

After `spanda install`, a `spanda.lock` JSON file records resolved versions:

```json
{
  "version": 1,
  "package": { "name": "my_robot", "version": "0.1.0" },
  "dependencies": {
    "spanda-ros2": {
      "name": "spanda-ros2",
      "version": "0.1.0",
      "source": { "kind": "registry", "registry": "local" }
    }
  }
}
```

Commit `spanda.lock` for reproducible builds.
