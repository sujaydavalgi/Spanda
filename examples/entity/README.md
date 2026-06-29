# Entity Model Examples

Programs and workflows demonstrating the Unified Entity Model across verification, graph traversal, readiness, trust, and traceability.

## Traceability chain

```text
Operator
    ↓
Mission (PatrolMission)
    ↓
Robot (rover-001)
    ↓
Camera / Lidar / GPS (devices)
    ↓
Firmware / Provider (spanda-gps, spanda-lidar, spanda-canbus)
    ↓
Package
```

## Programs

| File | Purpose |
|------|---------|
| [entity_verify.sd](./entity_verify.sd) | Patrol robot for `spanda entity verify` with warehouse fixture |

## CLI workflows

Use the warehouse fixture config (`crates/spanda-config/tests/fixtures/warehouse/spanda.toml`):

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml
PROGRAM=examples/entity/entity_verify.sd

# Inventory
spanda entity list --config "$CONFIG"
spanda entity inspect rover-001 --config "$CONFIG"

# Graph and relationships
spanda entity graph --config "$CONFIG" --json
spanda entity relationships rover-001 --config "$CONFIG"

# Operational dimensions
spanda entity health rover-001 --config "$CONFIG"
spanda entity readiness rover-001 --config "$CONFIG"
spanda entity trust gps-001 --config "$CONFIG"

# Unified verification (Phase 2)
spanda entity verify rover-001 --config "$CONFIG"
spanda entity verify rover-001 --program "$PROGRAM" --config "$CONFIG" --dependencies

# Query
spanda entity query --kind robot --config "$CONFIG"
spanda entity search rover --config "$CONFIG"

# Traceability
spanda entity traceability --entity-id rover-001 --config "$CONFIG"
```

## REST API

```bash
curl -s http://127.0.0.1:8080/v1/entities/rover-001/verify \
  -H 'Content-Type: application/json' \
  -d '{"include_dependencies":true}'
```

## SDK

```typescript
const report = await client.verifyEntity("rover-001", { includeDependencies: true });
```

## Related docs

- [entity-model.md](../../docs/entity-model.md)
- [entity-verification.md](../../docs/entity-verification.md)
- [entity-integration-report.md](../../docs/entity-integration-report.md)
