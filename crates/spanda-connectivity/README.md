# spanda-connectivity

Connectivity and positioning type catalogs for Spanda. Extracted from `spanda-core` as part of the lean-core architecture.

## Contents

| Module | Role |
|--------|------|
| Positioning / radio catalogs | GPS, Wi-Fi, BLE, cellular type definitions |
| `hardware_types` | `HardwareProfile`, `CompatItem`, and related foundation types (Phase 8) |

Driver implementations belong in official packages: `spanda-gps`, `spanda-wifi`, `spanda-ble`, `spanda-cellular`.

Hardware compatibility **validation** lives in `spanda-connectivity-runtime`. The public **`verify_compatibility`** API is exposed through `spanda-core::hardware_verify` for embedders.

## Related

- [spanda-connectivity-runtime](../spanda-connectivity-runtime/README.md) — runtime validation hooks
- [spanda-hardware](../spanda-hardware/README.md) — builtin profile catalog
- [spanda-core](../spanda-core/README.md) — `hardware_verify` facade
