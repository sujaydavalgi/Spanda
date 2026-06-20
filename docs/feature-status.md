# Feature Status

Honest snapshot of Spanda capabilities. **Stubbed** = syntax or API exists without real external integration.

## Language core

| Feature | Status | Notes |
|---------|--------|-------|
| Lexer / parser / AST | Implemented | Rust authoritative; TS mirror includes hardware/deploy |
| Type checker + units | Implemented | Physical unit algebra enforced |
| modules / imports | Implemented | Rust + TS `ModuleRegistry`; project vendor via `spanda install` |
| structs / enums / traits | Implemented | Generic struct literals `Box<Int> { ... }`; enum payloads |
| generics | Partially implemented | Module fn + struct type params with instantiation |
| trait objects (`dyn Trait`) | Implemented | Rust + TS mirror (parser/types/runtime) |
| match / Result / Option | Implemented | |
| async / await | Implemented | Cooperative single-threaded |
| spawn / select / channels | Partially implemented | Cooperative concurrency |
| test blocks | Implemented | Rust runtime + TS `runTests()` |
| `extern fn` / FFI | Partially implemented | `extern python`/`extern cpp` subprocess bridges; optional PyO3 in-process |
| Spanda IR (SIR) | Partially implemented | Enum payloads, bool `if`, float compares, `scan.nearest_distance`, const-folded conditions |
| Codegen / LLVM | Partially implemented | HAL profiles; LLVM emits float `if`, scan distance via `spanda_rt_scan_nearest` |

## Autonomous systems

| Feature | Status | Notes |
|---------|--------|-------|
| robot / sensor / actuator | Implemented | |
| agent / goal / task / skill | Implemented | Mock AI |
| ActionProposal → SafeAction | Implemented | Compile + runtime |
| safety zones / emergency stop | Implemented | |
| deterministic scheduler | Implemented | `task every Nms` |
| state machine / events | Implemented | |
| twin / replay | Implemented | Replay buffer; **`twin sync`** telemetry/replay wired |
| observe / fusion | Implemented | |
| verify { } behavioral assertions | Implemented | |
| hardware / deploy | Implemented | Rust verify CLI; TS parse + deploy validation |

## Tooling

| Feature | Status | Notes |
|---------|--------|-------|
| Native CLI (full) | Implemented | check, verify, run, fmt, lint, doc, package |
| TypeScript CLI | Implemented | Delegates to Rust when built; includes `llvm-ir` / `compile-native` |
| Formatter / linter / docgen | Implemented | Rust |
| LSP | Partially implemented | Symbols include struct/enum; hardware/deploy |
| DAP debugger | Partially implemented | Resumable sessions, setVariable, step-in/out with frame-depth step-out |
| N-API | Partially implemented | check, run, verify, sir, fmt |
| WASM | Partially implemented | check, run, verify, sir, fmt |

## Ecosystem / FFI

| Feature | Status | Notes |
|---------|--------|-------|
| python.* / cpp.* imports | Partially implemented | Subprocess bridges; optional `python-native` / `cpp-native` in-process |
| ROS2 adapter | Partially implemented | Native `ros2` CLI publish/subscribe/service (`SPANDA_ROS2_NATIVE`); rclrs stub feature |
| Transport adapters | Partially implemented | In-memory + live ROS2/MQTT via CLI or Python bridge |
| Package manager | Partially implemented | `spanda publish` upload; `spanda install` fetches remote tarballs when `SPANDA_REGISTRY_URL` set |
| LLVM / native codegen | Partially implemented | `compile-native` + `--target-triple` + `--hal-profile` conditional codegen |

See also [README.md](../README.md), [ffi-and-ecosystem.md](./ffi-and-ecosystem.md), [compiler-backend-roadmap.md](./compiler-backend-roadmap.md).
