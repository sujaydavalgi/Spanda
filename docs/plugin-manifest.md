# Plugin Manifest (`spanda.plugin.toml`)

Every plugin ships a **`spanda.plugin.toml`** manifest at its root. This is separate from **`spanda.toml`** (Spanda language packages).

## Minimal example

```toml
[plugin]
name = "spanda-plugin-example"
version = "0.1.0"
publisher = "example"
description = "Example plugin"
license = "Apache-2.0"
type = "readiness"

[compatibility]
spanda_version = ">=0.4.0"
api_version = "v1"

[capabilities]
requires = [
  "entity.read",
  "readiness.read",
  "device.read"
]

[security]
signed = true
sandbox = true
network = false
filesystem = "read-only"
```

## `[plugin]` fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | yes | Unique plugin identifier |
| `version` | yes | Semver plugin version |
| `publisher` | yes | Publisher slug |
| `description` | no | Human-readable summary |
| `license` | no | SPDX license identifier |
| `type` | yes | Plugin type (see [plugins.md](plugins.md)) |

## `[compatibility]`

| Field | Default | Description |
|-------|---------|-------------|
| `spanda_version` | `>=0.4.0` | Semver requirement for host Spanda |
| `api_version` | `v1` | Plugin host API version |

Install is rejected when the running Spanda version or API version is incompatible.

## `[capabilities].requires`

Plugins must declare every host capability they use. The runtime **denies undeclared access**.

Supported capabilities include `entity.read`, `device.read`, `readiness.read`, `health.read`, `report.generate`, `network.outbound`, `filesystem.read`, and write variants.

## `[security]`

| Field | Default | Description |
|-------|---------|-------------|
| `signed` | `false` | Require Ed25519 artifact signature |
| `sandbox` | `true` | Run inside sandbox (WASM default) |
| `network` | `false` | Allow outbound network |
| `filesystem` | `read-only` | Filesystem policy |

## Optional sections

- `[hooks].enabled` — lifecycle and event hooks
- `[control_center]` — panels, entity tabs, routes
- `[cli].commands` — namespaced CLI commands

## Artifacts

| File | Format |
|------|--------|
| `plugin.wasm` | P0 sandboxed WASM (default) |
| `libplugin.so` / `.dylib` / `.dll` | P1 native (dev only) |
| `index.js` | P2 Control Center UI |
