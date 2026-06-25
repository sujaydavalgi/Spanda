# Mission Differencing

**Status:** Experimental · **Phase:** Build, Verify · **Priority:** P1.3

Compare two Spanda mission files and surface change-impact for CI and review workflows.

## CLI

```bash
spanda diff rover-v1.sd rover-v2.sd
spanda diff baseline.sd candidate.sd --json
```

Exit codes:

| Code | Meaning |
|------|---------|
| `0` | No differences |
| `1` | Differences detected |
| `2` | Deploy or safety impact flagged |

## Dimensions

| Dimension | Compared elements |
|-----------|-------------------|
| Hardware | Profile sensors, actuators, connectivity |
| Robot | Declarations, hardware binding, sensors, actuators, behaviors |
| Capability | Declared and inferred capabilities |
| Mission | Robot mission definitions |
| Safety | Safety rule summaries |
| Deploy | `deploy` bindings |
| Health | Health checks and policies |
| Kill switch | Program-level kill switches |
| Recovery / continuity | Policy declarations |
| Fleet / mission plan | Program-level orchestration |

Each change includes a recommended **impact** action (e.g. re-run verify, safety-coverage, deploy gate).

## Output

`MissionDiffReport` with `added` / `removed` / `modified` counts, `has_deploy_impact`, `has_safety_impact`, and per-change rows.

## Integration

Complements `spanda config diff` (TOML layers) and configuration drift detection. Use in PR review and CI to block unsafe mission regressions.

## Crate

`spanda-diff` — composes `spanda-ast` and `spanda-capability` inference.

See [platform-maturity-roadmap.md](./platform-maturity-roadmap.md) · [drift-detection.md](./drift-detection.md).
