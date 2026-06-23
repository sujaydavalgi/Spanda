#!/usr/bin/env bash
# Golden path for observe → fusion → world_model belief workflow (Phase 24).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE="${ROOT}/examples/showcase/world_model_patrol.sd"

cargo build -p spanda --release --quiet --target-dir "${ROOT}/target"
SPANDA="${ROOT}/target/release/spanda"

echo "== check world_model_patrol showcase =="
"${SPANDA}" check "${SOURCE}"

echo "== run with fusion → world_model hook =="
"${SPANDA}" run "${SOURCE}" --verbose 2>&1 | grep -q "world_model: fused observation"

echo "World model golden path complete."
