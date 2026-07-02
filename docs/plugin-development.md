# Plugin Development

## Quick start

```bash
spanda plugin install --path my-plugin/
spanda plugin enable my-plugin-name
spanda plugin inspect my-plugin-name
```

## Project layout

```text
my-readiness-plugin/
├── spanda.plugin.toml
├── plugin.wasm          # P0 production artifact
└── README.md
```

## WASM (recommended)

Default loader uses Wasmtime for `plugin.wasm`. Network is disabled in the default WASM sandbox.

## Native (local dev only)

Enable `native-loader` on `spanda-plugin`. **Not for production.**

## TypeScript (Control Center)

For `control-center-ui` plugins, ship `index.js` and declare `[control_center]` panels.

## Testing

```bash
cargo test -p spanda-plugin
```

## Do not replace packages

- **Packages** (`spanda.toml`) — language modules and providers
- **Plugins** — dynamic platform extension (UI, CLI, hooks)
