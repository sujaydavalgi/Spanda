#!/usr/bin/env bash
# Smoke verify-time integrity verification.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/policy/warehouse.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== integrity warehouse hashes =="
run_spanda integrity "$FILE" >/dev/null

echo "== integrity warehouse baseline =="
run_spanda integrity "$FILE" --baseline "$FILE" >/dev/null

echo "== integrity warehouse json =="
run_spanda integrity "$FILE" --json >/dev/null

echo "Integrity smoke OK"
