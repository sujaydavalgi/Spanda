#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
BIN="${CARGO_TARGET_DIR:-target}/debug/spanda"
cargo build -p spanda -q
echo "== spoof-check program coverage =="
"$BIN" spoof-check examples/showcase/gps_spoofing/rover.sd | grep -q "PASS"
echo "== spoof-check trace plausibility (expect FAIL) =="
if "$BIN" spoof-check examples/showcase/gps_spoofing/spoof.trace; then
  echo "expected spoof.trace to fail" >&2
  exit 1
fi
echo "== spoof-check trace with mock ML backend =="
export SPANDA_SPOOFING_ML_BACKEND=mock
SPOOF_ML="$("$BIN" spoof-check examples/showcase/gps_spoofing/spoof.trace 2>&1 || true)"
echo "$SPOOF_ML" | grep -q "ML alerts merged: 1"
unset SPANDA_SPOOFING_ML_BACKEND
echo "== spoof-check trace with file ML backend =="
export SPANDA_SPOOFING_ML_BACKEND=file
export SPANDA_SPOOFING_ML_ALERTS_PATH="${ROOT}/examples/showcase/gps_spoofing/fixtures/ml-alerts.json"
SPOOF_FILE="$("$BIN" spoof-check examples/showcase/gps_spoofing/spoof.trace 2>&1 || true)"
echo "$SPOOF_FILE" | grep -q "ML alerts merged: 1"
unset SPANDA_SPOOFING_ML_BACKEND SPANDA_SPOOFING_ML_ALERTS_PATH
cargo test -p spanda-spoofing --test ml_integration -q
echo "spoof smoke ok"
