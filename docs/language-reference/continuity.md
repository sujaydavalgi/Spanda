# Continuity

Mission continuity policies, takeover modes, and succession planning for fleet and single-robot programs.

## Declarations

```spanda
continuity_policy FleetContinuity {
    on robot.failed {
        resume from checkpoint;
        reassign mission;
    }
}
```

Pair with `mission_plan` for checkpoint resume and `fleet` for multi-robot handoff.

## CLI

```bash
spanda continuity program.sd --failed RoverA --progress 72
spanda takeover program.sd --failed RoverA --successor RoverB
spanda delegate program.sd --failed RoverA --to RoverB
spanda succession program.sd --scope fleet
```

## Diagnostics

`spanda check --readiness-json` emits `continuity:policy`, `continuity:fleet`, `continuity:approval`, `continuity:handoff`, and `continuity:mission` categories.

## Guides

- [continuity-policies.md](../continuity-policies.md) — syntax and actions
- [mission-continuity.md](../mission-continuity.md) — architecture and runtime
- [recovery.md](./recovery.md) — recovery pairing and fleet mesh
- CLI: `spanda man continuity`
