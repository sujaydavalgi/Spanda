# Spanda Concurrency Model

Spanda uses a deterministic, cooperative concurrency model built around tasks, agents, messages, channels, and events.

## Core model

- Deterministic scheduling is the default runtime behavior for robot tasks.
- Concurrency primitives are language-level (`task`, `spawn`, `await`, `select`, `channel`) instead of raw thread APIs.
- Agent boundaries are isolation boundaries; communication is message-based.
- Topic/service/action communication is transport-agnostic (`local`, `ros2`, `mqtt`, `dds`, `sim`).

## Tasks and scheduler

Tasks are periodic units of work.

```sd
task perception high every 50ms {
  let scan = lidar.read();
}
```

Priority levels:

- `critical`
- `high`
- `normal`
- `low`

Priority affects execution order when multiple tasks are due on the same tick. `critical` tasks execute first.

Tasks may also omit `every` and use the runtime default period:

```sd
task SafetyMonitor critical {
  emergency_stop();
}
```

Per-task budgets are supported:

```sd
task mapping every 100ms {
  budget {
    battery <= 10%;
    cpu <= 20%;
  }
}
```

## Async / await

Async module functions return `Future<T>` in the type checker and are resolved by `await` in runtime execution.

```sd
export async fn detect_objects() -> Pose {
  return pose(x: 0.0 m, y: 0.0 m, theta: 0.0 rad);
}

let result = await detect_objects();
```

## Spawn, channels, and select

`spawn` returns a `TaskHandle<T>` that can be joined explicitly. Fire-and-forget `spawn` statements still queue work for completion at the end of the current behavior.

```sd
let h = spawn planning();
let plan = join(h);

spawn telemetry();  // fire-and-forget
```

Channels provide typed message passing inferred from first send.

```sd
let ch = channel();
send(ch, 42);
select {
  recv(ch) => {
    wheels.stop();
  }
};
spawn planning();
```

- The checker validates channel usage and payload compatibility.
- Runtime enforces channel payload type consistency.

## Parallel and join

Use `parallel` for cooperative concurrent call orchestration with deterministic completion.

```sd
parallel {
  let perception = spawn detect_objects();
  let localization = spawn localize();
  let map = spawn update_map();
};

let fused = _parallel;
```

Each branch runs in an isolated environment. `let` bindings and spawned handles are joined deterministically and exposed as `_parallel` (`ParallelResults`).

```sd
parallel {
  detect_objects();
  localize();
  update_map();
};
```

Use `join` to resolve a `Future<T>` or `TaskHandle<T>` handle explicitly.

```sd
let f = detect_objects_async();
let result = join(f);

let h = spawn perception();
let scan = join(h);
```

## Agents and multi-agent communication

Agents are first-class runtime entities with:

- plan execution
- memory (`short_term` / `long_term`)
- capability-based access control

Agent-to-agent links are declared with typed channels in robot declarations.

## Distributed and simulation execution

Spanda communication abstractions are transport-backed and usable across local simulation and distributed deployments.

- simulation transport: deterministic replay/twin workflows
- ROS2 / MQTT / DDS / websocket transport kinds
- discovery support for robots, agents, devices

## Safety isolation guidance

For safety-critical loops:

- use `task ... critical`
- keep periods short
- isolate safety checks from non-critical AI workloads

This ensures the scheduler services safety tasks before lower-priority workloads on each deterministic tick.

## Runtime telemetry and CLI tracing

The runtime collects lightweight metrics on every run (`RunResult.metrics`):

- `TaskMetrics`: ticks, skipped iterations, missed deadlines, budget violations, priority
- `SchedulerMetrics`: multiplexed task count, scheduler ticks, base tick
- `ExecutionMetrics`: spawns, joins, parallel blocks
- `replay_frames`: digital twin replay buffer size

Enable verbose trace logs:

```bash
spanda run robot.sd --trace-scheduler --trace-tasks
spanda sim robot.sd --replay --trace-scheduler
```

Trace logs are prefixed with `trace-scheduler:`, `trace-task:`, or `trace-replay:` in the runtime log stream.

See `examples/concurrency.sd` for a runnable program combining task priorities, spawn handles, parallel aggregation, and channels.

## Runtime budget enforcement

Per-task `budget { }` blocks are validated at compile/verify time and enforced at runtime:

- Measured task duration is compared against `cpu` and `battery` duty limits each tick.
- Over-budget tasks log a violation and are skipped on subsequent ticks until duration estimates recover.

## Agent mailboxes

Declare typed agent channels in the robot body:

```sd
agent Vision { goal "see"; plan { send_agent("Planner", scan); } }
agent Planner { goal "plan"; plan { let msg = recv_agent(); } }
Vision -> Planner: Scan;
```

`send_agent` and `recv_agent` require active agent context (inside `agent.plan()`).

## Fleet peer messaging

Peer robots declared with `robot RoverA;` can exchange messages via namespaced topics:

```sd
subscribe RoverA.pose;
peer_send("RoverA", "pose", current_pose);
receive RoverA.pose to pose_msg;
```

## Static concurrency lint

`spanda lint` warns on suspicious channel flows:

- `channel-recv-without-send` — `recv` on a channel with no matching `send` in the same scope
- `channel-send-without-recv` — `send` with no matching `recv` in the same scope

```bash
spanda lint robot.sd
spanda lint --json robot.sd
```

## Fleet CLI

`spanda fleet run` prints deploy targets and peer-robot wiring from a program, then runs the in-process simulation (same engine as `spanda run` / `spanda sim`):

```bash
spanda fleet run examples/communication/multi_robot_fleet.sd
spanda fleet run --trace-scheduler --trace-tasks fleet.sd
```

Output includes:

- `deploy <robot> -> <target>` lines from `deploy` declarations
- `peer robot <name> knows <peer>` lines from `robot Peer;` declarations
- final pose, battery, and replay trace when `--trace-*` flags are set

This is a single-process fleet simulation today; true multi-process orchestration (one subprocess per deploy target) is planned separately.

For fleet **recovery** and **mission continuity** (takeover, delegation, succession) across deployed agents, see [fleet-distributed.md](./fleet-distributed.md), [self-healing.md](./self-healing.md), and [mission-continuity.md](./mission-continuity.md). CLI: `spanda demo self-healing`, `spanda demo continuity`.

## TypeScript interpreter parity

The npm `spanda` package mirrors the Rust concurrency surface in the TypeScript interpreter:

- `task` priorities (`critical` / `high` / `normal` / `low`) and optional `every`
- `spawn`, `join`, `parallel`, `_parallel`, `channel`, `send`, `recv`, `select`, `await`
- per-task `budget { }` enforcement (skip + log on violation)
- `send_agent` / `recv_agent` and agent channel declarations (`Vision -> Planner`)
- `peer_send` and dotted `receive RoverA.pose` for fleet peer messaging
- `publishPeer` in the comm transport layer

Use `compile(source)` + `run(program, { backend })` from `src/compile.ts`, or the CLI with the default TS backend. Rust-native execution (`rust-cli` / `rust-native` backends) uses the same semantics via `spanda-core`.

Tests: `tests/concurrency.test.ts`, `tests/concurrency-extended.test.ts` (TS); `crates/spanda-core/tests/concurrency_extended.rs` (Rust).
