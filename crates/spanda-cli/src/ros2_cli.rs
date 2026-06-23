//! ROS 2 environment validation for the Spanda CLI (`spanda ros2 check`).

use serde::Serialize;
use std::env;
use std::process::{Command, Stdio};

#[derive(Serialize)]
struct Ros2CheckItem {
    name: String,
    ok: bool,
    detail: String,
}

#[derive(Serialize)]
struct Ros2CheckResponse {
    ok: bool,
    items: Vec<Ros2CheckItem>,
    hint: String,
}

/// Validate ROS 2 prerequisites for live transport (`SPANDA_ROS2_LIVE=1`).
///
/// Parameters:
/// - `json` — emit JSON instead of human-readable lines
///
/// Returns:
/// Nothing; exits with status 1 when required checks fail.
///
/// Options:
/// None.
///
/// Example:
/// `spanda ros2 check`
pub fn ros2_dispatch(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let mut items = Vec::new();

    let distro = env::var("ROS_DISTRO").unwrap_or_else(|_| String::new());
    items.push(Ros2CheckItem {
        name: "ROS_DISTRO".into(),
        ok: !distro.is_empty(),
        detail: if distro.is_empty() {
            "not set — source /opt/ros/<distro>/setup.bash".into()
        } else {
            distro.clone()
        },
    });

    let rclpy_ok = Command::new("python3")
        .args(["-c", "import rclpy"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    items.push(Ros2CheckItem {
        name: "rclpy".into(),
        ok: rclpy_ok,
        detail: if rclpy_ok {
            "importable".into()
        } else {
            "missing — install ros-${ROS_DISTRO}-rclpy".into()
        },
    });

    let bridge = env::var("SPANDA_PYTHON_BRIDGE").unwrap_or_else(|_| {
        format!(
            "{}/scripts/spanda_python_bridge.py",
            env::current_dir().unwrap_or_else(|_| ".".into()).display()
        )
    });
    let bridge_ok = std::path::Path::new(&bridge).is_file();
    items.push(Ros2CheckItem {
        name: "python_bridge".into(),
        ok: bridge_ok,
        detail: bridge,
    });

    let live = env::var("SPANDA_ROS2_LIVE").ok().as_deref() == Some("1");
    items.push(Ros2CheckItem {
        name: "SPANDA_ROS2_LIVE".into(),
        ok: live,
        detail: if live {
            "enabled".into()
        } else {
            "unset — export SPANDA_ROS2_LIVE=1 for live topics".into()
        },
    });

    let ok = items.iter().take(2).all(|i| i.ok);
    let hint = "See docs/ros2-golden-path.md — use `spanda ros2 check` before `spanda run` with ROS topics.";
    let response = Ros2CheckResponse {
        ok,
        items,
        hint: hint.into(),
    };

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&response).unwrap_or_default()
        );
    } else {
        for item in &response.items {
            let mark = if item.ok { "✓" } else { "✗" };
            println!("{mark} {}: {}", item.name, item.detail);
        }
        if response.ok {
            println!("✓ ROS 2 live transport prerequisites look good");
        } else {
            println!("✗ Fix ROS_DISTRO and rclpy before enabling SPANDA_ROS2_LIVE=1");
        }
        println!("{}", response.hint);
    }

    if !ok {
        std::process::exit(1);
    }
}
