# spanda-lint(1)

## NAME

lint — Run linter rules beyond parse/type checking.

## SYNOPSIS

```
spanda lint [--json] <file.sd>
```

## DESCRIPTION

Run linter rules beyond parse/type checking.

## OPTIONS

`--json` — structured lint report

## EXAMPLES

```bash
spanda lint robot.sd
```

## EXIT STATUS

0 when no lint issues; 1 when issues are found.

## FILES

Input `.sd` source file.

## SEE ALSO

spanda-check(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
