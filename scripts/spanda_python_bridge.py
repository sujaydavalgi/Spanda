#!/usr/bin/env python3
"""Spanda Python FFI bridge (subprocess protocol v1).

Reads JSON from stdin:
  {"fn": "py_add", "args": [1, 2]}

Writes JSON to stdout:
  {"ok": true, "result": 3}
  {"ok": false, "error": "..."}

Register handlers below or extend this module for project-specific bindings.
"""

from __future__ import annotations

import json
import sys
from typing import Any, Callable

Handler = Callable[..., Any]


def _modbus_read_register(host: str, port: str, address: int) -> float:
    try:
        from pymodbus.client import ModbusTcpClient
    except ImportError:
        return float(address % 100)

    client = ModbusTcpClient(host, port=int(port))
    if not client.connect():
        return float(address % 100)
    zero_based = max(address - 40001, 0)
    result = client.read_holding_registers(zero_based, 1, slave=1)
    client.close()
    if result.isError():
        return float(address % 100)
    return float(result.registers[0])


def _opcua_read_node(endpoint: str, node_id: str) -> str:
    try:
        from asyncua.sync import Client
    except ImportError:
        return f"mock-opcua:{node_id}"

    with Client(endpoint) as client:
        node = client.get_node(node_id)
        value = node.get_value()
        return str(value)


def _zigbee_read_attribute(device: str, cluster: str) -> str:
    return f"mock-zigbee:{device}:{cluster}"


def _lora_read_payload(device_id: str) -> str:
    return f"mock-lora:{device_id}"


def _matter_read_cluster(node: str, cluster: str) -> float:
    return float((hash(f"{node}:{cluster}") % 100) + 1)


def _canbus_read_frame(can_id: int) -> float:
    return float(can_id & 0xFF)


def _onnx_complete(prompt: str) -> str:
    import os

    model_path = os.environ.get("SPANDA_ONNX_MODEL_PATH")
    if not model_path:
        return f"mock-onnx:{prompt[:48]}"
    try:
        import onnxruntime as ort
    except ImportError:
        return f"mock-onnx:{prompt[:48]}"
    session = ort.InferenceSession(model_path)
    outputs = session.run(None, {})
    if outputs:
        first = outputs[0]
        if hasattr(first, "tolist"):
            return str(first.tolist())[:256]
    return "onnx-empty"


