# Agents

Agents declare autonomous goals inside a `robot` block:

```sd
robot Rover {
  agent navigator {
    goal: "reach charging dock safely";
  }
}
```

See [concurrency.md](../concurrency.md) and [triggers.md](./triggers.md).
