# Platform components

[← Overview](./README.md) · Structure: [platform-structure.md](./platform-structure.md)

> **Per-component guides:** [platform-overview.md](../platform-overview.md) (Verify, Safety, Sim, Fleet, …)

Quick-reference table — one line each; see the canonical guide for CLI links and examples.

| Component | What it does |
|-----------|--------------|
| **Spanda Language (.sd)** | Safety-first language — sensors, actuators, AI, safety, deploy as syntax |
| **Spanda Runtime** | Interpreter, scheduler, HAL, cooperative tasks |
| **Spanda Verify** | `spanda verify`, capabilities, `verify { }`, traceability |
| **Spanda Safety** | `ActionProposal` → `SafeAction`, zones, `stop_if`, kill switch |
| **Spanda Sim** | `spanda run` / `spanda sim`, digital twins |
| **Spanda Replay** | Mission traces, deterministic replay, playback |
| **Spanda Health** | `health_check`, fleet `require`, policies |
| **Spanda Readiness** | `spanda readiness`, agent APIs |
| **Spanda Mission Assurance** | Knowledge, state, anomaly, prognostics, resilience, assurance cases |
| **Spanda Fleet** | Multi-robot sim, orchestration, mesh, agents |
| **Spanda Registry** | Hosted index, signed tarballs, `spanda install` |
| **Spanda Providers** | Official packages via provider registry |

Diagrams: [diagrams/README.md](../diagrams/README.md)
