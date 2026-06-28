# Spatial Computing & Human-Robot Collaboration — Official Solution Blueprint

Demonstrates safe collaboration between **humans**, **wearables**, **AR/VR/XR devices**, **robots**, **drones**, and **fleets** — built on existing platform capabilities. No HRI-specific core language extensions.

**Profile:** `human_collaboration` · **Status:** Planned (roadmap H1–H4)

---

## Quick start (when H1 ships)

```bash
cd examples/solutions/spatial-computing
spanda install
spanda check warehouse-ar/pick_mission.sd
spanda verify warehouse-ar/pick_mission.sd --capabilities --traceability --config spanda.toml
spanda readiness warehouse-ar/pick_mission.sd --profile human_collaboration --config spanda.toml --json
spanda control-center serve --config spanda.toml --program warehouse-ar/pick_mission.sd
```

Smoke (planned): `./scripts/spatial_computing_smoke.sh`

---

## Blueprint layout

```
spatial-computing/
├── README.md
├── spanda.toml
├── spanda.devices.toml      # Humans, wearables, AR/VR, robots, drones
├── spanda.readiness.toml    # Operator, team, mission readiness
├── spanda.security.toml     # Privacy and AR session security
├── warehouse-ar/            # AR warehouse picking
├── remote-maintenance/      # Remote expert guided repair
├── vr-training/             # VR operator training
├── search-and-rescue-ar/    # SAR collaborative mission
├── wearable-health/         # Optional health monitoring
└── operator-approval/       # Supervisor approval workflow
```

---

## Example workflows

| Directory | Workflow |
|-----------|----------|
| [`warehouse-ar/`](./warehouse-ar/) | Operator + AR overlay + AMR pick coordination |
| [`remote-maintenance/`](./remote-maintenance/) | Field tech + HoloLens + remote expert |
| [`vr-training/`](./vr-training/) | Trainee VR session + mission replay |
| [`search-and-rescue-ar/`](./search-and-rescue-ar/) | Multi-human SAR + drone overlay |
| [`wearable-health/`](./wearable-health/) | Wearable vitals with privacy gates |
| [`operator-approval/`](./operator-approval/) | Supervisor mission approval |

---

## Documentation

| Guide | Topic |
|-------|-------|
| [docs/solutions/spatial-computing.md](../../../docs/solutions/spatial-computing.md) | Architecture and applications |
| [docs/human-interaction-spatial-computing-roadmap.md](../../../docs/human-interaction-spatial-computing-roadmap.md) | Phased roadmap |
| [docs/human-interaction.md](../../../docs/human-interaction.md) | Human entity model |
| [docs/hri-packages.md](../../../docs/hri-packages.md) | Optional packages |

---

## Control Center

```bash
spanda control-center serve --config spanda.toml --program warehouse-ar/pick_mission.sd
```

Planned tabs: Human Dashboard, Operator Readiness, Wearable Inventory, AR Session Viewer, Approval Queue.
