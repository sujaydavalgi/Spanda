# Spanda platform structure

[← Overview](./README.md)

> **Canonical guide:** [platform-overview.md](../platform-overview.md) (per-component detail, workflow, platform vs language)

> **Product roadmap:** [ROADMAP.md](../../ROADMAP.md) — 8 Platform Pillars, 14 Solution Blueprints, product family

## Product family

```text
Spanda Language → Runtime → Verify → Readiness → Assurance → Diagnosis
        → Recovery → Trust → Control Center → Registry → SDKs
```

## Eight platform pillars

| # | Pillar | Hub |
|---|--------|-----|
| 1 | Spanda Language | [pillars/language/](../pillars/language/README.md) |
| 2 | Compiler & Runtime | [pillars/compiler-runtime/](../pillars/compiler-runtime/README.md) |
| 3 | Verification Platform | [pillars/verification/](../pillars/verification/README.md) |
| 4 | Device & Fleet Platform | [pillars/device-fleet/](../pillars/device-fleet/README.md) |
| 5 | Security Platform | [pillars/security/](../pillars/security/README.md) |
| 6 | Operations Platform | [pillars/operations/](../pillars/operations/README.md) |
| 7 | Developer Platform | [pillars/developer/](../pillars/developer/README.md) |
| 8 | Packages & Ecosystem | [pillars/packages/](../pillars/packages/README.md) |

**Solution blueprints:** [solutions/README.md](../solutions/README.md) — industry compositions (not core features)

## Legacy component tree

High-level platform tree (maps to pillars above):

```
Spanda Platform
│
├── Spanda Language (.sd)          → Pillar 1
├── Spanda Runtime                 → Pillar 2
├── Spanda Verify                  → Pillar 3
├── Spanda Safety                  → Pillar 3
├── Spanda Sim / Replay            → Pillar 3
├── Spanda Health / Readiness      → Pillar 3 + 4
├── Spanda Mission Assurance       → Pillar 3
├── Spanda Fleet                   → Pillar 4
├── Spanda Control Center          → Pillar 6
├── Spanda Registry / Providers    → Pillar 8
└── Official Solution Blueprints   → Industry layer (separate)
```

Component summary table: [platform-components.md](./platform-components.md)
