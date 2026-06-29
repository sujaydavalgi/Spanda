# Python SDK (`spanda-sdk`)

Official Python client for robotics scripts, notebooks, CI/CD, and ROS2 integrations.

## Install

From PyPI (released packages):

```bash
pip install spanda-sdk
pip install "spanda-sdk[stream]"   # WebSocket telemetry extra
```

From this monorepo (development):

```bash
pip install -e sdk/python
# stream extras for WebSocket telemetry
pip install -e "sdk/python[stream]"
```

Maintainers: see [Publishing SDKs](sdk-publishing.md) for PyPI tokens, GitHub secrets, and release tags.

## Usage

```python
from spanda import SpandaClient

client = SpandaClient.local()
report = client.readiness("rover.sd")
score = report.get("report", {}).get("score", {})
print(score.get("total") if isinstance(score, dict) else score)
```

Alternative import:

```python
from spanda_sdk import SpandaClient
```

## Environment variables

| Variable | Purpose |
|----------|---------|
| `SPANDA_CONTROL_CENTER_URL` | Base URL (default `http://127.0.0.1:8080`) |
| `SPANDA_API_KEY` | Bearer token for authenticated endpoints |

## Entity model

```python
client = SpandaClient.local()
graph = client.entity_graph()
trace = client.entity_traceability(entity_id="rover-001")
result = client.query_entities({"kind": "robot"})
client.register_entity({"id": "bay-1", "entity_type": "calibration_station"})
client.tag_entity("bay-1", {"add": ["production"]})
client.relate_entities({"from_id": "rover-001", "to_id": "gps-001", "kind": "depends_on"})
client.sync_entities()
```

Mutation endpoints require `SPANDA_API_KEY`.

## Event stream

```python
from spanda_sdk import TelemetryStream

def on_event(event):
    print(event.get("type"), event)

TelemetryStream().listen(on_event)  # requires [stream] extra
```

## Error handling

```python
from spanda_sdk.errors import ConnectionError, PermissionError

try:
    client.list_devices()
except PermissionError as exc:
    print("Set SPANDA_API_KEY", exc.status)
except ConnectionError:
    print("Start Control Center first")
```

## Examples

```bash
python examples/sdk/python/readiness.py
python examples/sdk/python/robot_health.py
```

## Tests

```bash
python -m pytest sdk/python
```

## Legacy client

`packages/sdk-python` provides `ControlCenterClient` with enterprise ops helpers (drift, OTA, SRE). New integrations should use `SpandaClient` from `sdk/python`.
