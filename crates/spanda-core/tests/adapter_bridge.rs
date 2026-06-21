//! Adapter subprocess bridge integration tests.

use spanda_core::{invoke_nav2_bridge, invoke_slam_bridge};

#[test]
fn nav2_bridge_invokes_configured_command() {
    let script = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/adapters/nav2_bridge.sh"
    );
    let template = format!("bash {script} {{goal}}");
    unsafe {
        std::env::set_var("SPANDA_NAV2_CMD", &template);
    }
    let output = invoke_nav2_bridge("DockA").expect("nav2 bridge output");
    assert!(output.contains("nav2-bridge"));
    assert!(output.contains("DockA"));
    unsafe {
        std::env::remove_var("SPANDA_NAV2_CMD");
    }
}

#[test]
fn slam_bridge_invokes_configured_command() {
    let script = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/adapters/slam_bridge.sh"
    );
    let template = format!("bash {script} {{op}}");
    unsafe {
        std::env::set_var("SPANDA_SLAM_CMD", &template);
    }
    let output = invoke_slam_bridge("localize").expect("slam bridge output");
    assert!(output.contains("slam-bridge"));
    assert!(output.contains("localize"));
    unsafe {
        std::env::remove_var("SPANDA_SLAM_CMD");
    }
}
