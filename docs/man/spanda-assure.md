# spanda-assure(1)

## NAME

assure — Run assurance workflows: anomaly coverage, prognostics, and assurance cases.

## SYNOPSIS

```
spanda assure [--json] <file.sd>
```

## DESCRIPTION

Run assurance workflows: anomaly coverage, prognostics, and assurance cases.

## OPTIONS

`--json` — machine-readable assurance report

## EXAMPLES

```bash
spanda assure robot.sd --json
```

## EXIT STATUS

0 when assurance checks pass; 1 on gaps or violations.

## FILES

Assurance metadata in program declarations.

## SEE ALSO

spanda-readiness(1), spanda-diagnose(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
