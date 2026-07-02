# Control Center Plugins

Control Center UI plugins extend the operator console with dashboards, entity views, and configuration pages.

## Plugin type

```toml
[plugin]
type = "control-center-ui"
```

## Contributions

```toml
[control_center]
panels = [
  { id = "vitals-dashboard", title = "Vitals Dashboard", component = "VitalsDashboard" }
]
entity_tabs = [
  { id = "patient-vitals", title = "Vitals", component = "PatientVitalsTab" }
]
routes = [
  { path = "/plugins/vitals", title = "Vitals" }
]
```

## Examples

- **Healthcare** — `examples/plugins/healthcare-plugin/` (patient devices, vitals)
- **ADAS** — `examples/plugins/adas-plugin/` (vehicle readiness, sensor calibration)
- **Generic panel** — `examples/plugins/control-center-panel/`

## Install

```bash
spanda plugin install --path examples/plugins/healthcare-plugin
spanda plugin enable spanda-plugin-healthcare
```

UI plugins use the same capability and security model as other plugins. See [plugin-security.md](plugin-security.md).
