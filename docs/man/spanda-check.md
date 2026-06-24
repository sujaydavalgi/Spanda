# spanda-check(1)

## NAME

check — Type-check and parse a Spanda program or project.

## SYNOPSIS

```
spanda check [--json] [--verification-json] [--readiness-json] [<file.sd> | --project]
```

## DESCRIPTION

Type-check and parse a Spanda program or project. Optional JSON flags emit span-aware diagnostics for IDE, CI, and LSP integration.

## OPTIONS

`--json` — machine-readable type-check diagnostics

`--verification-json` — capability, traceability, minimum-hardware, health, and kill-switch diagnostics. See [verification-diagnostics.md](../verification-diagnostics.md).

`--readiness-json` — operational readiness plus recovery-policy diagnostics (`recovery:policy`, `recovery:approval`, `recovery:fleet`). Requires a successful check.

`--project` — check all modules in the current project

## EXAMPLES

```bash
spanda check examples/rover.sd
spanda check examples/showcase/self_healing/rover.sd --readiness-json --json
spanda check --project
```

## SEE ALSO

spanda-verify(1), spanda-run(1), [spanda-recovery(1)](./spanda-recovery.md), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
