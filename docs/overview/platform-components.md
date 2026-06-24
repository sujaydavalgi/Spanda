# Platform components

[← Overview](./README.md) · Structure: [platform-structure.md](./platform-structure.md)

What each major platform component does.

| Component | What it does |
|-----------|--------------|
| **Spanda Language (.sd)** | Safety-first language — sensors, actuators, AI, safety rules, and deployment targets as first-class syntax |
| **Spanda Runtime** | Interpreter, scheduler, cooperative tasks, HAL bindings, certified execution after compile-time gates |
| **Spanda Verify** | Hardware compatibility (`spanda verify`), capability exposure, behavioral `verify { }`, traceability matrices |
| **Spanda Safety** | `ActionProposal` → `SafeAction` gate, safety zones, `stop_if`, emergency stop, kill-switch handlers |
| **Spanda Sim** | Physics-lite simulation (`spanda run` / `spanda sim`) and digital twins — test without hardware |
| **Spanda Replay** | Mission trace record, deterministic replay, frame playback for regression and incidents |
| **Spanda Health** | `health_check`, fleet `require` clauses, health policies |
| **Spanda Readiness** | Weighted go/no-go scoring (`spanda readiness`), fleet readiness, agent APIs |
| **Spanda Mission Assurance** | Knowledge models, state estimation, anomaly detection, prognostics, mitigation, resilience, assurance cases |
| **Spanda Fleet** | Multi-robot simulation, orchestration, mesh coordination, distributed agent relay |
| **Spanda Registry** | Hosted package index, Ed25519-signed tarballs, `spanda publish` / `spanda install` |
| **Spanda Providers** | Official packages (ROS2, MQTT, GPS, vision, fleet, OTA, cloud, mission assurance) via provider registry |

Per-component guides: [platform-overview.md](../platform-overview.md) · Diagrams: [diagrams/README.md](../diagrams/README.md)
