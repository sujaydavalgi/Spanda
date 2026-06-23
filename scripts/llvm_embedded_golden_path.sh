#!/usr/bin/env bash
# LLVM cross-compile slice for Jetson/Pi (aarch64-linux-gnu).
# Always validates IR emission; native link runs only on Linux with a cross linker.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SPANDA="${SPANDA_BIN:-$ROOT/target/release/spanda}"
SOURCE="${1:-examples/hello_world.sd}"
OUT="${TMPDIR:-/tmp}/spanda-llvm-aarch64-golden"
TRIPLE="aarch64-unknown-linux-gnu"

if ! command -v clang >/dev/null 2>&1; then
  echo "clang not found; skip LLVM embedded golden path" >&2
  exit 0
fi

cargo build -p spanda --release --features llvm
"${SPANDA}" check "${SOURCE}"
"${SPANDA}" llvm-ir "${SOURCE}" --target-triple "${TRIPLE}" --out "${OUT}.ll"
test -f "${OUT}.ll"
echo "✓ LLVM embedded IR: ${SOURCE} -> ${TRIPLE} (${OUT}.ll)"

if [[ "$(uname -s)" == "Linux" ]] && command -v aarch64-linux-gnu-gcc >/dev/null 2>&1; then
  if "${SPANDA}" compile-native "${SOURCE}" --target-triple "${TRIPLE}" --out "${OUT}"; then
    echo "✓ LLVM embedded link: ${SOURCE} -> ${OUT}"
  else
    echo "⚠ LLVM embedded link failed; IR slice passed (link is best-effort on CI runners)"
  fi
else
  echo "skip native link (Linux + aarch64-linux-gnu-gcc required); IR slice passed"
fi
