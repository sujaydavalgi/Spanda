# Syntax

Spanda source files use the `.sd` extension.

## Modules

```sd
module my.robot;

import std.robotics;
```

## Doc comments

JavaDoc-style `///` comments attach to the next declaration:

```sd
/// Plans a safe path between two poses.
export fn plan_path(start: Pose, goal: Pose) -> Path {
  return trajectory(from: start, to: goal, steps: 3);
}
```

## Top-level declarations

Programs may declare functions, structs, enums, traits, robots, hardware profiles, safety policies, and operational metadata. See [spanda-language.md](../spanda-language.md) for a tutorial walkthrough.
