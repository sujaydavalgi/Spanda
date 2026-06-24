# spanda-test(1)

## NAME

test — Run in-language `test` blocks and package test suites for a Spanda project.

## SYNOPSIS

```
spanda test [--project <dir>]
```

## DESCRIPTION

Run in-language `test` blocks and package test suites for a Spanda project.

## OPTIONS

`--project` — project root (default: current directory)

## EXAMPLES

```bash
spanda test
spanda test --project examples/rover
```

## EXIT STATUS

0 when all tests pass; 1 on failures.

## FILES

`spanda.toml`, `spanda.lock`, project `.sd` sources.

## SEE ALSO

spanda-check(1), spanda-package(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
