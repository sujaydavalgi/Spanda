//! Integration test for live attestation HTTP endpoint merging.

use spanda_deploy_http::{serve_once, HttpResponse};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::evaluate_secure_boot_coverage;
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

fn repo_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    for part in parts {
        path.push(part);
    }
    path
}

#[test]
fn live_attestation_endpoint_merges_secure_boot_coverage() {
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
    std::env::set_var(
        "SPANDA_ATTESTATION_ENDPOINT",
        format!("http://127.0.0.1:{port}/attest"),
    );

    let source = r#"
import trust.jetson;
hardware Jetson { sensors [ GPS ]; actuators [ DifferentialDrive ]; }
robot Rover {
  uses hardware Jetson;
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  behavior patrol() { wheels.drive(0.1 m/s); }
}
"#;
    let program = parse(tokenize(source).expect("tokenize")).expect("parse");
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

    std::env::remove_var("SPANDA_ATTESTATION_ENDPOINT");
    server.join().expect("server thread");
}

#[test]
fn tpm_mock_backend_merges_secure_boot_coverage() {
    std::env::remove_var("SPANDA_ATTESTATION_ENDPOINT");
    std::env::set_var("SPANDA_TPM_BACKEND", "jetson");

    let source = r#"
import trust.jetson;
hardware Jetson { sensors [ GPS ]; actuators [ DifferentialDrive ]; }
robot Rover {
  uses hardware Jetson;
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  behavior patrol() { wheels.drive(0.1 m/s); }
}
"#;
    let program = parse(tokenize(source).expect("tokenize")).expect("parse");
    let coverage = evaluate_secure_boot_coverage(&program, Some("rover.sd"));
    assert!(coverage.live_attested);
    assert!(coverage
        .contracts
        .first()
        .and_then(|entry| entry.live_attestation.as_ref())
        .is_some_and(|live| live.attested));

    std::env::remove_var("SPANDA_TPM_BACKEND");
}

#[test]
fn vendor_tpm_with_ak_chain_validates_against_trust_store() {
    std::env::remove_var("SPANDA_ATTESTATION_ENDPOINT");
    std::env::remove_var("SPANDA_ATTESTATION_AK_CHAIN_OPTIONAL");
    std::env::remove_var("SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT");
    std::env::remove_var("SPANDA_ATTESTATION_OPENSSL_VERIFY");
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
    std::env::set_var(
        "SPANDA_ATTESTATION_TRUST_STORE",
        trust_store.to_string_lossy().to_string(),
    );
    let vendor_script = repo_path(&[
        "examples",
        "showcase",
        "secure_boot",
        "fixtures",
        "vendor-ak-chain.sh",
    ]);
    if !vendor_script.is_file() {
        std::env::remove_var("SPANDA_ATTESTATION_TRUST_STORE");
        return;
    }
    std::env::set_var("SPANDA_TPM_BACKEND", "vendor");
    std::env::set_var(
        "SPANDA_TPM_VENDOR_SDK",
        vendor_script.to_string_lossy().to_string(),
    );

    let source = r#"
import trust.jetson;
hardware Jetson { sensors [ GPS ]; actuators [ DifferentialDrive ]; }
robot Rover {
  uses hardware Jetson;
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  behavior patrol() { wheels.drive(0.1 m/s); }
}
"#;
    let program = parse(tokenize(source).expect("tokenize")).expect("parse");
    let coverage = evaluate_secure_boot_coverage(&program, Some("rover.sd"));
    assert!(coverage.live_attested);
    let live = coverage
        .contracts
        .first()
        .and_then(|entry| entry.live_attestation.as_ref())
        .expect("live attestation");
    assert!(live.attested, "detail={}", live.detail);
    assert_eq!(live.ak_chain_verified, Some(true));

    std::env::remove_var("SPANDA_TPM_BACKEND");
    std::env::remove_var("SPANDA_TPM_VENDOR_SDK");
    std::env::remove_var("SPANDA_ATTESTATION_TRUST_STORE");
}
