# spanda-heal(1)

## NAME

heal — Execute self-healing and recovery policies declared in the program.

## SYNOPSIS

```
spanda heal [--json] <file.sd>
```

## DESCRIPTION

Execute self-healing and recovery policies declared in the program.

## OPTIONS

`--json` — recovery report

## EXAMPLES

```bash
spanda heal robot.sd --json
```

## EXIT STATUS

0 when recovery succeeds; 1 on unrecoverable faults.

## FILES

Recovery policies in `.sd` source.

## SEE ALSO

spanda-diagnose(1), spanda-recovery(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
