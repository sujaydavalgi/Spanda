# spanda-sim(1)

## NAME

sim — Run a program in the built-in simulator with optional trace recording.

## SYNOPSIS

```
spanda sim [--json] [--replay] [--wall-clock] [--record] [--trace-*] <file.sd>
```

## DESCRIPTION

Run a program in the built-in simulator with optional trace recording.

## OPTIONS

`--replay` — replay mode
`--wall-clock` — real-time pacing
`--record` — mission trace output

## EXAMPLES

```bash
spanda sim examples/rover.sd --record
spanda sim robot.sd --wall-clock
```

## EXIT STATUS

0 on successful simulation; 1 on errors.

## FILES

Mission traces when using `--record`.

## SEE ALSO

spanda-run(1), spanda-replay(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
