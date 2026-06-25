# Mission Resource Estimation

**Status:** Experimental · **Phase:** Simulate, Deploy · **Priority:** P2.3

Estimate mission resource cost before execution.

## CLI

```bash
spanda estimate examples/showcase/hardware_compatibility.sd
spanda estimate mission.sd --target RoverV1 --json
```

## Estimates

| Resource | Source |
|----------|--------|
| Battery | Mission duration × hardware power draw vs battery capacity |
| CPU | Task periods and behavior loop intervals |
| Memory | Hardware profile baseline + sensors/agents |
| Storage | Trace buffer heuristic for control loops |
| Network | Topic count bandwidth heuristic |
| Mission duration | `mission { duration: … }` declaration |

## Output

`MissionEstimateReport` — per-resource estimates with confidence, assumptions, and overall budget status.

## Integration

Composes `spanda-hardware` profile registry with AST mission and robot structure.

Showcase: `examples/showcase/hardware_compatibility.sd` · smoke: `scripts/estimate_smoke.sh`

See [hardware-compatibility.md](./hardware-compatibility.md) · [platform-maturity-roadmap.md](./platform-maturity-roadmap.md).
