# spanda-man(1)

## NAME

man — Display man-page style documentation for Spanda CLI commands.

## SYNOPSIS

```
spanda man [<command>] [--roff]
```

## DESCRIPTION

Display man-page style documentation for Spanda CLI commands.

## OPTIONS

`--roff` — emit roff for Unix `man` viewers
No argument — list available pages

## EXAMPLES

```bash
spanda man
spanda man verify
spanda man run --roff
```

## EXIT STATUS

0 on success; 1 when the command page is not found.

## FILES

Man pages are generated from compiler metadata into `docs/man/`.

## SEE ALSO

spanda-reference(1), spanda-doc(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
