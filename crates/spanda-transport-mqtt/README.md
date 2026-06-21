# spanda-transport-mqtt

MQTT transport backend for Spanda. Extracted from `spanda-core` as part of the lean-core architecture.

## Features

- `live` — enable rumqttc broker integration (`SPANDA_LIVE_MQTT=1` at runtime in Spanda CLI)

## Usage

Used by the `spanda-mqtt` official package. Core retains a compatibility shim in `transport_mqtt.rs`.
