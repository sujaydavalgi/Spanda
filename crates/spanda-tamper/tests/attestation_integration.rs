//! Integration test for live attestation HTTP endpoint merging.

use spanda_deploy_http::{serve_once, HttpResponse};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::evaluate_secure_boot_coverage;
use std::net::TcpListener;
use std::thread;

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
    assert!(coverage.contracts[0].live_attestation.as_ref().unwrap().attested);

    std::env::remove_var("SPANDA_ATTESTATION_ENDPOINT");
    server.join().expect("server thread");
}
