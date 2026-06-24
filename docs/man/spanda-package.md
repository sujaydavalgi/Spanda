# spanda-package(1)

## NAME

package — Manage Spanda packages: manifests, dependencies, builds, and registry operations.

## SYNOPSIS

```
spanda <init|build|test|add|remove|install|publish|registry> [options]
```

## DESCRIPTION

Manage Spanda packages: manifests, dependencies, builds, and registry operations.

## OPTIONS

See `spanda init`, `build`, `test`, `add`, `remove`, `install`, `publish`, `registry search`, `registry info`.

## EXAMPLES

```bash
spanda init my-robot
spanda add std.robotics
spanda publish
```

## EXIT STATUS

0 on success; 1 on manifest, lockfile, or registry errors.

## FILES

`spanda.toml`, `spanda.lock`, `packages/` registry mirror.

## SEE ALSO

spanda-check(1), spanda-test(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