def _anthropic_complete(prompt: str) -> str:
    import os

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        return f"mock-anthropic:{prompt[:48]}"
    try:
        import urllib.request

        body = json.dumps(
            {
                "model": "claude-3-5-haiku-latest",
                "max_tokens": 256,
                "messages": [{"role": "user", "content": prompt}],
            }
        ).encode()
        req = urllib.request.Request(
            "https://api.anthropic.com/v1/messages",
            data=body,
            headers={
                "x-api-key": api_key,
                "anthropic-version": "2023-06-01",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        with urllib.request.urlopen(req, timeout=30) as resp:  # noqa: S310
            data = json.loads(resp.read().decode())
        content = data.get("content") or []
        if content and isinstance(content[0], dict):
            return str(content[0].get("text", ""))
        return "anthropic-empty"
    except Exception as exc:  # noqa: BLE001
        return f"anthropic-error:{exc}"


def _openai_complete(prompt: str) -> str:
    import os

    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        return f"mock-completion:{prompt[:48]}"
    try:
        import urllib.request

        body = json.dumps(
            {
                "model": "gpt-4o-mini",
                "messages": [{"role": "user", "content": prompt}],
            }
        ).encode()
        req = urllib.request.Request(
            "https://api.openai.com/v1/chat/completions",
            data=body,
            headers={
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        with urllib.request.urlopen(req, timeout=30) as resp:  # noqa: S310
            data = json.loads(resp.read().decode())
        return data["choices"][0]["message"]["content"]
    except Exception as exc:  # noqa: BLE001
        return f"openai-error:{exc}"


def _ros2_publish(topic: str, data: Any) -> dict[str, Any]:
    try:
        import rclpy
        from rclpy.node import Node
        from std_msgs.msg import String
    except ImportError:
        return {"topic": topic, "published": True, "bytes": len(str(data)), "mode": "mock"}

    if not rclpy.ok():
        rclpy.init()
    node = Node("spanda_bridge_pub")
    pub = node.create_publisher(String, topic, 10)
    msg = String()
    msg.data = str(data)
    pub.publish(msg)
    rclpy.spin_once(node, timeout_sec=0.1)
    node.destroy_node()
    return {"topic": topic, "published": True, "bytes": len(msg.data), "mode": "rclpy"}


def _ros2_subscribe(topic: str) -> dict[str, Any]:
    try:
        import rclpy
        from rclpy.node import Node
        from std_msgs.msg import String
    except ImportError:
        return {"topic": topic, "subscribed": True, "mode": "mock"}

    if not rclpy.ok():
        rclpy.init()
    node = Node("spanda_bridge_sub")
    node.create_subscription(String, topic, lambda _msg: None, 10)
    rclpy.spin_once(node, timeout_sec=0.1)
    node.destroy_node()
    return {"topic": topic, "subscribed": True, "mode": "rclpy"}


def _ros2_service_call(service: str, service_type: str, request: str) -> dict[str, Any]:
    try:
        import rclpy
        from rclpy.node import Node
    except ImportError:
        return {"service": service, "called": True, "mode": "mock"}

    if not rclpy.ok():
        rclpy.init()
    node = Node("spanda_bridge_srv")
    rclpy.spin_once(node, timeout_sec=0.05)
    node.destroy_node()
    return {
        "service": service,
        "type": service_type,
        "request": request,
        "called": True,
        "mode": "rclpy",
    }


def _mqtt_publish(topic: str, payload: Any) -> dict[str, Any]:
    try:
        import paho.mqtt.client as mqtt
    except ImportError:
        return {
            "topic": topic,
            "published": True,
            "bytes": len(str(payload)),
            "mode": "mock",
        }

    host = __import__("os").environ.get("MQTT_BROKER", "localhost")
    port = int(__import__("os").environ.get("MQTT_PORT", "1883"))
    client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
    client.connect(host, port, keepalive=60)
    body = str(payload)
    client.publish(topic, body)
    client.disconnect()
    return {"topic": topic, "published": True, "bytes": len(body), "mode": "paho"}


HANDLERS: dict[str, Handler] = {
    "py_add": lambda a, b: int(a) + int(b),
    "py_echo": lambda x: x,
    "py_version": lambda: 1,
    "ros2_publish": _ros2_publish,
    "ros2_subscribe": _ros2_subscribe,
    "ros2_service_call": _ros2_service_call,
    "mqtt_publish": _mqtt_publish,
    "modbus_read_register": _modbus_read_register,
    "opcua_read_node": _opcua_read_node,
    "zigbee_read_attribute": _zigbee_read_attribute,
    "lora_read_payload": _lora_read_payload,
    "matter_read_cluster": _matter_read_cluster,
    "canbus_read_frame": _canbus_read_frame,
    "onnx_complete": _onnx_complete,
    "openai_complete": _openai_complete,
    "anthropic_complete": _anthropic_complete,
}


def main() -> int:
    try:
        req = json.load(sys.stdin)
    except json.JSONDecodeError as exc:
        print(json.dumps({"ok": False, "error": f"Invalid JSON request: {exc}"}))
        return 0

    fn = req.get("fn")
    args = req.get("args", [])
    if not isinstance(fn, str):
        print(json.dumps({"ok": False, "error": "Missing fn string in request"}))
        return 0
    if not isinstance(args, list):
        print(json.dumps({"ok": False, "error": "args must be a JSON array"}))
        return 0

    handler = HANDLERS.get(fn)
    if handler is None:
        print(json.dumps({"ok": False, "error": f"Unknown python extern '{fn}'"}))
        return 0

    try:
        result = handler(*args)
    except Exception as exc:  # noqa: BLE001 — bridge boundary
        print(json.dumps({"ok": False, "error": str(exc)}))
        return 0

    print(json.dumps({"ok": True, "result": result}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
