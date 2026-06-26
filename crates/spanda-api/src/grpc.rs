//! Native gRPC server (tonic) for Control Center CLI parity.
//!
use crate::state::SharedState;
use tonic::{transport::Server, Request, Response, Status};

pub mod spanda_v1 {
    tonic::include_proto!("spanda.v1");
}

use spanda_v1::control_center_server::{ControlCenter, ControlCenterServer};
use spanda_v1::{DriftRequest, Empty, HealthResponse, JsonResponse};

struct GrpcControlCenter {
    state: SharedState,
}

#[tonic::async_trait]
impl ControlCenter for GrpcControlCenter {
    async fn health(&self, _request: Request<Empty>) -> Result<Response<HealthResponse>, Status> {
        Ok(Response::new(HealthResponse {
            status: "ok".into(),
        }))
    }

    async fn get_dashboard(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<JsonResponse>, Status> {
        let guard = self.state.lock().map_err(|e| Status::internal(e.to_string()))?;
        let registry = guard.device_registry();
        let fleet = spanda_fleet::load_fleet_agent_registry(&spanda_fleet::default_fleet_agents_path());
        let json = serde_json::json!({
            "version": "v1",
            "device_pool": registry.pool_summary(),
            "fleet_agent_count": fleet.agents.len(),
            "alert_count": guard.alert_store.list().len(),
        });
        Ok(Response::new(JsonResponse {
            json: serde_json::to_string(&json).map_err(|e| Status::internal(e.to_string()))?,
        }))
    }

    async fn detect_drift(
        &self,
        request: Request<DriftRequest>,
    ) -> Result<Response<JsonResponse>, Status> {
        let baseline_id = request.into_inner().baseline_id;
        let guard = self.state.lock().map_err(|e| Status::internal(e.to_string()))?;
        let query = format!("baseline_id={baseline_id}");
        let http = crate::e3::drift_report(&guard, &query);
        let json = http.body;
        Ok(Response::new(JsonResponse { json }))
    }
}

/// Start tonic gRPC server on `bind` (blocks the current thread's tokio runtime).
pub async fn serve_grpc(bind: String, state: SharedState) -> Result<(), String> {
    // Serve ControlCenter gRPC on a dedicated listener.
    //
    // Parameters:
    // - `bind` — socket address (for example `127.0.0.1:50051`)
    // - `state` — shared Control Center state
    //
    // Returns:
    // Ok when the server stops, or an error.
    //
    // Options:
    // None.
    //
    // Example:
    // serve_grpc("127.0.0.1:50051".into(), state).await?;

    let service = GrpcControlCenter { state };
    Server::builder()
        .add_service(ControlCenterServer::new(service))
        .serve(bind.parse().map_err(|e: std::net::AddrParseError| e.to_string())?)
        .await
        .map_err(|e| e.to_string())
}

/// Spawn gRPC server on a background thread with its own tokio runtime.
pub fn spawn_grpc_server(bind: String, state: SharedState) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("grpc tokio runtime");
        if let Err(error) = runtime.block_on(serve_grpc(bind.clone(), state)) {
            eprintln!("gRPC server on {bind} stopped: {error}");
        }
    });
}
