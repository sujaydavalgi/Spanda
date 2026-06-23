#!/usr/bin/env bash
# Copy showcase examples into the spanda crate for cargo install / crates.io publish.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST="${ROOT}/crates/spanda-cli/bundled-examples/examples/showcase"
mkdir -p "${DEST}"

for d in unsafe_ai hardware_verification capability_verification health_monitoring fleet_management replay; do
  rm -rf "${DEST}/${d}"
  cp -R "${ROOT}/examples/showcase/${d}" "${DEST}/"
done

for f in killer_demo.sd ai_safety_violation.sd hardware_compatibility.sd README.md; do
  cp "${ROOT}/examples/showcase/${f}" "${DEST}/"
done

echo "✓ Synced bundled examples to crates/spanda-cli/bundled-examples/"
