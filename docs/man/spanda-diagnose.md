# spanda-diagnose(1)

## NAME

diagnose — Diagnose failures from static analysis and optional mission traces.

## SYNOPSIS

```
spanda diagnose [--json] <file.sd> [<mission.trace>]
```

## DESCRIPTION

Diagnose failures from static analysis and optional mission traces.

## OPTIONS

`--json` — structured diagnosis report

## EXAMPLES

```bash
spanda diagnose robot.sd
spanda diagnose robot.sd mission.trace
```

## EXIT STATUS

0 when diagnosis completes; 1 on errors.

## FILES

Optional mission trace input.

## SEE ALSO

spanda-assure(1), spanda-heal(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
