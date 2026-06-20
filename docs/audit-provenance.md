# Audit and Provenance

Spanda provides **first-class audit and provenance abstractions** without baking blockchain into the language core. Mission records, telemetry, and safety events are captured append-only, hashed, and optionally signed before any optional ledger anchoring.

## Why audit-first?

- Robot control loops must **never** depend on blockchain latency
- On-chain actuation commands are **not allowed** in safety-critical paths
- Audit logs work offline; anchoring is asynchronous and optional
- Future packages (`spanda-ledger-ethereum`, `spanda-provenance`, …) plug in via traits

## Language syntax

### Audit block

Declare fields to record during a mission:

```spanda
audit MissionAudit {
    record robot.pose;
    record safety.events;
    record actuator.commands;
    record ai.reasoning_trace;
}
```

### Provenance block

Configure hashing and signing for mission records:

```spanda
provenance MissionRecord {
    hash: sha256;
    signed_by: robot.identity;
}
```

### Device identity

```spanda
identity RobotIdentity {
    id: "rover-001";
    public_key: "device-key-rover-001";
}
```

### Signed record streams

```spanda
record mission_event signed_by robot.identity;
```

### Runtime API

```spanda
behavior patrol() {
    let proposal = planner.reason(prompt: "move forward");
    let action = safety.validate(proposal);
    wheels.execute(action);
    audit.record("safe_action_executed", action);
}
```

## Rust backend (`spanda-audit`)

| Trait | Purpose |
|-------|---------|
| `AuditBackend` | Append-only records, verify, export JSON |
| `LedgerBackend` | Anchor content hashes (blockchain-ready) |

MVP implementations:

- `LocalAuditBackend` — in-memory append-only log
- `JsonAuditBackend` — export as JSON
- `MockLedgerBackend` — simulates on-chain anchoring without network

## Capabilities

Audit-related packages must declare capabilities in `spanda.toml`:

- `audit.write` — append audit records
- `audit.read` — read/export audit logs
- `identity.sign` — sign records with device key
- `identity.verify` — verify signatures
- `ledger.anchor` — anchor hashes to a ledger backend

High-risk capabilities trigger validation warnings when not explicitly granted. See [security.md](./security.md).

## Examples

- `examples/std/audit_log.sd` — patrol with audit recording
- `examples/std/provenance.sd` — identity + provenance configuration
- `examples/std/device_identity.sd` — signed mission events
- `examples/std/mock_ledger.sd` — hash anchoring via mock ledger

## Related

- [future-blockchain-support.md](./future-blockchain-support.md) — optional ledger packages
- [security.md](./security.md) — capabilities and trust levels
