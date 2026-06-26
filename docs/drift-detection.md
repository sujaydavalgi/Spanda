# Configuration Drift Detection

**Status:** Experimental · **Phase:** Deploy, Operate · **Priority:** P1.1

Detect mismatch between **expected** (approved baseline configuration, declared deploy posture, and program artifacts) and **actual** (live resolved configuration and on-device agent reports).

## CLI

```bash
# Compare approved baseline against live project config
spanda drift --baseline configs/approved/ --config spanda.toml

# Program + live deploy agent check (uses .spanda/deploy-agents.json)
spanda drift rover.sd --agent Rover@JetsonOrin --config spanda.toml

# Program drift against all registered deploy/fleet agents
spanda drift rover.sd --config spanda.toml

# Config subcommand alias
spanda config drift --baseline configs/approved/ rover.sd --json

# Readiness with baseline drift gates
spanda readiness rover.sd --config spanda.toml --baseline configs/approved/

# Readiness with live deploy agent attestation drift
spanda readiness examples/showcase/secure_boot/rover.sd --agent Rover@Jetson
```

## Comparison dimensions

| Dimension | Expected | Actual |
|-----------|----------|--------|
| Configuration | Baseline merged TOML | Live merged TOML |
| Fleet | Baseline fleet tree | Live fleet tree |
| Device | Baseline `DeviceRegistry` | Live device records |
| Provider / Package | Baseline manifests | Live manifests |
| Mapping | Baseline logical map | Live logical map |
| Program | `.sd` sensors/actuators | Live logical map |
| Hardware | `deploy … to <profile>` | Agent `/v1/status` `hardware_profile` |
| Firmware | Device `firmware_version` in config | Agent `/v1/status` `firmware_version` |
| Program hash | SHA-256 of `.sd` file | Agent `/v1/status` `program_hash` |
| Packages | `ResolvedSystemConfig.packages` | Agent `/v1/status` `packages` |
| Attestation | `trust.jetson` / `trust.pi` imports in program | Agent `/v1/status` `attestation_verified`, `attestation_contract`, `boot_state` |

## Agent status fields

Deploy agents (`spanda deploy agent`) and fleet agents (`spanda fleet agent`) expose drift fields on `GET /v1/status`:

- `program_hash` — set on rollout
- `hardware_profile` — from deploy assignment or rollout payload
- `firmware_version` — optional rollout metadata
- `packages` — optional rollout metadata
- `healthy` — agent health flag
- `attestation_contract` — secure-boot contract from `SPANDA_ATTESTATION_CONTRACT`
- `attestation_verified` — `true` when `SPANDA_ATTESTATION_VERIFIED=1`
- `boot_state` — optional boot posture from `SPANDA_BOOT_STATE`

See [hardware-attestation.md](./hardware-attestation.md).

## Operational drift API (Control Center)

`GET /v1/drift?baseline_id=<snapshot>` uses `detect_operational_drift_full` — config manifest drift, program alignment (when Control Center is started with `--program`), policy enforcement drift, and live fleet/deploy agent findings. Reports roll up into seven enterprise dimensions: configuration, firmware, package, provider, capability, policy, safety.

### Scheduled scans

Set `SPANDA_DRIFT_SCAN_INTERVAL_SECS` (for example `3600`) when starting Control Center to run background scans against the latest config snapshot (or `SPANDA_DRIFT_SCAN_BASELINE_ID`). Failed scans emit `ConfigDrift` alerts; high-severity findings open SRE incidents automatically.

| Endpoint | Description |
|----------|-------------|
| `GET /v1/drift/scans` | History of scheduled and manual scans |
| `POST /v1/drift/scan` | Trigger a scan (Bearer token); optional `baseline_id` in JSON body |

```bash
# Manual scan via CLI
spanda control-center drift scan --baseline-id cfg-123
spanda control-center drift scans
```

## Output

`ConfigDriftReport` — structured findings with `dimension`, `severity`, `message`, and optional `path`. Medium-or-higher severity fails the check (exit code 1).

## Foundation

- Semantic config comparison: `spanda-config::drift`
- Agent snapshot comparison: `expected_agent_states` + `detect_agent_drift`
- Readiness baseline gates: `spanda readiness --baseline`
- Readiness agent attestation gates: `spanda readiness --agent <Robot@Hardware>`

## Related

[configuration.md](./configuration.md) · [readiness.md](./readiness.md) · [platform-maturity-roadmap.md](./platform-maturity-roadmap.md)
