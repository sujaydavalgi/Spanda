# Publish mirror example

Minimal package demonstrating `spanda publish` validation and in-repo mirror to `registry/packages/`.

```bash
cd examples/packages/publish_mirror_project
spanda check src/main.sd
spanda publish
```

After publish, the bundle appears under `registry/packages/publish-mirror-demo/0.1.0/`. Maintainers run `./scripts/build-registry.sh` to refresh `registry/index.json` checksums and signatures.

See [docs/packages.md](../../docs/packages.md) and [docs/registry.md](../../docs/registry.md).
