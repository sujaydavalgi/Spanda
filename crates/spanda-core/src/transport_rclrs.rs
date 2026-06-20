//! Future in-process ROS2 transport via `rclrs` (not linked yet).
//!
//! Enable with `SPANDA_ROS2_RCLRS=1` once the native crate is wired in CI.
//! Today all live ROS2 I/O uses the `ros2` CLI when `SPANDA_ROS2_NATIVE=1`.

pub fn rclrs_enabled() -> bool {
    std::env::var("SPANDA_ROS2_RCLRS").is_ok()
}

pub fn rclrs_available() -> bool {
    rclrs_enabled() && cfg!(feature = "ros2-rclrs")
}

#[cfg(feature = "ros2-rclrs")]
pub fn init_node(_name: &str) -> Result<(), String> {
    Err("rclrs feature stub — link rclrs in build.rs to enable".into())
}

#[cfg(not(feature = "ros2-rclrs"))]
pub fn init_node(_name: &str) -> Result<(), String> {
    Err("build without ros2-rclrs feature".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rclrs_off_by_default() {
        std::env::remove_var("SPANDA_ROS2_RCLRS");
        assert!(!rclrs_enabled());
        assert!(!rclrs_available());
    }
}
