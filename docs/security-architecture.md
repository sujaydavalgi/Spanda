# Security Architecture

Spanda separates **security contracts** (core) from **crypto backends** (packages and `spanda-security`).

## Core security surface

| Component | Crate / module | Role |
|-----------|----------------|------|
| Capability model | `spanda-security` | Grant/deny runtime operations |
| Identity and trust | `spanda-security` | Source IDs, trust boundaries |
| Encrypted messages | `spanda-core` + `spanda-security` | `EncryptedMessage`, `VerifiedMessage`, AES-256-GCM |
| Secure comm policy | `spanda-core/transport_security` | TLS negotiation, wire frames |
| Audit records | `spanda-audit` | Append-only provenance |
| Security CLI | `spanda-cli` | `spanda security check`, `spanda security audit` |

## Lean-core boundary

Core owns:

- Type definitions for signed/verified messages
- Trust-boundary validation hooks in the comm router
- Capability checks before actuator execution
- `CryptoProvider` trait (packages implement vendor HSM/cloud KMS backends)

Core does **not** own:

- Blockchain or ledger implementations → `spanda-ledger` package
- Cloud secret stores → `spanda-cloud` or environment `secrets` blocks
- Vendor TPM/HSM drivers

## Provider integration

```rust
// CryptoProvider — implemented by spanda-security defaults or spanda-ledger
pub trait CryptoProvider {
    fn hash(&self, algorithm: &str, payload: &[u8]) -> ProviderResult<Vec<u8>>;
    fn sign(&self, key_id: &str, payload: &[u8]) -> ProviderResult<Vec<u8>>;
    fn verify(&self, key_id: &str, payload: &[u8], signature: &[u8]) -> ProviderResult<bool>;
}
```

Transport encryption uses `TransportSecurityConfig` and `TlsTransportSession` in core; live TLS/mTLS handshakes are optional features on transport shims (`spanda-mqtt`, `spanda-ros2`).

## Deploy and OTA security

- Deploy bundles: Ed25519 signatures (`spanda-core/deploy_bundle`)
- Agent verification: `--require-signature`, `--require-hash`, `--require-certify`
- Certification proofs: `spanda certify prove`

OTA rollout security moves to `spanda-ota` package over time; CLI commands remain unchanged via compatibility shims.

## Related docs

- [secure-communication.md](./secure-communication.md)
- [identity.md](./identity.md)
- [secrets.md](./secrets.md)
- [trust-boundaries.md](./trust-boundaries.md)
- [lean-core.md](./lean-core.md)
