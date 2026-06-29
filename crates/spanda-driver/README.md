# spanda-driver

**Compile and run driver** — owns the high-level pipeline API extracted from `spanda-core` in Phases 9–13.

## Responsibilities

| API | Description |
|-----|-------------|
| `compile` / `check` | Lex → parse → type-check (`spanda-lexer`, `spanda-parser`, `spanda-typecheck`) |
| `run` / `run_program` | Optional FFI defaults + `spanda-interpreter` execution |
| `lower_to_sir` | AST → SIR for LLVM and tooling |
| `replay_mission` / `playback_mission` | Mission trace replay helpers |
| `run_debug` | Debugger integration with `spanda-debug` |
| `debug_session` | Debugger machine (`DebugMachine`, step kinds) |
| `tokenize` | Lexer wrapper with `SpandaError` diagnostics |

## Related surfaces (not in this crate)

| API | Owner |
|-----|-------|
| `verify_compatibility` | `spanda-core::hardware_verify` (public facade) |
| `build_deploy_plan` | `spanda-ota` (shimmed via `spanda_core::deploy_service`) |
| Certification runtime gate | `spanda-assurance` (injected at CLI boundary) |

## Who depends on this crate

- `spanda-cli`, `spanda-node`, `spanda-wasm`, `spanda-dap` (first-party)
- `spanda-llvm` (dev-deps / tests only — no production upward edge)
- `spanda-core` (re-exports compile/run API)

## Example

```rust
use spanda_driver::{check, run, RunOptions};

check(source)?;
let result = run(source, RunOptions::default())?;
```

## Related

- [spanda-interpreter](../spanda-interpreter/README.md) — runtime execution
- [spanda-core](../spanda-core/README.md) — `verify_compatibility` facade
- [spanda-ota](../spanda-ota/README.md) — deploy plan extraction
