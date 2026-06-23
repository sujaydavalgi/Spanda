# Package example projects

Sample `spanda.toml` layouts — local dependencies, adapter packages, and provider scaffolds. Use these after [Spanda 101 Lesson 9](../../docs/spanda-101/09-packages-and-tests.md).

**Guide:** [docs/packages.md](../../docs/packages.md) · **Official registry:** [packages/registry/](../../packages/registry/) · [official-packages.md](../../docs/official-packages.md)

```bash
spanda check examples/packages/basic_project/src/main.sd
cd examples/packages/local_dependency && spanda test
```

---

## Layouts

| Project | Focus |
|---------|--------|
| [`basic_project/`](basic_project/) | Minimal manifest + `src/main.sd` |
| [`local_dependency/`](local_dependency/) | Path dependency between packages |
| [`robot_driver_package/`](robot_driver_package/) | Driver module layout |
| [`ai_provider_package/`](ai_provider_package/) | Custom AI provider scaffold |
| [`publish_mirror_project/`](publish_mirror_project/) | `spanda publish` mirror to `registry/packages/` |

---

## Adapter packages (lean-core)

Framework adapter metadata for `spanda verify-adapter`:

| Project | Integration |
|---------|-------------|
| [`nav2_adapter_package/`](nav2_adapter_package/) | Nav2 / `spanda-nav` |
| [`cartographer_adapter_package/`](cartographer_adapter_package/) | Cartographer SLAM |
| [`rtabmap_adapter_package/`](rtabmap_adapter_package/) | RTAB-Map SLAM |
| [`ros2_adapter_package/`](ros2_adapter_package/) | ROS2 transport surface |

Production bridges: optional `SPANDA_NAV2_CMD` / `SPANDA_SLAM_CMD` — see [adapters/README.md](../adapters/README.md).

---

## Try with official packages

After `spanda init`, add registry dependencies (see [registry.md](../../docs/registry.md)):

```toml
[dependencies]
spanda-ros2 = "0.1"
spanda-mqtt = "0.1"
```

Official package sources live under `packages/registry/`; runtime wiring uses [`spanda-providers`](../../crates/spanda-providers/README.md).

---

## Adding a package example

1. Create `examples/packages/<name>/` with `spanda.toml` + `src/main.sd`
2. Add a row to this README
3. Ensure `spanda check` passes from project root or package dir
4. Link from [docs/tutorials/README.md](../../docs/tutorials/README.md) if tutorial-worthy
