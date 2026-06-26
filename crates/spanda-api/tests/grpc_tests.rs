//! gRPC server smoke tests for Control Center.
use spanda_api::grpc::spanda_v1::control_center_client::ControlCenterClient;
use spanda_api::{run_control_center_server, ControlCenterOptions};
use std::thread;
use std::time::Duration;
use tonic::transport::Channel;

#[tokio::test]
async fn grpc_health_and_dashboard() {
    let http_bind = "127.0.0.1:18081";
    let grpc_bind = "127.0.0.1:50051";
    let options = ControlCenterOptions {
        bind: http_bind.into(),
        grpc_bind: Some(grpc_bind.into()),
        once: true,
        timeout_ms: 500,
        ..Default::default()
    };
    thread::spawn(move || {
        let _ = run_control_center_server(&options);
    });
    thread::sleep(Duration::from_millis(400));
    let channel = Channel::from_shared(format!("http://{grpc_bind}"))
        .unwrap()
        .connect()
        .await
        .expect("grpc connect");
    let mut client = ControlCenterClient::new(channel);
    let health = client
        .health(spanda_api::grpc::spanda_v1::Empty {})
        .await
        .expect("health rpc")
        .into_inner();
    assert_eq!(health.status, "ok");
    let dashboard = client
        .get_dashboard(spanda_api::grpc::spanda_v1::Empty {})
        .await
        .expect("dashboard rpc")
        .into_inner();
    assert!(dashboard.json.contains("device_pool"));
}
