# Demos & examples

[← Overview](./README.md)

> **Examples ladder & `std.*`:** [library.md](./library.md) · **`.sd` samples:** [code-samples.md](./code-samples.md)

## Quick start (5 minutes)

```bash
./scripts/install.sh
# Or: cargo install --path crates/spanda-cli --locked

spanda demo rover
spanda demo assurance

spanda check examples/showcase/killer_demo.sd
spanda verify examples/showcase/hardware_compatibility.sd
spanda sim examples/showcase/killer_demo.sd
```

Walkthrough: [killer-demo.md](../killer-demo.md) · Video script: [demo-script.md](../demo-script.md)

## Flagship pillars

| Pillar | Purpose | Command |
|--------|---------|---------|
| **Safety** | Block unsafe AI at compile time | `spanda check examples/showcase/ai_safety_violation.sd` |
| **Verify** | Hardware fit before deploy | `spanda verify examples/showcase/hardware_compatibility.sd --json` |
| **Sim** | Patrol without hardware | `spanda sim examples/showcase/killer_demo.sd` |
| **Platform** | Packages → providers → replay | `cd examples/showcase/autonomous_rover && spanda install && spanda run src/rover.sd --trace-providers` |
| **Assurance** | Mission assurance CLI suite | `spanda demo assurance` |

Showcase index: [examples/showcase/README.md](../../examples/showcase/README.md)

## Learn Spanda

| Track | Guide | Time |
|-------|-------|------|
| Plain English | [Spanda for Dummies](../spanda-for-dummies/README.md) | ~45 min |
| Hands-on course | [Spanda 101](../spanda-101/README.md) | ~3 hours |
| Quickstart | [Getting started](../getting-started.md) | ~10 min |

Master examples index: [examples/README.md](../../examples/README.md)
