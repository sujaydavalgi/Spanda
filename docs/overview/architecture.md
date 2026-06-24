# Architecture overview

[← Overview](./README.md) · Crate layers: [layers.md](./layers.md)

> **Canonical deep dive:** [architecture.md](../architecture.md) · [lean-core.md](../lean-core.md)

Spanda uses a **lean-core, package-first** workspace. `spanda-driver` orchestrates compile and run; `spanda-interpreter` is the runtime composition root.

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

Workspace crate layers live in [layers.md](./layers.md) (not duplicated here). Crate index: [crates/README.md](../../crates/README.md).
