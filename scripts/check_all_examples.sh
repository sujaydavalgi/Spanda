#!/usr/bin/env bash
# Type-check Spanda examples with manifest-driven expect-fail and skip lists.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
MANIFEST="$ROOT/scripts/examples-check-manifest.txt"
LIST="$(mktemp)"
trap 'rm -f "$LIST"' EXIT
find examples -name '*.sd' | sort >"$LIST"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  SPANDA=("$SPANDA_BIN")
elif [[ -x "$ROOT/target/release/spanda" ]]; then
  SPANDA=("$ROOT/target/release/spanda")
else
  SPANDA=(cargo run -q -p spanda-cli --)
fi

manifest_has() {
  local kind="$1"
  local file="$2"
  grep -q "^${kind} ${file}$" "$MANIFEST" 2>/dev/null || return 1
}

check_file() {
  local file="$1"
  local pkg_dir
  pkg_dir="$(dirname "$file")"
  while [[ "$pkg_dir" != "$ROOT/examples" && "$pkg_dir" != "/" && "$pkg_dir" != "." ]]; do
    if [[ -f "$pkg_dir/spanda.toml" ]]; then
      local rel="${file#$pkg_dir/}"
      (cd "$pkg_dir" && "${SPANDA[@]}" check "$rel")
      return $?
    fi
    pkg_dir="$(dirname "$pkg_dir")"
  done
  "${SPANDA[@]}" check "$file"
}

total=0
passed=0
failed=0
skipped=0
expected_failed=0

while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  total=$((total + 1))
  if manifest_has skip "$file"; then
    skipped=$((skipped + 1))
    continue
  fi
  if check_file "$file" >/dev/null 2>&1; then
    if manifest_has expect-fail "$file"; then
      echo "UNEXPECTED PASS (expected fail): $file"
      failed=$((failed + 1))
    else
      passed=$((passed + 1))
    fi
  else
    if manifest_has expect-fail "$file"; then
      expected_failed=$((expected_failed + 1))
    else
      echo "FAIL: $file"
      failed=$((failed + 1))
    fi
  fi
done <"$LIST"

echo "Examples: $passed passed, $expected_failed expected-fail, $skipped skipped, $failed unexpected failures (of $total)"

if [[ "$failed" -gt 0 ]]; then
  exit 1
fi
