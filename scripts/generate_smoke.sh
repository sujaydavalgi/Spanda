#!/usr/bin/env bash
# Smoke guardrailed generate and suggest commands.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/readiness/rover.sd"
OUT="${ROOT}/target/generate_mission_smoke.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

mkdir -p "${ROOT}/target"

echo "== generate mission =="
run_spanda generate mission --out "$OUT" >/dev/null

echo "== generate mission json =="
run_spanda generate mission --json >/dev/null

echo "== suggest readiness rover =="
run_spanda suggest "$FILE" >/dev/null || true

echo "Generate smoke OK"
