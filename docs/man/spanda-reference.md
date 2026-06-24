# spanda-reference(1)

## NAME

reference — Emit the full Spanda language reference and optional man pages.

## SYNOPSIS

```
spanda reference [--json] [--out <file.md>] [--man-dir <dir>]
```

## DESCRIPTION

Emit the full Spanda language reference and optional man pages.

## OPTIONS

`--out` — write reference markdown
`--man-dir` — write man pages
`--json` — wrap markdown in JSON

## EXAMPLES

```bash
spanda reference --out docs/spanda-reference.md --man-dir docs/man
```

## EXIT STATUS

0 on success.

## FILES

`docs/spanda-reference.md`, `docs/man/` when generating.

## SEE ALSO

spanda-doc(1), spanda-man(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
