# Lean-Core Roadmap

Phased plan to complete the package-first architecture.

## Phase 1 — Complete ✓

- Provider trait contracts in `spanda-core/src/providers/`
- `ProviderRegistry` and `bootstrap_default_providers()`
- 20 official package scaffolds under `packages/registry/`
- Compatibility shims documented on legacy core modules
- Architecture docs and migration guide
- TypeScript providers mirror and fleet CLI fix

## Phase 2 — Complete ✓

Runtime wiring: `ProviderRegistry` on interpreter, lockfile/manifest official deps, comm-bus sync, package-scoped bootstrap.

## Phase 3 — Complete ✓

Transport, fleet, OTA, connectivity, and deploy-http crates extracted with core shims and registry-backed comm-bus routing.

## Phase 4 — Complete ✓ (kernel)

Compiler/runtime kernel extracted; interpreter remains the composition root in `spanda-core`:

| Crate | Status |
|-------|--------|
| `spanda-hardware` | Done — breaks `spanda-package` → `spanda-core` cycle |
| `spanda-ast` | Done |
| `spanda-lexer` | Done |
| `spanda-typecheck` | Done — `TypeCheckHost` + `CoreTypeCheckHost` |
| `spanda-runtime` | Done — scheduler, provider types, robotics state, `RuntimeValue`, `Environment`, `RuntimeError`, `RuntimeHost` |
| `spanda-core` | Facade — `Interpreter`, `RobotBackend`, full `ProviderRegistry`, domain shims |

The ~8k-line `Interpreter` intentionally stays in core as the orchestration layer until additional subsystems (HAL, safety, transport) expose narrower host traits.

## Phase 5 — Complete ✓ (bootstrap wiring)

All 20 official packages register capabilities; transport, positioning, navigation, SLAM, fleet, ledger, cloud, maintenance, vision, and simulation packages register `*Provider` stubs when installed. Spanda-language `.sd` exports remain scaffolds; live I/O is in workspace crates + core shims. See [official-packages.md](./official-packages.md).

## Phase 6 — Complete ✓

TypeScript parity: `bootstrapProvidersForPackages()`, registry-backed `RoutingCommBus`, interpreter `officialPackages` / `providerRegistry`, full classification table, `tests/providers-comm.test.ts`.

## Technical debt addressed

| Item | Status |
|------|--------|
| `cargo clippy --workspace -D warnings` | Green |
| `cargo test --workspace` | Green |
| `npm test` | Green |
| Example regression (`scripts/check_all_examples.sh`) | 162 pass, 2 expected-fail, 0 skips |
| `lean_core_cycle` cargo tree guard | Done |
| Clippy / visibility / API hygiene | Fixed across hardware, fleet, ota, core, cli |
| Transport `TransportAdapter` impls | Moved to `spanda-transport-{ros2,mqtt,dds,websocket}`; `lean_core_shims` guards `transport.rs` |
| Nav2/SLAM adapter bridge | Moved from `spanda-core` to `spanda-connectivity::adapter_bridge` |
| ROS2 rclrs transport | Consolidated in `spanda-transport-ros2` (`rclrs.rs`); removed `transport_rclrs*.rs` from core |
| Unused TLS deps in `spanda-core` | Removed (`rustls`, `rustls-pemfile`, `webpki-roots`, `hex`); TLS lives in `spanda-transport` / deploy crates |

### Example repairs — Complete ✓

All 20 previously skipped examples now pass `spanda check`. The manifest retains only two `expect-fail` negative tests (`ai_safety_violation.sd`).

## Success criteria

- [x] `cargo test --workspace` green
- [x] `npm test` green
- [x] Example regression script in CI (162 + 2 negative tests)
- [x] `spanda-package` does not depend on `spanda-core`
- [x] Every official package has bootstrap registration or documented stub status
- [ ] Zero protocol-specific code in core except traits + wire types (ongoing shim deprecation — transport adapters, ROS2 rclrs, and live bridges in `spanda-transport-*`; Nav2/SLAM bridge in `spanda-connectivity`; core `transport.rs` is routing-only with re-exports)

See also: [lean-core.md](./lean-core.md), [migration.md](./migration.md#lean-core-package-first-refactor)
