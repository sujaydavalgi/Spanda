# Operator Approval Workflow

Supervisor must approve collaborative mission start — integrates with Control Center approval queue and existing recovery approval paths.

## Run

```bash
spanda check collaborative_mission.sd
spanda readiness collaborative_mission.sd --profile human_collaboration --config ../spanda.toml --json
spanda control-center approvals list
```

## Demonstrates

- `approve_mission` supervisor capability requirement
- Mission `requires approval` gate (Stable — mission continuity)
- `require_supervisor_for_mission` in readiness profile
- Control Center **Approval Queue** (extends E1 operator workflows)

## Roles

| Human | Role | Capability |
|-------|------|------------|
| `operator-001` | operator | `operate_robot` |
| `supervisor-001` | supervisor | `approve_mission` |

## Docs

[hri.md](../../../docs/hri.md) · [operator-capabilities.md](../../../docs/operator-capabilities.md)
