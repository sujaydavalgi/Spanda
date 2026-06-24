# spanda-sim(1)

## NAME

sim — Run a program in the built-in simulator with optional trace recording.

## SYNOPSIS

```
spanda sim [--json] [--replay] [--wall-clock] [--record] [--inject-failure <kind>] [--trace-*] <file.sd>
```

## DESCRIPTION

Run a program in the built-in simulator with optional trace recording. With `--inject-failure`, simulates a sensor or subsystem failure and runs recovery planning in the simulation path.

## OPTIONS

`--inject-failure <kind>` — inject failure and evaluate recovery (e.g. `gps`, `lidar`, `fleet`)

`--replay` — replay mode
`--wall-clock` — real-time pacing
`--record` — mission trace output

## EXAMPLES

```bash
spanda sim examples/rover.sd --record
spanda sim examples/showcase/self_healing/rover.sd --inject-failure gps
spanda sim robot.sd --wall-clock
```

## SEE ALSO

spanda-run(1), spanda-replay(1), [spanda-recovery(1)](./spanda-recovery.md), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
