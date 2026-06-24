# spanda-trace(1)

## NAME

trace — Record scheduler, task, trigger, and event traces from a program run.

## SYNOPSIS

```
spanda trace [--json] [--out <file>] <file.sd>
```

## DESCRIPTION

Record scheduler, task, trigger, and event traces from a program run.

## OPTIONS

`--out` — trace output path
`--json` — structured trace summary

## EXAMPLES

```bash
spanda trace robot.sd --out mission.trace
```

## EXIT STATUS

0 on success; 1 on runtime errors.

## FILES

Output trace file (`.trace`).

## SEE ALSO

spanda-replay(1), spanda-run(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
