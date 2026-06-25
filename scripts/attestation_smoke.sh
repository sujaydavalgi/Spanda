#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== attestation integration tests =="
cargo test -p spanda-tamper --test attestation_integration live_attestation_endpoint_merges_secure_boot_coverage -q
cargo test -p spanda-config agent_drift_detects_missing_secure_boot_attestation -q
cargo test -p spanda-ota --test agent_attestation agent_status_includes_attestation_from_environment -q

echo "attestation smoke ok"
