# Architecture overview

[← Overview](./README.md) · Layers: [layers.md](./layers.md)

Spanda uses a **lean-core, package-first** workspace. `spanda-driver` orchestrates compile and run; `spanda-interpreter` is the runtime composition root. `spanda-core` is the stable public facade for external embedders.

## Compiler pipeline

```
.sd source + spanda.toml
        ↓
   spanda-driver (orchestration)
        ↓
   lexer → parser → AST → type checker (+ units, safety, capabilities)
        ↓
   hardware verifier · behavioral verify · capability / health gates
        ↓
   spanda-certify runtime gate → interpreter + simulator
        ↓
   provider registry ← official packages (ROS2, MQTT, GPS, …)
        ↓
   SIR → LLVM → native binary (experimental)
```

## Crate layers (summary)

| Layer | Crates | Responsibility |
|-------|--------|----------------|
| **Public facade** | `spanda-core` | Stable `spanda_core::` re-exports |
| **Apps** | `spanda-cli`, `spanda-node`, `spanda-wasm`, `spanda-dap`, `spanda-llvm` | CLI, bindings, debugger, codegen |
| **Pipeline** | `spanda-driver`, `spanda-lexer`, `spanda-parser`, `spanda-ast`, `spanda-typecheck`, `spanda-sir` | Lex → parse → check → SIR |
| **Runtime** | `spanda-interpreter`, `spanda-runtime`, `spanda-safety`, `spanda-hal`, `spanda-concurrency` | Execution, scheduling, safety, HAL |
| **Transport** | `spanda-transport-routing`, `spanda-transport-*` | Adapters, live bridges, `RoutingCommBus` |
| **Domain** | `spanda-hardware`, `spanda-fleet`, `spanda-ota`, `spanda-certify`, `spanda-assurance` | Verify, fleet, rollout, assurance |
| **Packages** | `spanda-package`, `spanda-providers` | `spanda.toml`, registry, provider bootstrap |
| **Mirror & UX** | `src/`, `packages/lsp`, `packages/web`, `editor/vscode` | TypeScript tests, LSP, web playground |

Full detail: [layers.md](./layers.md) · [architecture.md](../architecture.md) · [lean-core.md](../lean-core.md) · [crates/README.md](../../crates/README.md)
