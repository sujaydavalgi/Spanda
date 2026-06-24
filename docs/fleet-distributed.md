# Distributed fleet operations

Spanda supports **multi-robot programs** locally (`spanda fleet run`) and **remote orchestration** when fleet agents are registered on the network.

## Local multi-robot

```bash
spanda fleet run examples/showcase/fleet_management/main.sd
```

The interpreter sets up and executes each robot in isolation so coordinator-only robots (no actuators) do not shadow member bindings.

## Remote orchestration

### 1. Start mesh and agents

**Coordinator host:**

```bash
spanda fleet mesh start --bind 127.0.0.1:9700 --token "$FLEET_TOKEN"
```

**Per-robot hosts:**

```bash
spanda fleet agent start --bind 127.0.0.1:9701 --robot RoverA --token "$FLEET_TOKEN"
spanda fleet agent register RoverA http://127.0.0.1:9701 --token "$FLEET_TOKEN"
```

List registered agents:

```bash
spanda fleet agent list
spanda fleet agent list --json
```

### 2. Orchestrate with relay

```bash
spanda fleet orchestrate --remote --json examples/robotics/fleet_field_trial.sd
```

`--remote` relays peer deliveries to registered fleet agents. JSON output includes `remote_relayed` and `remote_failed` counts per fleet block.

### 3. Mesh URL (optional)

When a mesh coordinator is already running elsewhere:

```bash
spanda fleet orchestrate --remote \
  --mesh-url http://coordinator:9700 --mesh-token "$FLEET_TOKEN" \
  fleet_program.sd
```

## OTA deploy (remote rollout)

Remote OTA uses the same agent registry:

```bash
spanda deploy plan --version 1.2.0 program.sd
spanda deploy rollout --remote --require-certify program.sd
spanda deploy rollback --remote program.sd
```

See [phase-18-security-hardening.md](./phase-18-security-hardening.md) for token requirements on non-loopback binds.

## Fleet recovery (mesh + agents)

When programs declare `recovery_policy` and robots are registered on the mesh, runtime recovery actions can relay through the coordinator:

```bash
export SPANDA_FLEET_MESH_URL=http://coordinator:9700
export SPANDA_FLEET_MESH_TOKEN="$FLEET_TOKEN"
spanda run coordinator.sd   # publishes /fleet/recovery and POSTs to mesh when env is set
```

### Mesh coordinator

| Endpoint | Description |
|----------|-------------|
| `POST /v1/fleet/recovery` | Relay recovery action to registered fleet agents (`fleet_recovery` peer topic) |

Request body (JSON):

```json
{
  "action": "enter degraded_mode",
  "fleet_name": "PatrolFleet",
  "from_robot": "RoverAlpha",
  "members": ["RoverAlpha", "RoverBeta"]
}
```

### Fleet agent (deployed program)

Upload program source, then trigger or receive recovery:

| Endpoint | Description |
|----------|-------------|
| `POST /v1/program` | Upload `.sd` source (`{"program": "..."}`) |
| `POST /v1/recovery/execute` | Direct recovery trigger (`{"action": "pause mission"}`) |
| `POST /v1/recovery/ack` | Clear active recovery state |
| `GET /v1/status` | Agent state including recovery fields |

Recovery fields on `GET /v1/status`:

- `recovery_engine` — `interpreter` (live runtime dispatch), `assurance` (plan/validate fallback), or `fallback`
- `recovery_validation` — `PASS`, `PARTIAL`, or `FAIL`
- `recovery_active`, `recovery_actions_applied`, `recovery_mode`, `mission_paused`
- `last_recovery_runtime_logs`, `last_recovery_evidence`

Deployed agents prefer **interpreter recovery** (`execute_recovery_on_program`) for mode transitions, speed caps, mission pause, and connectivity restart. Set `SPANDA_OPERATOR_APPROVAL=1` when testing high-risk actions without Approval topics.

See [self-healing.md](./self-healing.md).

## Golden path

```bash
./examples/robotics/golden_path_deploy.sh
```

CI job: `robotics-golden-path`.

## Related

- [fleet-health.md](./fleet-health.md)
- [concurrency.md](./concurrency.md)
- [tier-3-experimental.md](./tier-3-experimental.md)
