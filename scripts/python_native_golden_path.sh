#!/usr/bin/env bash
# Golden path for in-process Python FFI (PyO3 python-native feature).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SPANDA="${SPANDA_BIN:-$ROOT/target/release/spanda}"
SOURCE="${ROOT}/examples/ffi_python_extern.sd"

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 not found; skip python-native golden path" >&2
  exit 0
fi

if ! python3 -c "import sys; print(sys.version)" >/dev/null 2>&1; then
  echo "python3 not usable; skip python-native golden path" >&2
  exit 0
fi

echo "== spanda-bridge PyO3 unit test =="
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
cargo test -p spanda-bridge --features python-native native_py_add_when_available -- --nocapture

echo "== build spanda-cli with python-native =="
cargo build -p spanda-cli --release --features python-native
SPANDA="${ROOT}/target/release/spanda"

echo "== check and run ffi_python_extern (in-process) =="
"${SPANDA}" check "${SOURCE}"
unset SPANDA_PYTHON_SUBPROCESS
"${SPANDA}" run "${SOURCE}"

echo "Python native golden path complete."
