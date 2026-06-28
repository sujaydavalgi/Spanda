# Wearable Health Monitoring

Optional health integration for Connected Healthcare deployments. **Disabled by default.**

## Privacy

Set in `../spanda.security.toml`:

```toml
[security.human_health]
enabled = false  # set true only with consent policy
```

Enable at runtime:

```bash
export SPANDA_HUMAN_HEALTH_ENABLED=1
spanda readiness health_patrol.sd --profile human_collaboration --config ../spanda.toml
```

## Wearables

- `watch-001` — heart rate, connectivity (`spanda-smartwatch`)
- `vest-001` — fall detection (`spanda-industrial-wearables`)

## Demonstrates

- Fatigue alert → mission pause + audit
- `medical_responder` capability for healthcare worker role
- Privacy-first defaults (health off unless explicitly enabled)

## Docs

[human-readiness.md](../../../docs/human-readiness.md) · [wearables.md](../../../docs/wearables.md)
