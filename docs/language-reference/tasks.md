# Tasks

Periodic tasks run on a fixed interval inside a robot:

```sd
robot Rover {
  task heartbeat every 1000ms {
    // ...
  }
}
```

See [triggers.md](./triggers.md) and [concurrency.md](../concurrency.md).
