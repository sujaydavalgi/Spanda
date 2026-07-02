# Plugin Security

Spanda plugins run under a **defense-in-depth** model: manifest validation, capability review, trust tiers, signature verification, sandboxing, and audit logging.

## Requirements

| Control | Behavior |
|---------|----------|
| **Signature verification** | Ed25519 over canonical plugin payload |
| **Manifest validation** | Schema, plugin type, capability names |
| **Capability review** | Dangerous capabilities require `--approve-dangerous` |
| **Sandboxing** | WASM default; `[security].sandbox = true` |
| **Trust tier** | Registry tier gates install (`blocked` rejected) |
| **Audit logging** | Install, enable, disable, hook dispatch logged |
| **Version compatibility** | Spanda semver + API `v1` enforced at install |

## Blocked installs

Install is **rejected** when:

- Plugin registry tier is **`blocked`**
- **Official** plugin has `signed = true` but signature verification fails
- Plugin requests **dangerous capabilities** without approval
- Plugin is **incompatible** with current Spanda or plugin API version

## Dangerous capabilities

Require `spanda plugin install --approve-dangerous`:

- `entity.write`, `device.write`, `filesystem.write`
- `network.outbound`, `readiness.write`, `recovery.write`

## Trust tiers

| Tier | Install |
|------|---------|
| `official` | Allowed; signature required when `signed = true` |
| `verified` | Allowed |
| `community` | Allowed |
| `experimental` | Allowed |
| `deprecated` | Allowed (discouraged) |
| `blocked` | **Denied** |

## Relationship to packages

Plugin security is **orthogonal** to package trust. Packages continue to use `spanda.toml` and registry provenance unchanged.
