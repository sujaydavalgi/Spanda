#!/usr/bin/env bash
# Regenerate architecture-manifest.json from architecture-manifest.yaml.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
YAML="$ROOT/scripts/architecture-manifest.yaml"
JSON="$ROOT/scripts/architecture-manifest.json"

if ! command -v ruby >/dev/null 2>&1; then
  echo "ruby is required to sync architecture-manifest.json" >&2
  exit 1
fi

ruby -ryaml -rjson -e "puts JSON.pretty_generate(YAML.load_file('$YAML'))" >"$JSON"
echo "Wrote $JSON"
