# Plugin API

Plugins access Spanda platform services through a **stable host API** in `spanda-plugin`. Internal workspace crates are **not** exposed.

## Surfaces

| API surface | Capability |
|-------------|------------|
| Entity | `entity.read` |
| Readiness | `readiness.read` |
| Assurance | `assurance.read` |
| Diagnosis | `diagnosis.read` |
| Recovery | `recovery.read` |
| Health | `health.read` |
| Trust | `trust.read` |
| Telemetry | `telemetry.read` |
| Report | `report.generate` |

## Host context (Rust)

```rust
use spanda_plugin::api::PluginApiContext;

let ctx = PluginApiContext::new(&manifest.plugin.name, manifest.capability_set());
let readiness = ctx.readiness_read("mission-42")?;
```

Undeclared access returns `PluginError::Capability`.

## Hooks

| Hook | Trigger |
|------|---------|
| `on_install` | After install |
| `on_enable` | After enable |
| `on_disable` | After disable |
| `on_uninstall` | Before removal |
| `on_entity_event` | Entity model events |
| `on_health_changed` | Health transitions |
| `on_readiness_completed` | Readiness completion |
| `on_diagnosis_completed` | Diagnosis completion |
| `on_recovery_completed` | Recovery completion |
| `on_report_requested` | Report request |

## API versioning

Current stable API: **`v1`**. Set `api_version = "v1"` in manifest.
