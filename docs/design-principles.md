# Design Principles

Guiding principles for Spanda Platform architecture and contribution decisions.

**Parent:** [platform-architecture.md](./platform-architecture.md)

---

## 1. Entity-first

The Entity Model is the canonical foundation. Robots, missions, packages, providers, humans, devices, fleets, wearables, and facilities are specialized entities — not parallel data structures.

**Do:** Extend `EntityRecord` and entity kinds in `spanda-config`.  
**Don't:** Introduce `RobotRecord` or blueprint-local inventory types.

---

## 2. Layer discipline

Dependencies flow downward. Interfaces aggregate; blueprints compose; services operate; core defines; runtime executes; compiler analyzes.

When tempted to import upward, extract a trait or type to the lower layer first.

See [dependency-rules.md](./dependency-rules.md).

---

## 3. Lean core, rich ecosystem

The workspace stays minimal. Domain behavior belongs in registry packages unless it is language syntax, compiler infrastructure, runtime execution, entity infrastructure, verification contracts, or core APIs.

See [lean-core.md](./lean-core.md).

---

## 4. Single responsibility per service

Each platform service answers one operational question. Readiness gates; assurance evidences; diagnosis explains; trust scores; telemetry records.

See [platform-services.md](./platform-services.md).

---

## 5. Blueprints compose, never fork

Solution blueprints import packages and call CLI/API surfaces. They do not add workspace crates or platform features.

If a blueprint needs a capability, implement it in the platform first.

---

## 6. Shared models across APIs

CLI JSON, REST, gRPC, and SDKs share entity and readiness payloads. Avoid duplicated DTOs that drift.

Version all public API surfaces (path prefix, proto semver, crate/npm/PyPI semver).

---

## 7. Events everywhere

Subsystems publish to the common event model. Features that need audit, replay, or Control Center visibility must emit events — not ad-hoc logs only.

See [event-model.md](./event-model.md).

---

## 8. Package-first providers

Provider traits live in core/runtime; implementations live in packages. Official packages under `packages/registry/` are the default integration path for ROS2, MQTT, vision, fleet, etc.

See [provider-interfaces.md](./provider-interfaces.md).

---

## 9. Safety is structural

Safety types (`ActionProposal` vs `SafeAction`), units, hardware verification, and capability traceability are compiler/runtime concerns — not optional lint rules.

---

## 10. Verify before operate

Operational readiness and assurance are first-class gates. Deployment paths (`spanda verify`, readiness API, OTA rollout) consume the same evaluation engines.

See [readiness.md](./readiness.md), [ci-verify.md](./ci-verify.md).

---

## 11. Document and enforce

Architecture decisions are worthless without enforcement.

- Module ownership → `architecture-manifest.yaml`
- Dependency rules → `validate_architecture.py` in CI
- Public API docs → `validate_documentation.py`
- Feature status → [feature-status.md](./feature-status.md)

---

## 12. Incremental refactor over big bang

Known upward dependencies and the compile-run-verify SCC are baselined with waivers. New violations are blocked; existing waivers are removed incrementally — not ignored permanently.

---

## Decision checklist

Before merging a feature, confirm:

- [ ] Module assigned to a layer and owner in the manifest
- [ ] No new upward dependencies (or waiver with ticket)
- [ ] No new SCC members
- [ ] Entity model used for inventory/state (if applicable)
- [ ] Events emitted for operational state changes
- [ ] Docs updated for user-visible behavior
- [ ] Blueprints unchanged unless composing the feature
