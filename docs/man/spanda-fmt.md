# spanda-fmt(1)

## NAME

fmt — Format Spanda source to canonical style.

## SYNOPSIS

```
spanda fmt [--json] <file.sd>
```

## DESCRIPTION

Format Spanda source to canonical style.

## OPTIONS

`--json` — report whether the file changed

## EXAMPLES

```bash
spanda fmt examples/rover.sd
```

## EXIT STATUS

0 on success; 1 on parse errors.

## FILES

In-place `.sd` source file.

## SEE ALSO

spanda-check(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
