//! In-process ROS2 via dynamically loaded native rclrs, rclpy daemon, or Python bridge.
//!
//! Priority when `SPANDA_ROS2_RCLRS=1`:
//! 1. Native `rclrs` shared library (`libspanda_ros2_rclrs_native`, built with sourced ROS 2)
//! 2. Persistent rclpy daemon subprocess
//! 3. Per-call Python bridge fallback

use crate::runtime::RuntimeValue;
use crate::transport_live::{
    try_ros2_bridge_publish, try_ros2_bridge_service_call, try_ros2_bridge_subscribe,
};
use crate::transport_rclrs_daemon::{daemon_publish, daemon_service_call, daemon_subscribe};
use crate::transport_rclrs_native as native;

pub fn rclrs_enabled() -> bool {
    // Rclrs enabled.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::rclrs_enabled();

    // Produce is ok as the result.
    std::env::var("SPANDA_ROS2_RCLRS").is_ok()
}

pub fn rclrs_available() -> bool {
    // Rclrs available.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::rclrs_available();

    // Produce rclrs enabled as the result.
    rclrs_enabled()
}

fn payload_string(value: &RuntimeValue) -> String {
    // Payload string.
    //
    // Parameters:
    // - `value` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::payload_string(value);

    // Match on value and handle each case.
    match value {
        RuntimeValue::String { value } => value.clone(),
        RuntimeValue::Number { value, .. } => value.to_string(),
        RuntimeValue::Bool { value } => value.to_string(),
        other => format!("{other:?}"),
    }
}

pub fn try_rclrs_publish(topic: &str, value: &RuntimeValue) -> bool {
    // Try rclrs publish.
    //
    // Parameters:
    // - `topic` — input value
    // - `value` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::try_rclrs_publish(topic, value);

    // take the branch when rclrs enabled is false.
    if !rclrs_enabled() {
        return false;
    }

    // Take this path when native::publish(topic, &payload string(value)).
    if native::publish(topic, &payload_string(value)) {
        return true;
    }

    // Take this path when daemon publish(topic, value).
    if daemon_publish(topic, value) {
        return true;
    }
    try_ros2_bridge_publish(topic, value)
}

pub fn try_rclrs_subscribe(topic: &str) -> bool {
    // Try rclrs subscribe.
    //
    // Parameters:
    // - `topic` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::try_rclrs_subscribe(topic);

    // take the branch when rclrs enabled is false.
    if !rclrs_enabled() {
        return false;
    }

    // Take this path when native::subscribe(topic).
    if native::subscribe(topic) {
        return true;
    }

    // Take this path when daemon subscribe(topic).
    if daemon_subscribe(topic) {
        return true;
    }
    try_ros2_bridge_subscribe(topic)
}

pub fn try_rclrs_service_call(service: &str, service_type: &str, request: &str) -> bool {
    // Try rclrs service call.
    //
    // Parameters:
    // - `service` — input value
    // - `service_type` — input value
    // - `request` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::try_rclrs_service_call(service, service_type, request);

    // take the branch when rclrs enabled is false.
    if !rclrs_enabled() {
        return false;
    }

    // Take this path when native::service call(service, service type, request).
    if native::service_call(service, service_type, request) {
        return true;
    }

    // Take this path when daemon service call(service, service type, request).
    if daemon_service_call(service, service_type, request) {
        return true;
    }
    try_ros2_bridge_service_call(service, service_type, request)
}

pub fn init_node(name: &str) -> Result<(), String> {
    // Init node.
    //
    // Parameters:
    // - `name` — input value
    //
    // Returns:
    // Success value on completion, or an error.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::init_node(name);

    // take this path when native::sdk available().
    if native::sdk_available() {
        return native::init_node(name);
    }

    // Take this path when daemon subscribe("/spanda/rclrs/init").
    if daemon_subscribe("/spanda/rclrs/init") {
        let _ = name;
        Ok(())
    } else {
        Err(
            "ROS2 rclrs SDK unavailable — build libspanda_ros2_rclrs_native and source ROS 2"
                .into(),
        )
    }
}

pub fn native_sdk_available() -> bool {
    // Native sdk available.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport_rclrs::native_sdk_available();

    // Produce sdk available as the result.
    native::sdk_available()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rclrs_off_by_default() {
        // Rclrs off by default.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::transport_rclrs::rclrs_off_by_default();

        std::env::remove_var("SPANDA_ROS2_RCLRS");
        assert!(!rclrs_enabled());
    }
}
