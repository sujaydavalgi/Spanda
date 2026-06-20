# Security

Spanda separates **language-level agent capabilities**, **robot safety**, and **package-level capability declarations**. Audit and blockchain-related libraries integrate through the package capability system.

## Capability layers

### 1. Agent capabilities (`.sd`)

```spanda
agent planner {
  can [ read(lidar), propose_motion, plan ];
  plan { ... }
}
```

Enforced at runtime by the interpreter.

### 2. Package capabilities (`spanda.toml`)

```toml
[capabilities]
required = [
  "camera.read",
  "audit.write",
  "identity.sign",
  "ledger.anchor",
  "network.outbound"
]
```

Validated by `spanda-package` before install/publish.

### 3. Known capabilities

| Capability | Risk | Description |
|------------|------|-------------|
| `audit.write` | High | Append tamper-evident audit records |
| `audit.read` | Low | Export/read audit logs |
| `identity.sign` | High | Sign telemetry and mission logs |
| `identity.verify` | Low | Verify device signatures |
| `ledger.anchor` | High | Anchor content hashes (async, non-control-path) |
| `network.outbound` | High | Outbound network access |
| `actuator.execute` | High | Direct actuator control |
| `actuator.execute.safe` | Medium | Actuator control via `SafeAction` only |

High-risk capabilities (`ledger.anchor`, `identity.sign`, `audit.write`, `actuator.execute`, `network.outbound`) produce **validation warnings** when packages declare them without application approval.

## Safety levels

```toml
[safety]
level = "simulation_only"   # experimental | simulation_only | hardware_safe | certified
can_control_actuators = false
requires_review = true
```

`simulation_only` packages **cannot** declare `can_control_actuators = true`.

## std.security types

| Type | Purpose |
|------|---------|
| `Identity` / `RobotIdentity` | Device identity (id + public key) |
| `Signature` | Cryptographic signature over payload |
| `Permission` | Granted capability token |
| `TrustLevel` | Trust tier for devices and packages |

## Crypto builtins

From `std.crypto` (re-exported as builtins):

- `sha256(data)` — content hash
- `sign(data, key)` — sign payload
- `verify_signature(data, signature, key)` — verify signature

## Rules for audit/blockchain libraries

1. **Never** block robot control on ledger confirmation
2. **Never** send actuation commands through blockchain transports
3. Declare all required capabilities in `spanda.toml`
4. Use `AuditBackend` / `LedgerBackend` traits — do not extend core syntax for chain-specific features
5. Prefer `hardware_safe` or `certified` safety levels for production audit packages

## Related

- [audit-provenance.md](./audit-provenance.md)
- [spanda-toml.md](./spanda-toml.md)
- [packages.md](./packages.md)
