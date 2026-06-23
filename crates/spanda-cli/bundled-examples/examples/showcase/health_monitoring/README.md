# Health monitoring

Demonstrates robot and sensor health checks, health policies, and runtime fault injection.

## Healthy baseline

```bash
spanda check examples/showcase/health_monitoring/rover.sd
spanda health robot examples/showcase/health_monitoring/rover.sd --json
spanda sim examples/showcase/health_monitoring/rover.sd
```

## Inject faults — Degraded → Critical → Failed

```bash
spanda sim examples/showcase/health_monitoring/rover.sd --inject-health-faults
```

Policies react to **Degraded**, **Critical**, **Failed**, and **Unsafe** states.

One command: `spanda demo health`

Docs: [health-checks.md](../../docs/health-checks.md)
