# spanda-deploy-http

Minimal HTTP/1.1 and TLS helpers for Spanda deploy agents, fleet agents, and mesh coordinators.

Extracted from `spanda-core` for the lean-core package architecture.

## Modules

| Module | Purpose |
|--------|---------|
| Core (`lib.rs`) | `http_request`, `parse_http_url`, TLS server/client, deploy agent request/response parsing |
| `fleet_recovery` | `FleetRecoveryRequest`, `FleetRecoveryResponse`, `relay_recovery_via_mesh` — HTTP client for `POST /v1/fleet/recovery` on the fleet mesh coordinator |

The `fleet_recovery` client is used by the runtime (`spanda-interpreter`) and re-exported through `spanda-fleet` so recovery mesh relay does not create a crate dependency cycle.

## Example

```rust
use spanda_deploy_http::{relay_recovery_via_mesh, FleetRecoveryRequest};

let response = relay_recovery_via_mesh(
    "http://127.0.0.1:9700",
    &FleetRecoveryRequest {
        action: "pause mission".into(),
        fleet_name: Some("PatrolFleet".into()),
        from_robot: Some("RoverAlpha".into()),
        members: vec!["RoverBeta".into()],
    },
    Some("token"),
)?;
```

See [fleet-distributed.md](../../docs/fleet-distributed.md) and [self-healing.md](../../docs/self-healing.md).
