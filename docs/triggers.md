# Trigger-Based Execution

Spanda autonomous systems are **trigger-driven**. Sensors publish data, timers elapse, safety conditions change, state machines transition, and AI agents complete goals — each of these is a **trigger** that can invoke reactive handler code.

The unified trigger model extends existing Spanda primitives (`event`/`on`/`emit`, `task every`, topics, state machines, safety) rather than replacing them.

## Philosophy

Autonomous robots do not run a single linear program. They:

- React to sensor events and messages
- Run periodic control loops on deterministic schedules
- Respond immediately to safety violations
- Transition through operational states
- Coordinate AI planning with hardware constraints

Triggers make this reactive structure a **first-class language concept** with shared registration, priority ordering, metrics, and scheduler integration.

## Trigger Categories

| Category | Syntax | Status |
|----------|--------|--------|
| Event | `on ObstacleDetected { ... }` | Implemented |
| Message | `on lidar_scan { ... }` | Implemented (dispatches on publish) |
| Timer | `every 100ms { ... }` | Implemented |
| Condition (edge) | `when expr { ... }` | Implemented (fires once when becoming true) |
| Condition (level) | `while expr { ... }` | Implemented (fires each tick while true) |
| State | `on state Entered(Navigating) { ... }` | Implemented |
| Safety | `on safety EmergencyStop priority critical { ... }` | Implemented |
| Hardware | `on hardware CameraFailure { ... }` | Implemented (monitor + fault injection) |
| AI | `on ai GoalCompleted { ... }` | Implemented (plan complete, confidence) |
| Verification | `on verification Failed / Warning { ... }` | Implemented |
| Digital Twin | `on twin DivergenceDetected / FaultInjected { ... }` | Implemented |
| Transport inbound | subscribed topics on ROS2/MQTT/DDS/WebSocket | Implemented (polled each tick) |

## Examples

### Event-driven

```spanda
event ObstacleDetected;

on ObstacleDetected {
  wheels.stop();
}

behavior run() {
  emit ObstacleDetected;
}
```

### Message-driven

```spanda
topic lidar_scan: Scan subscribe on "/scan";

on lidar_scan {
  update_world_model();
}
```

Handlers fire when the topic is published (in-process via `publish`).

### Timer-driven

```spanda
every 100ms {
  publish_pose();
}

every 1s {
  publish_status();
}
```

Timer triggers integrate with the deterministic scheduler (same base tick as multiplexed tasks).

### Condition-driven

```spanda
when lidar.nearest_distance < 1.0 m {
  stop();
}
```

The compiler validates that the condition expression is boolean. Conditions are **edge-triggered** (fire once when becoming true).

For continuous monitoring while a condition holds, use `while`:

```spanda
while battery.level < 20% {
  return_home();
}
```

Level triggers fire on each scheduler tick while the condition is true (subject to trigger storm limits).

### Verification-driven

```spanda
verify {
  true;              // hard rule — failure stops execution
  warning false;     // soft rule — dispatches on verification Warning
}

on verification Failed {
  block_deployment();
}

on verification Warning {
  request_review();
}
```

### Hardware-driven

Hardware triggers dispatch when `simulate_compatibility` faults are injected, sensor reads fail repeatedly, or comm faults match sensor types:

```spanda
simulate_compatibility { fault LidarFailure; }

on hardware LidarFailure {
  enter_safe_mode();
}
```

### AI-driven

```spanda
on ai GoalCompleted {
  notify_operator();
}

on ai ConfidenceLow {
  request_human_review();
}
```

`GoalCompleted` fires after a successful `agent.plan()`. `ConfidenceLow` fires when `planner.reason()` returns low confidence (derived from sensor context).

### Twin-driven

```spanda
on twin DivergenceDetected {
  create_alert();
}

on twin FaultInjected {
  run_recovery();
}
```

Twin divergence is detected when mirrored shadow state diverges from live robot state beyond a threshold. Fault injection via `simulate_compatibility` or comm faults dispatches `FaultInjected`.

### State-driven

```spanda
state_machine Nav {
  state Idle;
  state Navigating;
  transition Idle -> Navigating;
}

on state Entered(Navigating) {
  start_navigation();
}

on state Exited(Navigating) {
  cleanup();
}
```

### Safety-driven

```spanda
on safety EmergencyStop priority critical {
  stop_all_actuators();
}
```

Safety triggers use the same priority levels as tasks: `critical`, `high`, `normal`, `low`.

## Priority

```spanda
on safety EmergencyStop priority critical {
  stop_all_actuators();
}
```

Critical triggers run before lower-priority handlers in the same dispatch cycle.

## Agent Integration

```spanda
agent Vision {
  on camera_frame {
    process_frame();
  }
}
```

Agent-scoped triggers register against declared topics or events.

## Architecture

All triggers register in a unified `TriggerRegistry` at robot setup:

- **Events** — `emit` dispatches synchronously (existing behavior, now via registry)
- **Messages** — `publish` invokes matching topic handlers
- **Timers / conditions** — evaluated each scheduler tick
- **State** — `enter` invokes Entered/Exited handlers
- **System categories** — runtime emits internal triggers (safety, verification, etc.)

### Reused Infrastructure

- `EventBus` — backward compatible; events also register in `TriggerRegistry`
- `CommBus` — topic publish path invokes message triggers
- Task scheduler — timer and condition triggers share the multiplexed tick loop
- `SafetyMonitor` — emergency stop dispatches `on safety EmergencyStop`
- Telemetry — `TriggerMetrics` records executions, failures, missed deadlines

### Hardware-Aware Execution

A per-tick dispatch limit (`MAX_TRIGGERS_PER_TICK`, default 64) prevents trigger storms from overwhelming devices.

## Observability

```bash
spanda run robot.sd --trace-triggers
spanda sim robot.sd --trace-events
```

`RunResult.metrics.triggers` contains per-handler execution counts, failures, and timing.

## Simulation and Replay

Trigger execution uses the same interpreter path in `spanda run`, `spanda sim`, and replay modes. Timer triggers advance with simulation time.

## See Also

- [concurrency.md](./concurrency.md) — task scheduler and priorities
- [feature-status.md](./feature-status.md) — transport and live hardware status
- `examples/triggers_demo.sd` — comprehensive trigger example
