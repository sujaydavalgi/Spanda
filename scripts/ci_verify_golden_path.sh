#!/usr/bin/env bash
# Golden path for spanda verify --json CI gating (P1 adoption).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SPANDA="${SPANDA_BIN:-$ROOT/target/release/spanda}"
SOURCE="${ROOT}/examples/showcase/hardware_compatibility.sd"

if [[ ! -x "${SPANDA}" ]]; then
  cargo build -p spanda --release
  SPANDA="${ROOT}/target/release/spanda"
fi

echo "== check hardware compatibility showcase =="
"${SPANDA}" check "${SOURCE}"

echo "== verify RoverV1 (expect compatible) =="
"${SPANDA}" verify "${SOURCE}" --json --target RoverV1 | grep -q '"compatible":true'

echo "== verify ESP32 override (expect incompatible) =="
if "${SPANDA}" verify "${SOURCE}" --json --target ESP32 >/dev/null 2>&1; then
  echo "expected verify failure for ESP32 target" >&2
  exit 1
fi

echo "== verify matrix (--all-targets) =="
"${SPANDA}" verify "${SOURCE}" --json --all-targets | grep -q '"matrix"'

echo "CI verify golden path complete."
