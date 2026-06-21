#!/usr/bin/env bash
# Build curated registry tarballs into registry/packages/<name>/<version>.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PACKAGES=(spanda-openai spanda-ros2)

for name in "${PACKAGES[@]}"; do
  src="$ROOT/packages/registry/$name"
  if [[ ! -f "$src/spanda.toml" ]]; then
    echo "missing $src/spanda.toml" >&2
    exit 1
  fi
  version=$(grep '^version' "$src/spanda.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
  dest_dir="$ROOT/registry/packages/$name"
  mkdir -p "$dest_dir"
  tmp=$(mktemp -d)
  tar -czf "$tmp/$name-$version.tar.gz" -C "$src" .
  cp "$tmp/$name-$version.tar.gz" "$dest_dir/$version"
  rm -rf "$tmp"
  echo "✓ registry/packages/$name/$version"
done

echo "✓ Registry index: registry/index.json"
