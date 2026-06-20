//! verify cli support for Spanda.
//!
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

fn spanda_bin() -> PathBuf {
    // Spanda bin.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // PathBuf.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_cli::verify_cli::spanda_bin();

    // Produce var os as the result.
    std::env::var_os("CARGO_BIN_EXE_spanda")
        .map(PathBuf::from)
        .expect("CARGO_BIN_EXE_spanda not set (run via cargo test -p spanda-cli)")
}

fn rover_deploy() -> PathBuf {
    // Rover deploy.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // PathBuf.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_cli::verify_cli::rover_deploy();

    // Produce sd") as the result.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hardware/rover_deploy.sd")
}

fn run_verify(args: &[&str]) -> (std::process::Output, PathBuf) {
    // Run verify.
    //
    // Parameters:
    // - `args` — input value
    //
    // Returns:
    // (std::process::Output, PathBuf).
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_cli::verify_cli::run_verify(args);

    // Compute file for the following logic.
    let file = rover_deploy();
    let mut cmd = Command::new(spanda_bin());
    cmd.arg("verify");

    // Apply each command-line argument.
    for arg in args {
        cmd.arg(arg);
    }
    cmd.arg(&file);
    (cmd.output().expect("failed to run spanda verify"), file)
}

#[test]
fn verify_rover_deploy_passes_by_default() {
    // Verify rover deploy passes by default.
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
    // let result = spanda_cli::verify_cli::verify_rover_deploy_passes_by_default();

    let (output, _) = run_verify(&[]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "expected success, stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("Hardware compatibility:"));
    assert!(stdout.contains("Deployment compatible"));
    assert!(stdout.contains("RoverV1"));
}

#[test]
fn verify_with_target_rover_v1() {
    // Verify with target rover v1.
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
    // let result = spanda_cli::verify_cli::verify_with_target_rover_v1();

    let (output, _) = run_verify(&["--target", "RoverV1"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Target: RoverV1"));
    assert!(stdout.contains("Deployment compatible"));
}

#[test]
fn verify_with_target_esp32_fails() {
    // Verify with target esp32 fails.
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
    // let result = spanda_cli::verify_cli::verify_with_target_esp32_fails();

    let (output, _) = run_verify(&["--target", "ESP32"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success());
    assert!(stdout.contains("Deployment incompatible"));
    assert!(stdout.contains("Camera") || stdout.contains("Lidar"));
}

#[test]
fn verify_all_targets_prints_matrix() {
    // Verify all targets prints matrix.
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
    // let result = spanda_cli::verify_cli::verify_all_targets_prints_matrix();

    let (output, _) = run_verify(&["--all-targets"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Compatibility Matrix"));
    assert!(stdout.contains("RoverProgram"));
    assert!(stdout.contains("RoverV1"));
    assert!(stdout.contains("ESP32"));
}

#[test]
fn verify_json_output_shape() {
    // Verify json output shape.
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
    // let result = spanda_cli::verify_cli::verify_json_output_shape();

    let (output, _) = run_verify(&["--json"]);
    assert!(output.status.success());
    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON stdout");
    assert_eq!(parsed["ok"], true);
    assert_eq!(parsed["compatible"], true);
    assert_eq!(parsed["target"], "RoverV1");
    assert!(parsed["items"].is_array());
    assert!(parsed["items"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| { item["category"].is_string() && item["severity"].is_string() }));
}

#[test]
fn verify_json_esp32_fails_with_exit_code() {
    // Verify json esp32 fails with exit code.
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
    // let result = spanda_cli::verify_cli::verify_json_esp32_fails_with_exit_code();

    let (output, _) = run_verify(&["--json", "--target", "ESP32"]);
    assert!(!output.status.success());
    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON stdout");
    assert_eq!(parsed["ok"], false);
    assert_eq!(parsed["compatible"], false);
    assert_eq!(parsed["target"], "ESP32");
    let items = parsed["items"].as_array().expect("items array");
    assert!(items.iter().any(|i| i["severity"] == "error"));
}

#[test]
fn verify_json_all_targets_includes_matrix() {
    // Verify json all targets includes matrix.
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
    // let result = spanda_cli::verify_cli::verify_json_all_targets_includes_matrix();

    let (output, _) = run_verify(&["--json", "--all-targets"]);
    assert!(output.status.success());
    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON stdout");
    let cells = parsed["matrix"]["cells"]
        .as_array()
        .expect("matrix.cells array");
    assert!(!cells.is_empty());
    assert!(cells.iter().any(|c| {
        c["robot"] == "RoverProgram" && c["target"] == "RoverV1" && c["compatible"] == true
    }));
    assert!(cells.iter().any(|c| {
        c["robot"] == "RoverProgram" && c["target"] == "ESP32" && c["compatible"] == false
    }));
}

#[test]
fn compatibility_alias_matches_verify() {
    // Compatibility alias matches verify.
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
    // let result = spanda_cli::verify_cli::compatibility_alias_matches_verify();

    let file = rover_deploy();
    let verify = Command::new(spanda_bin())
        .args(["verify", "--json", file.to_str().unwrap()])
        .output()
        .expect("verify");
    let compat = Command::new(spanda_bin())
        .args(["compatibility", "--json", file.to_str().unwrap()])
        .output()
        .expect("compatibility");
    assert_eq!(verify.stdout, compat.stdout);
    assert_eq!(verify.status, compat.status);
}
