# Hardware & verification examples

Programs for hardware profiles, deploy targets, capability traceability, health checks, and kill switches.

**Guides:** [hardware-compatibility.md](../../docs/hardware-compatibility.md) · [capability-traceability.md](../../docs/capability-traceability.md) · [health-checks.md](../../docs/health-checks.md) · [kill-switch.md](../../docs/kill-switch.md)

```bash
spanda verify examples/hardware/capability_verification.sd --health --capabilities
spanda check examples/hardware/capability_verification.sd --verification-json
spanda health robot examples/hardware/capability_verification.sd --json
```

---

## Files

| File | Focus |
|------|--------|
| [`capability_verification.sd`](capability_verification.sd) | Phase 27 flagship — `uses hardware`, `exposes capabilities`, `requires_capability`, health, kill switch |
| [`rover_deploy.sd`](rover_deploy.sd) | `hardware` + `deploy` wiring |
| [`full_compat.sd`](full_compat.sd) | Compatibility matrix walkthrough |

---

## Related feature examples

| Feature | File |
|---------|------|
| Fleet `require` clauses | [`../features/fleet_health_require.sd`](../features/fleet_health_require.sd) |
| Kill switch handler | [`../features/kill_switch.sd`](../features/kill_switch.sd) |
| Remote-signed kill switch | [`../security/remote_signed_kill_switch.sd`](../security/remote_signed_kill_switch.sd) |
| Verify walkthrough | [`../integration/verify_walkthrough.sd`](../integration/verify_walkthrough.sd) |
