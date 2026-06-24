# Spanda platform structure

[← Overview](./README.md)

High-level shape of the platform. The language is the expressive core; verification, safety, simulation, and operations wrap around it.

```
Spanda Platform
│
├── Spanda Language (.sd)
├── Spanda Runtime
├── Spanda Verify
├── Spanda Safety
├── Spanda Sim
├── Spanda Replay
├── Spanda Health
├── Spanda Readiness
├── Spanda Mission Assurance
├── Spanda Fleet
├── Spanda Registry
└── Spanda Providers
```

## Platform vs language

| | **Spanda Platform** | **Spanda Language** |
|---|---------------------|---------------------|
| **What it is** | End-to-end toolchain for designing, verifying, simulating, deploying, and operating autonomous systems | The `.sd` programming language — one core component |
| **Scope** | Runtime, verification, safety, simulation, replay, health, fleet, packages, providers | Syntax, types, robot primitives, safety types, units, compile-time checks |
| **How you use it** | `spanda check`, `verify`, `sim`, `replay`, `fleet`, `install`, `demo`, … | Write `.sd`; declare `robot`, `sensor`, `actuator`, `safety`, `deploy` |

Deep dive: [platform-overview.md](../platform-overview.md) · Components: [platform-components.md](./platform-components.md)
