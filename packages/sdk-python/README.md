# Spanda Python SDK

Thin HTTP client for the [Control Center](../../docs/control-center.md) REST API v1.

```bash
pip install -e packages/sdk-python
export SPANDA_API_KEY=your-key
python -c "from spanda_sdk import ControlCenterClient; c=ControlCenterClient(); print(c.health())"
```

See `tests/test_integration.py` for smoke-style usage against a running `spanda control-center serve`.
