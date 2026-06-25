#!/usr/bin/env bash
# Smoke mission resource estimation.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/hardware_compatibility.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== estimate text =="
run_spanda estimate "$FILE" --target RoverV1 >/dev/null

echo "== estimate json =="
run_spanda estimate "$FILE" --target RoverV1 --json >/dev/null

echo "Estimate smoke OK"
