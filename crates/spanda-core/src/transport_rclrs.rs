//! In-process ROS2 via persistent rclpy daemon when `SPANDA_ROS2_RCLRS=1`.
//!
//! Falls back to per-call Python bridge when the daemon cannot start.
//! Native `rclrs` crate linking remains optional behind `ros2-rclrs`.

use crate::runtime::RuntimeValue;
use crate::transport_live::{
    try_ros2_bridge_publish, try_ros2_bridge_service_call, try_ros2_bridge_subscribe,
};
use crate::transport_rclrs_daemon::{daemon_publish, daemon_service_call, daemon_subscribe};

pub fn rclrs_enabled() -> bool {
    std::env::var("SPANDA_ROS2_RCLRS").is_ok()
}

pub fn rclrs_available() -> bool {
    rclrs_enabled()
}

pub fn try_rclrs_publish(topic: &str, value: &RuntimeValue) -> bool {
    if !rclrs_enabled() {
        return false;
    }
    if daemon_publish(topic, value) {
        return true;
    }
    try_ros2_bridge_publish(topic, value)
}

pub fn try_rclrs_subscribe(topic: &str) -> bool {
    if !rclrs_enabled() {
        return false;
    }
    if daemon_subscribe(topic) {
        return true;
    }
    try_ros2_bridge_subscribe(topic)
}

pub fn try_rclrs_service_call(service: &str, service_type: &str, request: &str) -> bool {
    if !rclrs_enabled() {
        return false;
    }
    if daemon_service_call(service, service_type, request) {
        return true;
    }
    try_ros2_bridge_service_call(service, service_type, request)
}

#[cfg(feature = "ros2-rclrs")]
pub fn init_node(name: &str) -> Result<(), String> {
    if daemon_subscribe("/spanda/rclrs/init") {
        let _ = name;
        Ok(())
    } else {
        Err("native rclrs feature enabled but daemon unavailable".into())
    }
}

#[cfg(not(feature = "ros2-rclrs"))]
pub fn init_node(_name: &str) -> Result<(), String> {
    Err("enable ros2-rclrs feature for native node init".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rclrs_off_by_default() {
        std::env::remove_var("SPANDA_ROS2_RCLRS");
        assert!(!rclrs_enabled());
    }
}
