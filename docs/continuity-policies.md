# Continuity Policies

Continuity policies declare takeover, delegation, and succession behavior when robots or fleet members fail mid-mission.

Related: [mission-continuity.md](./mission-continuity.md) · [recovery-policies.md](./recovery-policies.md) · [fleet-distributed.md](./fleet-distributed.md)

---

## Syntax

```spanda
continuity_policy WarehouseContinuity {
    on robot.failed {
        resume from checkpoint;
        reassign mission;
    }
    on battery.critical {
        hot takeover;
    }
    on fleet.failed {
        reassign mission;
    }
}
```

Declared actions influence takeover mode inference (`resume`, `hot takeover`, `restart`, `human takeover`, etc.).

---

## Triggers

Common `on` conditions:

| Trigger | Meaning |
|---------|---------|
| `robot.failed` | Primary robot offline or faulted |
| `robot.degraded` | Robot degraded but partially operational |
| `battery.critical` | Battery below safe threshold |
| `fleet.failed` / `swarm.failed` | Fleet or swarm member lost |
| `communication.interrupted` | Link loss between coordinator and agent |
| `hardware.capability_lost` | Required sensor or actuator unavailable |

Pair fleet triggers with a `fleet` declaration. Diagnostics emit `continuity:fleet` when fleet/swarm triggers appear without a fleet block.

---

## Actions

| Action | Typical takeover mode |
|--------|----------------------|
| `resume from checkpoint` | Resume at last checkpoint |
| `reassign mission` | Hand off to ranked successor |
| `hot takeover` | Immediate replacement |
| `cold takeover` | Initialize successor after failure |
| `shadow takeover` | Pre-synchronized backup takes over |
| `human takeover` | Operator assumes control |
| `restart` / `partial restart` | Full or stage-level restart |

Hot, cold, and human takeover are high-risk. Declare an `Approval` topic or operator path; diagnostics emit `continuity:approval` when missing.

---

## Mission plans and checkpoints

Resume and checkpoint actions work best with a `mission_plan`:

```spanda
mission_plan WarehouseInventoryScan {
    step navigate_aisles;
    step scan_shelves;
    step report_inventory;
}

continuity_policy WarehouseContinuity {
    on robot.failed {
        resume from checkpoint;
        reassign mission;
    }
}
```

Without a `mission_plan`, the continuity engine synthesizes progress from context; diagnostics warn with `continuity:mission` when resume/checkpoint actions are declared without a plan.

---

## Fleet programs

Declare a fleet and continuity policy together:

```spanda
fleet PatrolFleet {
    RoverAlpha;
    RoverBeta;
}

continuity_policy PatrolContinuity {
    on robot.failed {
        resume from checkpoint;
        reassign mission;
    }
}
```

Diagnostics emit `continuity:policy` when a multi-member fleet lacks `continuity_policy`. Pair `recovery_policy` reassign actions with `continuity_policy` for takeover mode — `continuity:handoff` hints when recovery reassigns without continuity.

---

## Runtime dispatch

When `SPANDA_FLEET_MESH_URL` is set, recovery handoff and continuity CLI commands relay through the mesh:

- Coordinator: `POST /v1/fleet/continuity`
- Agent: `POST /v1/continuity/execute`, `POST /v1/continuity/ack`
- Peer topic: `fleet_takeover`

See [fleet-distributed.md](./fleet-distributed.md) and [mission-continuity.md](./mission-continuity.md).

---

## CLI

```bash
spanda continuity examples/showcase/continuity/warehouse.sd \
  --failed ScannerAlpha --progress 72 --trigger robot_failed
spanda takeover examples/showcase/takeover/patrol.sd --failed RoverA
spanda delegate examples/showcase/delegation/survey.sd --failed SurveyBot --to RelayBot
spanda succession examples/showcase/fleet_succession/delivery.sd --scope fleet
spanda check examples/showcase/continuity/warehouse.sd --readiness-json
spanda demo continuity
```

---

## Package

Official scaffold: `spanda-mission-continuity` (`assurance.continuity`).

---

## Examples

| Example | Focus |
|---------|--------|
| [`examples/showcase/continuity/warehouse.sd`](../examples/showcase/continuity/warehouse.sd) | Checkpoint resume |
| [`examples/showcase/takeover/patrol.sd`](../examples/showcase/takeover/patrol.sd) | Hot takeover |
| [`examples/showcase/delegation/survey.sd`](../examples/showcase/delegation/survey.sd) | Delegation |
| [`examples/showcase/fleet_succession/delivery.sd`](../examples/showcase/fleet_succession/delivery.sd) | Fleet succession |
