# Remote Maintenance

Field technician (`tech-001`) + HoloLens + live wrist camera + remote expert annotations.

## Workflow

```
Field Technician → AR Glasses → Live Robot Camera → Remote Expert → Annotations → Guided Repair
```

## Run

```bash
spanda check repair.sd
spanda sim repair.sd --record
spanda replay repair.trace --deterministic
spanda diagnose repair.sd repair.trace
```

## Demonstrates

- `maintenance_technician` operator capability
- Recovery approval gate (`requires approval for resume`)
- Replay + diagnosis for expert session audit
- Integration with `spanda-mission-continuity`

## Docs

[remote-expert.md](../../../docs/remote-expert.md)
