# Spanda Language Reference

Structured reference for the Spanda (`.sd`) language. For a single generated document see [spanda-reference.md](../spanda-reference.md).

## Topics

| Topic | Guide |
|-------|-------|
| [Syntax](./syntax.md) | Lexical structure, modules, declarations |
| [Types](./types.md) | Built-in and user-defined types |
| [Functions](./functions.md) | Module functions, generics, contracts |
| [Agents](./agents.md) | Agent blocks and goals |
| [Tasks](./tasks.md) | Periodic tasks, triggers, scheduling |
| [Triggers](./triggers.md) | `on`, `every`, `when`, events |
| [Safety](./safety.md) | Safety zones, contracts, kill switches |
| [Hardware](./hardware.md) | Profiles, HAL, deploy targets |
| [Capabilities](./capabilities.md) | Robot and hardware capabilities |
| [Health checks](./health-checks.md) | Health policies and readiness |
| [Packages](./packages.md) | `spanda.toml`, dependencies, registry |
| [Recovery](./recovery.md) | Heal, recover, resilience policies |
| [Examples](./examples.md) | Runnable demos and golden paths |

## API documentation

- **Program API** — `spanda doc <file.sd>` or `spanda doc --html examples/`
- **Language API** — [spanda-reference.md](../spanda-reference.md) (keywords, stdlib, builtins)
- **Rust crates** — `cargo doc --workspace --no-deps`

## CLI manual pages

Man-page style CLI docs live under [../man/](../man/). View with `spanda man <command>`.
