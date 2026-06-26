#!/usr/bin/env bash
# Verify PyPI and npm SDK publish readiness without publishing.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
echo "== Python SDK =="
pip install -q -e "packages/sdk-python[dev]"
pytest packages/sdk-python/tests -q
python -m pip install -q build
(cd packages/sdk-python && python -m build >/dev/null)
echo "Python wheel OK ($(ls packages/sdk-python/dist/*.whl | tail -1))"
echo "== npm @spanda/web =="
npm ci
npm run build --workspace=@spanda/web
(cd packages/web && npm pack >/dev/null)
echo "npm pack OK"
echo "Publish readiness verified. Tag sdk-python-v* or npm-web-v* to release."
