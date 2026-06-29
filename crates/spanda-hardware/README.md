# spanda-hardware

Builtin robot hardware profile catalog shared by `spanda-core::hardware_verify` and `spanda-package` validation.

Extracted to break the `spanda-package` → `spanda-core` dependency cycle during the Phase 4 compiler split.

## Phase 8 note

`HardwareProfile` and `CompatItem` foundation types now live in [`spanda-connectivity`](../spanda-connectivity/README.md). This crate re-exports profiles and compatibility helpers for callers that import hardware catalogs directly.

Public embedders should call `spanda_core::verify_compatibility` rather than depending on this crate.

## Related

- [spanda-connectivity](../spanda-connectivity/README.md) — hardware profile foundation types
- [spanda-core](../spanda-core/README.md) — `hardware_verify` facade
