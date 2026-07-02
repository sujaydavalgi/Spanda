# Spanda Plugin System

Spanda plugins extend the platform **without modifying core**. They complement the existing **package** and **provider** systems — packages supply Spanda language modules and official providers; plugins add Control Center UI, CLI commands, and lifecycle hooks for readiness, assurance, diagnosis, recovery, trust, health, and telemetry.

## Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                        Spanda CLI / API                      │
├─────────────────────────────────────────────────────────────┤
│  spanda plugin search | install | enable | inspect | trust   │
├─────────────────────────────────────────────────────────────┤
│                     spanda-plugin crate                      │
│  manifest · registry · security · loader · runtime · hooks   │
├──────────────┬──────────────────────────────┬───────────────┤
│ Plugin API   │ Capability enforcement       │ Audit log     │
│ (stable)     │ (deny undeclared access)     │               │
├──────────────┴──────────────────────────────┴───────────────┤
│ Packages & providers (unchanged)                             │
└─────────────────────────────────────────────────────────────┘
```

## Plugin types

| Type | Purpose |
|------|---------|
| `provider` | Extend provider integrations (complements `spanda-providers`) |
| `control-center-ui` | Dashboard panels, entity tabs, routes |
| `cli` | Namespaced CLI commands |
| `readiness` | Mission/readiness hooks |
| `assurance` | Assurance workflow hooks |
| `diagnosis` | Diagnosis completion hooks |
| `recovery` | Recovery workflow hooks |
| `trust` | Trust evaluation extensions |
| `health` | Health change hooks |
| `telemetry` | Telemetry stream extensions |
| `device-discovery` | Device discovery integrations |
| `report-generator` | Report generation |
| `solution-blueprint` | Solution template bundles |

## Loading model (priority)

| Priority | Format | Use case |
|----------|--------|----------|
| **P0 (default)** | Sandboxed **WASM** (`plugin.wasm`) | Production plugins |
| **P1** | Native Rust dynamic library | Trusted local development only |
| **P2** | TypeScript/JS (`index.js`) | Control Center UI plugins only |

Default execution path: **sandboxed WASM**. Manifest-only plugins (metadata + hooks, no artifact) are supported for development and tests.

## Registry trust tiers

| Tier | Meaning |
|------|---------|
| `official` | Spanda-published; signature required when `signed = true` |
| `verified` | Third-party verified publisher |
| `community` | Community registry entry |
| `experimental` | Early access |
| `deprecated` | Still listed; install discouraged |
| `blocked` | Install rejected |

## CLI

```bash
spanda plugin search readiness
spanda plugin install --path examples/plugins/readiness-plugin
spanda plugin enable spanda-plugin-readiness-example
spanda plugin inspect spanda-plugin-readiness-example --json
spanda plugin trust spanda-plugin-readiness-example verified
spanda plugin disable spanda-plugin-readiness-example
spanda plugin uninstall spanda-plugin-readiness-example
```

## Examples

| Example | Path |
|---------|------|
| Readiness | `examples/plugins/readiness-plugin/` |
| Report generator | `examples/plugins/report-plugin/` |
| Device discovery | `examples/plugins/device-discovery-plugin/` |
| Control Center panel | `examples/plugins/control-center-panel/` |
| Healthcare solution | `examples/plugins/healthcare-plugin/` |
| ADAS solution | `examples/plugins/adas-plugin/` |

## Related docs

- [Plugin manifest](plugin-manifest.md)
- [Plugin security](plugin-security.md)
- [Plugin API](plugin-api.md)
- [Plugin development](plugin-development.md)
- [Control Center plugins](control-center-plugins.md)

## Crate layout

Implementation lives in `crates/spanda-plugin/`. Installed plugins are stored under `.spanda/plugins/` in the project tree.
