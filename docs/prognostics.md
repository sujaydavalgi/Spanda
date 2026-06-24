# Prognostics

Prognostics declare **remaining useful life (RUL)** predictions and **degradation warnings**.

## Syntax

```spanda
prognostics BatteryPrognostics {
    predict battery.remaining_useful_life;
    warn_if remaining_useful_life < 30 min;
}
```

## Core types

| Type | Role |
|------|------|
| `PrognosticModel` | Named model with rules |
| `RemainingUsefulLife` | Component RUL estimate |
| `FailurePrediction` | Probability and horizon |
| `DegradationTrend` | Metric direction and rate |

## CLI

```bash
spanda prognostics rover.sd [--json]
```

## Package

Advanced degradation models: **`spanda-prognostics`** (`assurance.prognostics`).

Complements `spanda-maintenance` (predictive maintenance) without duplicating health metrics.

## Example

See `examples/prognostics/battery_degradation.sd`.
