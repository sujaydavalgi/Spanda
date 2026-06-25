#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
BIN="${CARGO_TARGET_DIR:-target}/debug/spanda"
if [[ ! -x "$BIN" ]]; then
  cargo build -p spanda --quiet
fi
echo "== spoof-check program coverage =="
"$BIN" spoof-check examples/showcase/gps_spoofing/rover.sd | grep -q "PASS"
echo "== spoof-check trace plausibility (expect FAIL) =="
if "$BIN" spoof-check examples/showcase/gps_spoofing/spoof.trace; then
  echo "expected spoof.trace to fail" >&2
  exit 1
fi
echo "spoof smoke ok"
