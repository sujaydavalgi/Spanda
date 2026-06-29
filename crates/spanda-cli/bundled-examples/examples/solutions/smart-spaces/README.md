# Smart Spaces & Ambient Intelligence — Official Solution Blueprint

Safety-first verification, orchestration, readiness, assurance, and trust for intelligent environments — from smart homes to smart cities.

**Profile:** `smart_space` · **Status:** Experimental (scaffold) · **Doc:** [docs/solutions/smart-spaces.md](../../docs/solutions/smart-spaces.md)

> Spanda is **not** a home automation platform. It composes above Matter hubs, BMS systems, and ecosystems like Home Assistant — adding verified missions, continuity, and evidence without replacing device pairing UX.

---

## Quick start

```bash
cd examples/solutions/smart-spaces
spanda check smart-home/night_mode.sd
spanda verify smart-building/floor_readiness.sd --target SmartSpaceGatewayV1
spanda readiness smart-home/night_mode.sd --profile smart_space --config spanda.toml --json
```

From repo root:

```bash
./scripts/smart_spaces_smoke.sh
```

---

## Blueprint layout

```
smart-spaces/
├── README.md
├── spanda.toml
├── spanda.devices.toml
├── spanda.readiness.toml
├── spanda.security.toml
├── spanda.providers.toml
├── smart-home/
├── smart-office/
├── smart-building/
├── hospital-at-home/
├── energy-management/
└── emergency-response/
```

---

## Example applications

| Directory | Deployment | Mission |
|-----------|------------|---------|
| [smart-home/](smart-home/) | Residential | Night mode, leak hooks |
| [smart-office/](smart-office/) | Office floor | Occupancy climate, cleaning |
| [smart-building/](smart-building/) | Commercial tower | Floor readiness, lockdown |
| [hospital-at-home/](hospital-at-home/) | Clinical home | Patient monitoring |
| [energy-management/](energy-management/) | Any scale | Demand response |
| [emergency-response/](emergency-response/) | Any scale | Fire response, evacuation |

---

## Platform pillars used

| Pillar | Capabilities |
|--------|--------------|
| Device & Fleet | Unified entity model, device tree, health policies |
| Verification | Readiness, assurance, diagnose, recovery |
| Operations | Mission continuity, Control Center, telemetry |
| Security | Trust, access audit, tamper |
| Packages | Matter, BACnet, energy, building, HA bridge |

---

## Control Center

```bash
spanda control-center serve \
  --config examples/solutions/smart-spaces/spanda.toml \
  --program examples/solutions/smart-spaces/smart-building/floor_readiness.sd
```

---

## Documentation

- [docs/solutions/smart-spaces.md](../../docs/solutions/smart-spaces.md) — Architecture
- [docs/building-automation.md](../../docs/building-automation.md)
- [docs/ambient-intelligence.md](../../docs/ambient-intelligence.md)
- [docs/energy-management.md](../../docs/energy-management.md)
- [docs/smart-space-readiness.md](../../docs/smart-space-readiness.md)
- [docs/smart-space-security.md](../../docs/smart-space-security.md)
- [docs/smart-space-device-tree.md](../../docs/smart-space-device-tree.md)
- [docs/smart-space-packages.md](../../docs/smart-space-packages.md)

---

## Related blueprints

- [Spatial Computing](../spatial-computing/) — Human, wearable, AR
- [Environmental Monitoring](../environmental-monitoring/) — Outdoor sensor mesh
- [Connected Healthcare](../spatial-computing/wearable-health/) — Clinical wearables
