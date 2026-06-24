# spanda-doc(1)

## NAME

doc — Generate JavaDoc-style API docs for `.sd` modules (markdown or HTML).

## SYNOPSIS

```
spanda doc [--json] [--html] [--out <file>] <file.sd|dir/>
```

## DESCRIPTION

Generate JavaDoc-style API docs for `.sd` modules (markdown or HTML).

## OPTIONS

`--out` — write output file or directory
`--html` — emit HTML instead of markdown
`--json` — wrap output in JSON

## EXAMPLES

```bash
spanda doc module.sd --out module-api.md
spanda doc --html examples/
```

## EXIT STATUS

0 on success; 1 on lex/parse errors.

## FILES

Output docs under `--out` or stdout.

## SEE ALSO

spanda-reference(1), spanda-man(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
