# Examples & standard library

[← Overview](./README.md)

## Examples library

Runnable `.sd` programs for learning, demos, and CI.

**Master index:** [examples/README.md](../../examples/README.md)

| Tier | Directory | Highlights |
|------|-----------|------------|
| Basics | [examples/basics/](../../examples/basics/README.md) | Robot syntax → observe & fusion |
| Features | [examples/features/](../../examples/features/README.md) | One file per capability |
| Integration | `examples/integration/` | Triggers, concurrency, verify |
| End-to-end | [examples/end_to_end/](../../examples/end_to_end/README.md) | Patrol, fleet, replay |
| Showcase | [examples/showcase/](../../examples/showcase/README.md) | `spanda demo` programs |
| Assurance | `examples/assurance/`, `examples/anomaly/`, … | Mission assurance domains |

Flagship demos: [demos-and-examples.md](./demos-and-examples.md) · Code samples: [code-samples.md](./code-samples.md)

## Standard library (`std.*`)

Modular namespaces — import only what you need (`import std.robotics;`).

| Namespace | Purpose |
|-----------|---------|
| `std.robotics` | Robot graph, `ActionProposal`, `SafeAction` |
| `std.sensors` / `std.actuators` | Sensor payloads and actuator types |
| `std.safety` | Constraints, hazards, emergency stop |
| `std.navigation` / `std.fusion` / `std.slam` | Navigation, fusion, SLAM types |
| `std.communication` | Topics, services, events |
| `std.ai` | LLM, vision, reasoning types |

Full reference: [standard-library.md](../standard-library.md) · [spanda-reference.md](../spanda-reference.md) · Samples: [examples/std/](../../examples/std/)
