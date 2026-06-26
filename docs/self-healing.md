# Self-Healing Framework

Spanda self-healing follows a **safety-first** recovery workflow:

```
Detect → Diagnose → Plan Recovery → Validate Safety → Execute Recovery → Verify Outcome → Audit Evidence
```

Self-healing **never bypasses**:

- Safety validation
- Hardware verification
- Capability verification
- Kill switch
- Human approval requirements

## Recovery levels

| Level | Name | Behavior |
|-------|------|----------|
| 0 | Detection Only | Report failures only |
| 1 | Recommend Recovery | Suggest actions to operator |
| 2 | Automatic Low-Risk | Execute low-risk corrections |
| 3 | Automatic With Validation | Execute after all validation gates pass |
| 4 | Human Approval Required | High-risk actions need operator approval |

## CLI

```bash
spanda heal rover.sd
spanda heal mission.trace
spanda recover rover.sd --failure gps
spanda recovery-report rover.sd
spanda recovery knowledge rover.sd
spanda sim rover.sd --inject-failure gps
spanda analyze-failure rover.sd --with-recovery
```

## Example output

```
Issue:
gps.failed

Diagnosis:
Satellite lock lost

Recovery:
switch_to visual_odometry

Risk:
Low

Safety Validation:
PASS

Outcome:
Success
```

## Runtime execution

Validated recovery actions dispatch at runtime:

- `enter degraded_mode` / `safe_mode` / `recovery_mode` — mode transitions
- `reduce_speed` — lowers safety monitor speed cap
- `restart connectivity` — reconnects active link
- `pause mission` — pauses mission controller
- Fleet actions — `reassign mission`, `redistribute tasks`, `promote backup coordinator` (also relay **continuity takeover** via `POST /v1/fleet/continuity` when mesh URL is set)

### Auto-trigger during run/sim

When a program declares `recovery_policy`, the interpreter automatically evaluates recovery during:

- Hardware fault events (`run_hardware_triggers`)
- Critical/degraded health transitions (`poll_runtime_health_changes`)
- Runtime fault polling (`poll_runtime_fault_changes`)

Look for log lines `recovery: auto-triggered for '…'` during `spanda run` / `spanda sim --inject-health-faults`.

### Operator approval

High-risk actions require operator approval via:

- `SPANDA_OPERATOR_APPROVAL=1` (simulation/testing)
- `SPANDA_GRANT_RECOVERY_APPROVAL=<action substring>`
- `Approval` topic messages received on subscribed comm topics (polled every trigger maintenance tick)
- Mission `requires approval Operator for: <action>` gates `mission.start`, `mission.advance`, and `mission.resume` until approval is granted
- Deferred recovery: approval-gated actions retry automatically when approval is granted on a later tick

Runtime recovery actions publish fleet coordination commands on `/fleet/recovery`
(Command) for in-process comm buses. When `SPANDA_FLEET_MESH_URL` is set, the runtime
also posts the same action to the fleet mesh coordinator (`POST /v1/fleet/recovery`),
which relays `fleet_recovery` peer messages to registered fleet agents. Mission handoff
actions additionally publish `/fleet/continuity` and relay through `POST /v1/fleet/continuity`
for successor takeover. Deployed
agents prefer **live interpreter recovery** (`execute_recovery_on_program`) when a
program is loaded via `POST /v1/program`, running mode transitions, speed caps, mission
pause, and connectivity restart through the same dispatcher as `spanda run`. Assurance-only
fallback applies when interpreter setup fails. Use `POST /v1/recovery/execute` for direct
recovery triggers. Agent `/v1/status` exposes `recovery_engine`, `recovery_validation`,
runtime logs, and applied actions.

High-risk actions require operator approval via:

- `SPANDA_OPERATOR_APPROVAL=1` (simulation/testing)
- `SPANDA_GRANT_RECOVERY_APPROVAL=<action substring>`
- `Approval` topic messages received on subscribed comm topics
- Mission `requires approval Operator for: <action>` gates `mission.start`, `mission.advance`, and `mission.resume` until approval is granted

`spanda check --readiness-json` includes recovery-policy diagnostics (missing policies, fleet triggers without fleet, high-risk actions without Approval topics) and continuity-policy diagnostics (`continuity:policy`, `continuity:fleet`, `continuity:approval`, `continuity:handoff`). The TypeScript LSP fallback (`scripts/lsp-readiness.mts`) mirrors the same recovery and continuity diagnostics when the native CLI is unavailable. See [mission-continuity.md](./mission-continuity.md).

Recovery outcomes are recorded to `.spanda/recovery_knowledge.json` for future recommendations (no automatic code or safety rule changes).

## Example

See `examples/showcase/self_healing/rover.sd`.

## Related

- [recovery-policies.md](./recovery-policies.md) — `recovery_policy` syntax
- [fleet-distributed.md](./fleet-distributed.md) — mesh and fleet agent recovery HTTP APIs
- [verification-diagnostics.md](./verification-diagnostics.md) — `recovery:*` diagnostic categories
- [man/spanda-recovery.md](./man/spanda-recovery.md) — CLI man page
