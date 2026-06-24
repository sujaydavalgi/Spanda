# spanda-readiness(1)

## NAME

readiness — Evaluate operational readiness: health, safety, fleet, and deployment gates.

## SYNOPSIS

```
spanda readiness [--json] [--readiness-json] <file.sd>
```

## DESCRIPTION

Evaluate operational readiness: health, safety, fleet, and deployment gates.

## OPTIONS

`--json` / `--readiness-json` — structured readiness report

## EXAMPLES

```bash
spanda readiness robot.sd --readiness-json
```

## EXIT STATUS

0 when ready; 1 when blocking issues are found.

## FILES

Readiness reports may reference `spanda.toml` safety metadata.

## SEE ALSO

spanda-verify(1), spanda-assure(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
