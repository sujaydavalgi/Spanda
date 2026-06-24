# Knowledge Models

Knowledge models capture **system structure** and **capability dependencies** for mission assurance.

## Syntax

```spanda
knowledge_model RoverModel {
    component gps;
    component camera;
    component wheels;
    dependency navigation requires [gps, wheels];
    dependency obstacle_avoidance requires [camera, lidar];
}
```

## Core types

| Type | Role |
|------|------|
| `SystemModel` | Named model with components and dependencies |
| `ComponentModel` | Hardware or software component |
| `DependencyGraph` | Capability → required components |
| `MissionKnowledgeBase` | Collection of system models |
| `CapabilityOntology` | Capability requirement mapping |

## Analysis

Static analysis in `spanda-assurance`:

- Extracts components and dependency edges
- Validates non-empty components and dependency lists
- Warns when robots exist without a knowledge model

## Package

Optional runtime scaffolds: **`spanda-knowledge-model`** (`assurance.knowledge`).

## Example

See `examples/assurance/rover_assurance.sd`.
