//! Integration tests for optional ML spoofing backends.

use spanda_deploy_http::{serve_once, HttpResponse};
use spanda_spoofing::{generate_trace_spoof_check, MissionTrace, TraceFrame};
use std::net::TcpListener;
use std::thread;

fn spoof_trace() -> MissionTrace {
    MissionTrace {
        version: 1,
        source: "spoof.trace".into(),
        deterministic: true,
        frames: vec![
            TraceFrame {
                sim_time_ms: 0.0,
                event: "gps_reading".into(),
                payload: serde_json::json!({"lat": 37.7749, "lon": -122.4194}),
            },
            TraceFrame {
                sim_time_ms: 500.0,
                event: "emit gps.spoofed".into(),
                payload: serde_json::json!({"reason": "imu_disagreement"}),
            },
        ],
    }
}

#[test]
fn http_ml_endpoint_merges_alerts_into_trace_report() {
    let _guard = spanda_spoofing::testing::env_lock();
    std::env::remove_var("SPANDA_SPOOFING_ML_BACKEND");
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let server = thread::spawn(move || {
        serve_once(&listener, |_request| HttpResponse {
            status: 200,
            body: r#"{"alerts":[{"sensor":"gps","severity":"critical","confidence":0.99,"message":"ml endpoint spoof","evidence":"endpoint-model"}]}"#
                .into(),
        })
        .expect("serve once");
    });
    std::env::set_var(
        "SPANDA_SPOOFING_ML_ENDPOINT",
        format!("http://127.0.0.1:{port}/spoof"),
    );
    std::env::remove_var("SPANDA_SPOOFING_ML_BACKEND");

    let report = generate_trace_spoof_check(&spoof_trace(), "spoof.trace");
    assert_eq!(report.ml_alerts_merged, 1);
    assert!(report
        .alerts
        .iter()
        .any(|alert| alert.evidence.starts_with("ml:")));

    std::env::remove_var("SPANDA_SPOOFING_ML_ENDPOINT");
    server.join().expect("server thread");
}

#[test]
fn mock_ml_backend_merges_alerts_into_trace_report() {
    let _guard = spanda_spoofing::testing::env_lock();
    std::env::remove_var("SPANDA_SPOOFING_ML_ENDPOINT");
    std::env::set_var("SPANDA_SPOOFING_ML_BACKEND", "mock");

    let report = generate_trace_spoof_check(&spoof_trace(), "spoof.trace");
    assert_eq!(report.ml_alerts_merged, 1);
    assert!(!report.passed);

    std::env::remove_var("SPANDA_SPOOFING_ML_BACKEND");
}
