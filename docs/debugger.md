# Debugger

Spanda provides CLI debugging and experimental DAP support via VS Code.

## CLI

```bash
spanda debug rover.sd
spanda debug rover.sd --break 42
```

## Capabilities

- Breakpoints, step over, step into, continue
- Variable inspection
- Task, trigger, provider, safety, and hardware verification state
- Debug events: trigger fired, task started/completed, deadline missed, safety validation failed, kill switch activated (`kill_switch_activated`), critical health (`health_critical`), message received, provider called

## VS Code

Launch configuration uses `spanda-dap` over stdio. See `editor/vscode/README.md`.

Status: **Experimental** — step-in/out partial; DAP hardened in core, full task/trigger/provider events via `--trace-*` flags.

See [Debugging guide](./debugging.md) for details.
