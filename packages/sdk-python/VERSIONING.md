# spanda-sdk versioning

The Python SDK (`spanda-sdk` on PyPI) follows [Semantic Versioning 2.0.0](https://semver.org/).

## Alignment with Control Center

| Component | Version source |
|-----------|----------------|
| `spanda-sdk` | `packages/sdk-python/pyproject.toml` → `[project].version` |
| Control Center REST | `GET /v1/version` → `api` field |
| gRPC proto | `GET /v1/version` → `grpc` field |

Bump the SDK **minor** version when new REST routes or request fields are added without breaking existing clients. Bump **major** when removing or renaming public Python APIs.

## Release tags

CI publishes on manual workflow dispatch or git tags:

```text
sdk-python-v0.4.0
```

## Pre-release checklist

1. Update `pyproject.toml` version and `CHANGELOG.md`.
2. Run `pytest` in `packages/sdk-python`.
3. Run `scripts/enterprise_ops_smoke.sh` against a local Control Center build.
4. Tag `sdk-python-vX.Y.Z` and push.
