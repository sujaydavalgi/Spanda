#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${CARGO_TARGET_DIR:-$ROOT/target}/debug/spanda"
ROVER="$ROOT/examples/showcase/tamper_policy/rover.sd"

cd "$ROOT"
cargo build -p spanda -q

echo "== tamper-check policy coverage =="
OUTPUT="$("$BIN" tamper-check "$ROVER" 2>&1 || true)"
echo "$OUTPUT"
echo "$OUTPUT" | grep -q "tamper_policy declared"

echo "== sim tamper policy dispatch =="
SIM="$("$BIN" sim "$ROVER" --inject-security-faults 2>&1 || true)"
echo "$SIM"
echo "$SIM" | grep -q "tamper:"

echo "tamper policy smoke ok"
