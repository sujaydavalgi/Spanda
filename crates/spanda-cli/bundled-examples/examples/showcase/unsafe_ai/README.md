# Unsafe AI vs Safe AI

Demonstrates Spanda's core differentiator: **AI proposals cannot reach actuators without safety validation**.

## Unsafe (compile error)

```bash
spanda check examples/showcase/unsafe_ai/unsafe.sd
```

Expected: type error — `ActionProposal` cannot be passed to `actuator.execute()`.

## Safe (passes)

```bash
spanda check examples/showcase/unsafe_ai/safe.sd
spanda sim examples/showcase/unsafe_ai/safe.sd
```

Flow:

```
ActionProposal → safety.validate() → SafeAction → wheels.execute()
```

One command: `spanda demo safety`
