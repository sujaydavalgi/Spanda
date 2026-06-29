# Dependency Rules

Enforceable dependency governance for Spanda Platform v2.0.

**Parent:** [platform-architecture.md](./platform-architecture.md) · **Matrix:** [module-ownership.md](./module-ownership.md)

---

## Core rule

Every module may depend only on **layers below it** (lower index) or **the same layer** (horizontal composition).

```
Solution Blueprints  (6)
        ↓
    Interfaces       (5)
        ↓
 Platform Services   (4)
        ↓
   Core Platform     (3)
        ↓
 Language Runtime    (2)
        ↓
     Compiler        (1)
        ↓
    Foundation       (0)
```

| Dependency | Allowed? |
|------------|----------|
| Layer 4 → Layer 3 | Yes (downward) |
| Layer 4 → Layer 4 | Yes (same-layer, must stay acyclic) |
| Layer 2 → Layer 4 | **No** (upward) |
| Layer 5 → Layer 0 | Yes |

---

## What counts as a dependency

| Kind | Tracked by |
|------|------------|
| Rust path dependency in `[dependencies]` (`path = "../…"`) | `validate_architecture.py` |
| Rust `[dev-dependencies]` / `[build-dependencies]` | Not tracked (tests and tooling only) |
| npm workspace import | Manual review + TS layer map in manifest |
| Blueprint → platform API | Review in blueprint PRs |
| Registry package → provider trait | `spanda-package` resolver |

Only **upward** Rust path dependencies fail CI (unless waived). Same-layer and downward edges are always allowed.

---

## Anti-patterns

| Anti-pattern | Example | Fix |
|--------------|---------|-----|
| Runtime imports API server | `spanda-interpreter` → `spanda-api` | Call through trait at core layer |
| Duplicate entity struct | `RobotRecord` in analysis crate | Use `EntityRecord` from `spanda-config` |
| Blueprint adds workspace crate | New `spanda-warehouse` crate | Use packages + examples only |
| Service owns parsing | `spanda-trust` embeds lexer | Accept AST/program summary from driver |
| Facade in first-party apps | `spanda-cli` → `spanda-core` | Import owning crate directly |

---

## Waiver process

Production upward dependencies are tracked in `scripts/architecture-manifest.yaml` under `dependency_waivers`. As of Phase 8, **all baselines are cleared** — Rust, TypeScript, and SCC waiver lists are empty.

Each waiver entry (when needed) has:

- `from` / `to` crate names
- `reason` — why the edge exists today
- `ticket` — tracking ID (`ARCH-xxx` or `TS-ARCH-xxx`)

**Adding a new waiver** requires:

1. Architecture review (explain why a downward refactor is not immediate)
2. Entry in the appropriate waiver list with ticket ID
3. Run `scripts/sync_architecture_manifest.sh`
4. Note in PR description

**Removing a waiver** is the default outcome of refactors — delete the entry in the same PR that eliminates the edge.

Current counts (should stay at zero):

```bash
python3 scripts/validate_architecture.py
# Layer violations (waived): 0
# TypeScript layer violations (waived): 0
# Circular dependencies (waived): 0
```

---

## Circular dependencies

Strongly connected components (SCCs) with more than one crate indicate architectural coupling.

The production dependency graph must stay **acyclic**. `validate_architecture.py` builds the graph from `[dependencies]` only (not dev/build deps). **Any new strongly connected component** in production dependencies fails CI.

To inspect:

```bash
python3 scripts/validate_architecture.py --verbose
```

---

## Package boundaries

The core workspace stays minimal. New functionality defaults to **registry packages** unless it is:

- Language syntax or semantics
- Compiler functionality
- Runtime execution infrastructure
- Entity infrastructure
- Verification framework contracts
- Core public APIs

Everything else — ROS2, MQTT, vision, SLAM, industry logic — belongs in `packages/registry/`.

See [lean-core.md](./lean-core.md) and [how-packages-work.md](./how-packages-work.md).

---

## TypeScript mirror

The root `src/` tree mirrors lean-core layers. TypeScript modules should follow the same dependency direction:

- `src/parser` must not import from `src/readiness*`
- Platform service mirrors may call core types, not vice versa

Full TS layer map: `scripts/architecture-manifest.yaml` → `typescript_packages`.

---

## Validation commands

```bash
# Full check (CI)
python3 scripts/validate_architecture.py

# Regenerate JSON after manifest edit
scripts/sync_architecture_manifest.sh

# Dependency graph
python3 scripts/validate_architecture.py --write-graph docs/architecture-dependency-graph.dot
```

---

## Dependency graph

Machine-readable graph: [architecture-dependency-graph.dot](./architecture-dependency-graph.dot)

Visual overview (simplified):

```mermaid
flowchart TB
  CLI[spanda CLI] --> API[spanda-api]
  CLI --> RDY[spanda-readiness]
  API --> CFG[spanda-config]
  API --> INT[spanda-interpreter]
  INT --> RT[spanda-runtime]
  INT --> CFG
  RDY --> CFG
  RDY --> HW[spanda-hardware]
  DRV[spanda-driver] --> INT
  DRV --> LEX[spanda-lexer]
  LEX --> AST[spanda-ast]
```

Full edge list: 399 production path dependencies across 75 workspace crates (see `validate_architecture.py` output).
