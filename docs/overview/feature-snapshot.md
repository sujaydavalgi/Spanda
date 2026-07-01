# Feature status snapshot

[← Overview](./README.md)

Compact snapshot — full matrix: [feature-status.md](../feature-status.md)

| Feature | Status | Notes |
|---------|--------|-------|
| Spanda Language | **Stable** | `.sd` robot programs, units, safety types |
| Parser | **Stable** | Rust authoritative; TypeScript mirror |
| Type Checker | **Stable** | Physical units, `SafeAction` gate |
| CLI | **Stable** | `check`, `verify`, `run`, `sim`, `demo`, packages |
| Safety-Typed AI | **Stable** | `ActionProposal` → `safety.validate()` → `SafeAction` |
| Hardware Verification | **Stable** | `spanda verify` against hardware profiles |
| Capability Verification | **Stable** | Traceability, grants, minimum-hardware analysis |
| Readiness | **Stable** | Weighted go/no-go scoring |
| Assurance | **Stable** | `spanda assure`, assurance cases, mission assurance CLI |
| Diagnosis | **Stable** | `spanda diagnose` on traces and programs |
| Simulation | **Stable** | `spanda run` / `spanda sim`, physics-lite 2D |
| Replay | **Stable** | Mission trace record, deterministic replay |
| Health | **Stable** | `health_check`, fleet `require`, policies |
| Security / Encryption | **Stable** | Capabilities, audit, AES-GCM wire frames; live TLS optional |
| Package System | **Stable** | `spanda install`, `build`, `test`, hosted index |
| Provider Registry | **Stable** | Official packages + dispatch; local mirror |
| Cascading configuration | **Experimental** | `spanda config`, `spanda drift`, `spanda graph`, `spanda deploy gate`, device identity registry, `device discover`, `network scan`, `--config` / `--baseline` on readiness |
| Fleet | **Experimental** | In-process sim stable; distributed HTTP agents experimental |
| IoT | **Experimental** | Live Modbus/OPC-UA env-gated; hub fallback |
| Debugger | **Experimental** | VS Code DAP via `spanda-dap` |
| LLVM | **Experimental** | `spanda ir`, `compile-native` — interpreter is primary runtime |
| WASM | **Experimental** | Browser check/run/verify; limited vs native CLI |
| ROS2 | **Experimental** | rclrs/rclpy bridge; requires ROS Humble setup |
| Control Center | **Stable** | `spanda control-center serve` (REST v1 + optional `--grpc-bind` tonic), embedded UI, Tauri desktop **0.4.2** (`desktop-v0.4.2` GitHub Release) — [desktop-release-runbook.md](../desktop-release-runbook.md) |
| Official SDKs | **Stable** | **0.4.2** on [crates.io/spanda-sdk](https://crates.io/crates/spanda-sdk), [PyPI](https://pypi.org/project/spanda-sdk/), [npm @davalgi-spanda/sdk](https://www.npmjs.com/package/@davalgi-spanda/sdk) |
| Control Center desktop | **Stable** | `@spanda/control-center-desktop` **0.4.2** — tag `desktop-v*` → GitHub Release |
| GitHub Pages / Docs Site | **Experimental** | mdBook under [docs-site/](../../docs-site/); build with `mdbook build docs-site` |
