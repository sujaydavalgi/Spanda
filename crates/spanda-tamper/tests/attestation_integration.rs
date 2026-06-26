//! Integration test for live attestation HTTP endpoint merging.

use spanda_deploy_http::{serve_once, HttpResponse};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::{attestation_env_lock, clear_attestation_env, evaluate_secure_boot_coverage};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn repo_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    for part in parts {
        path.push(part);
    }
    path
}

const ROVER_SOURCE: &str = r#"
import trust.jetson;
hardware Jetson { sensors [ GPS ]; actuators [ DifferentialDrive ]; }
robot Rover {
  uses hardware Jetson;
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  behavior patrol() { wheels.drive(0.1 m/s); }
}
"#;

#[test]
fn live_attestation_endpoint_merges_secure_boot_coverage() {
    let _guard = attestation_env_lock();
    clear_attestation_env();

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let server = thread::spawn(move || {
        serve_once(&listener, |_request| HttpResponse {
            status: 200,
            body: r#"{"attested":true,"boot_state":"verified","score":100,"detail":"mock tpm"}"#
                .into(),
        })
        .expect("serve once");
    });
    thread::sleep(Duration::from_millis(50));
    std::env::set_var(
        "SPANDA_ATTESTATION_ENDPOINT",
        format!("http://127.0.0.1:{port}/attest"),
    );

    let program = parse(tokenize(ROVER_SOURCE).expect("tokenize")).expect("parse");
    let coverage = evaluate_secure_boot_coverage(&program, Some("rover.sd"));
    assert!(coverage.live_attested);
    assert_eq!(coverage.contracts.len(), 1);
    assert!(coverage.contracts[0].live_attestation.is_some());
    assert!(
        coverage.contracts[0]
            .live_attestation
            .as_ref()
            .unwrap()
            .attested
    );

    clear_attestation_env();
    server.join().expect("server thread");
}

#[test]
fn tpm_mock_backend_merges_secure_boot_coverage() {
    let _guard = attestation_env_lock();
    clear_attestation_env();
    std::env::set_var("SPANDA_TPM_BACKEND", "jetson");

    let program = parse(tokenize(ROVER_SOURCE).expect("tokenize")).expect("parse");
    let coverage = evaluate_secure_boot_coverage(&program, Some("rover.sd"));
    assert!(coverage.live_attested);
    assert!(coverage
        .contracts
        .first()
        .and_then(|entry| entry.live_attestation.as_ref())
        .is_some_and(|live| live.attested));

    clear_attestation_env();
}

#[test]
fn vendor_tpm_with_ak_chain_validates_against_trust_store() {
    let _guard = attestation_env_lock();
    clear_attestation_env();

    let trust_store = repo_path(&[
        "examples",
        "showcase",
        "secure_boot",
        "fixtures",
        "trust-store",
    ]);
    if !trust_store.is_dir() {
        return;
    }
    let vendor_script = repo_path(&[
        "examples",
        "showcase",
        "secure_boot",
        "fixtures",
        "vendor-ak-chain.sh",
    ]);
    if !vendor_script.is_file() {
        return;
    }

    std::env::set_var(
        "SPANDA_ATTESTATION_TRUST_STORE",
        trust_store.to_string_lossy().to_string(),
    );
    std::env::set_var("SPANDA_TPM_BACKEND", "vendor");
    std::env::set_var(
        "SPANDA_TPM_VENDOR_SDK",
        vendor_script.to_string_lossy().to_string(),
    );

    let program = parse(tokenize(ROVER_SOURCE).expect("tokenize")).expect("parse");
    let coverage = evaluate_secure_boot_coverage(&program, Some("rover.sd"));
    assert!(coverage.live_attested);
    let live = coverage
        .contracts
        .first()
        .and_then(|entry| entry.live_attestation.as_ref())
        .expect("live attestation");
    assert!(live.attested, "detail={}", live.detail);
    assert_eq!(live.ak_chain_verified, Some(true));

    clear_attestation_env();
}
