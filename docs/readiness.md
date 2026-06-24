# Operational Readiness

Spanda's **Operational Readiness Engine** answers one question:

> Can this robot safely perform this mission right now?

It composes existing platform gates — hardware verification, capability verification, health checks, connectivity validation, safety rules, and mission requirements — into a single weighted **readiness score** and go/no-go report.

## Quick start

```bash
spanda readiness examples/showcase/readiness/rover.sd
spanda readiness examples/showcase/readiness/rover.sd --target RoverV1
spanda readiness examples/showcase/readiness/rover.sd --runtime
spanda readiness examples/showcase/readiness/rover.sd --runtime --inject-health-faults
spanda readiness examples/showcase/readiness/rover.sd --json
spanda readiness examples/showcase/readiness/rover.sd --agent-json
spanda check examples/showcase/readiness/rover.sd --readiness-json --json
spanda demo readiness
```

### Runtime and agent readiness

- **`--runtime`** — evaluate against live hardware monitor signals (health checks use runtime fault/event state).
- **`--inject-health-faults`** — simulate degraded sensors for what-if analysis (pairs with `--runtime`).
- **`GET /v1/readiness`** on deploy and fleet agents; query `runtime=true` and `inject_health_faults=true` for on-device evaluation.
- **`POST /v1/program`** on deploy and fleet agents to upload `.sd` source for live readiness.
- **Remote CLI:** `spanda deploy agent readiness <Robot@Hardware>` and `spanda fleet agent readiness <RobotName>`.
- **`--agent-json`** on `spanda readiness` — same JSON envelope as agent `GET /v1/readiness` (for scripts, CI, and local parity checks).
- **`spanda check --readiness-json`** — merges operational readiness diagnostics with **recovery-policy** diagnostics (`recovery:policy`, `recovery:approval`, `recovery:fleet`). See [verification-diagnostics.md](./verification-diagnostics.md).
- **Web playground** — switch to **Operations** view in `packages/web` for local scoring or agent fetch.

Example output:

```
Mission Ready: YES
Score: 92/100

Issues:
* LTE signal weak
* Camera calibration due in 5 days
* Battery below recommended threshold
```

## Readiness factors

| Factor | Source |
|--------|--------|
| Hardware | `spanda verify` / hardware profiles |
| Capabilities | Capability registry + minimum hardware |
| Health | `health_check` declarations |
| Connectivity | Connectivity policy + hardware |
| Safety | Minimum capabilities + kill switches |
| Battery | Mission duration vs battery budget |
| Storage / Compute | Resource budgets (when declared) |
| Packages / Providers | Traceability matrix |
| Mission Requirements | `mission { requires capabilities [...] }` |

## Types

- `ReadinessStatus` — Ready, Degraded, NotReady, Unknown
- `ReadinessReport` — full evaluation with score and issues
- `ReadinessScore` — weighted total and per-factor breakdown
- `ReadinessIssue` — severity, factor, message, suggested action
- `ReadinessPolicy` — minimum score threshold and factor weights

## Related commands

| Command | Purpose |
|---------|---------|
| `spanda readiness <file.sd>` | Unified readiness evaluation |
| `spanda verify mission <file.sd>` | Mission achievability check |
| `spanda fleet readiness <file.sd>` | Fleet-level readiness |
| `spanda twin readiness <file.sd>` | Physical vs digital twin drift |
| `spanda audit <file.sd>` | Autonomous safety auditor |

## Crate

Rust API: `spanda-readiness` (`evaluate_readiness`, `ReadinessReport`, …)

See also: [Mission Verification](mission-verification.md), [Fleet Readiness](fleet-readiness.md), [Safety Reporting](safety-reporting.md).
