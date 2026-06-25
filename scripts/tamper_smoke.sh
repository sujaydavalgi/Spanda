#!/usr/bin/env bash
# Smoke verify-time tamper analysis.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/policy/warehouse.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== tamper-check warehouse =="
run_spanda tamper-check "$FILE" >/dev/null

echo "== tamper-check warehouse json =="
run_spanda tamper-check "$FILE" --json >/dev/null

echo "Tamper smoke OK"
