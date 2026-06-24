# Workspace layers

[← Overview](./README.md) · Pipeline: [architecture.md](./architecture.md)

Rust workspace layout for the lean-core platform (Phases 1–35 complete).

| Layer | Key crates | Responsibility |
|-------|------------|----------------|
| **Public facade** | `spanda-core` | Stable re-exports + thin shims for embedders |
| **Apps** | `spanda-cli`, `spanda-node`, `spanda-wasm`, `spanda-dap`, `spanda-llvm` | Native CLI, Node/WASM bindings, DAP debugger, LLVM codegen |
| **Pipeline** | `spanda-driver`, `spanda-lexer`, `spanda-parser`, `spanda-ast`, `spanda-typecheck`, `spanda-sir`, `spanda-error` | Lex → parse → AST → check → SIR |
| **Runtime** | `spanda-interpreter`, `spanda-runtime`, `spanda-runtime-host`, `spanda-comm`, `spanda-safety`, `spanda-hal`, `spanda-concurrency` | Tree-walking execution, scheduling, safety, HAL |
| **Transport** | `spanda-transport-routing`, `spanda-transport-mqtt`, `spanda-transport-ros2`, … | Comm adapters and live bridges |
| **Domain** | `spanda-hardware`, `spanda-fleet`, `spanda-ota`, `spanda-certify`, `spanda-connectivity`, `spanda-assurance`, `spanda-readiness` | Verify, fleet, deploy, assurance analysis |
| **FFI & bridge** | `spanda-bridge`, `spanda-ffi` | Python/C++ subprocess bridges, live AI/IoT |
| **Tooling** | `spanda-format`, `spanda-lint`, `spanda-codegen`, `spanda-docs` | fmt, lint, codegen, docgen |
| **Packages** | `spanda-package`, `spanda-providers` | Manifest, registry fetch, provider dispatch |
| **Official packages** | `packages/registry/*` | 37 hosted `.sd` packages (ROS2, MQTT, GPS, mission assurance, …) |
| **TypeScript mirror** | `src/`, `packages/lsp` | Parser/typecheck parity, LSP, Vitest |

**Dependency rule:** Only `spanda-core` pulls the full facade graph. `spanda-cli`, `spanda-node`, `spanda-wasm`, and `spanda-dap` depend on workspace crates directly.

References: [architecture.md](../architecture.md) · [lean-core.md](../lean-core.md) · [crates/README.md](../../crates/README.md)
