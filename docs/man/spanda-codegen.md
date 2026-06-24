# spanda-codegen(1)

## NAME

codegen — Generate deployable artifacts from a Spanda program.

## SYNOPSIS

```
spanda codegen [--target native|wasm|esp32] [--out <file>] <file.sd>
```

## DESCRIPTION

Generate deployable artifacts from a Spanda program.

## OPTIONS

`--target` — output format

## EXAMPLES

```bash
spanda codegen --target wasm robot.sd --out robot.wasm
```

## EXIT STATUS

0 on success; 1 on codegen errors.

## FILES

Generated artifact at `--out`.

## SEE ALSO

spanda-deploy(1), spanda-compile-native(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
