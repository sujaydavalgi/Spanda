# spanda-alert-pagerduty

PagerDuty Events API v2 integration for Spanda Control Center alerting.

## Status

**Experimental** — core dispatch uses `spanda-ops` PagerDuty payload formatting.

## Configuration

```bash
export SPANDA_ALERT_PAGERDUTY_URL="https://events.pagerduty.com/v2/enqueue"
export SPANDA_ALERT_PAGERDUTY_ROUTING_KEY="your-routing-key"
```
