//! Native tonic gRPC client — optional `grpc` feature on `spanda-sdk`.
//!
use crate::error::{SpandaError, SpandaResult};
use serde_json::Value;
use tonic::transport::Channel;

pub mod spanda_v1 {
    tonic::include_proto!("spanda.v1");
}

use spanda_v1::control_center_client::ControlCenterClient;
use spanda_v1::{EntityIdRequest, JsonBodyRequest};

/// Async gRPC client for Control Center (`spanda.v1.ControlCenter`).
pub struct GrpcClient {
    inner: ControlCenterClient<Channel>,
}

impl GrpcClient {
    /// Connect to a gRPC endpoint (for example `http://127.0.0.1:50051`).
    pub async fn connect(endpoint: impl Into<String>) -> SpandaResult<Self> {
        let channel = Channel::from_shared(endpoint.into())
            .map_err(|e| SpandaError::connection(e.to_string()))?
            .connect()
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Ok(Self {
            inner: ControlCenterClient::new(channel),
        })
    }

    /// Blocking connect helper for scripts without an async runtime.
    pub fn connect_blocking(endpoint: impl Into<String>) -> SpandaResult<Self> {
        tokio::runtime::Runtime::new()
            .map_err(|e| SpandaError::connection(e.to_string()))?
            .block_on(Self::connect(endpoint))
    }

    fn parse_json(raw: String) -> SpandaResult<Value> {
        serde_json::from_str(&raw).map_err(|e| SpandaError::validation(e.to_string()))
    }

    fn program_body(file: &str, extra: Value) -> String {
        let mut body = serde_json::json!({ "file": file });
        if let Some(obj) = body.as_object_mut() {
            if let Some(extra_obj) = extra.as_object() {
                for (key, value) in extra_obj {
                    obj.insert(key.clone(), value.clone());
                }
            }
        }
        body.to_string()
    }

    /// Evaluate program readiness via `EvaluateProgramReadiness`.
    pub async fn readiness(&mut self, file: &str) -> SpandaResult<Value> {
        let body = Self::program_body(file, Value::Null);
        let resp = self
            .inner
            .evaluate_program_readiness(JsonBodyRequest { body_json: body })
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }

    /// Evaluate program assurance via `EvaluateProgramAssure`.
    pub async fn assure(&mut self, file: &str) -> SpandaResult<Value> {
        let body = Self::program_body(file, Value::Null);
        let resp = self
            .inner
            .evaluate_program_assure(JsonBodyRequest { body_json: body })
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }

    /// Run program simulation via `RunProgramSimulation`.
    pub async fn run_simulation(&mut self, file: &str, execute: bool) -> SpandaResult<Value> {
        let body = Self::program_body(file, serde_json::json!({ "execute": execute }));
        let resp = self
            .inner
            .run_program_simulation(JsonBodyRequest { body_json: body })
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }

    /// Replay or inspect a mission trace via `ReplayProgram`.
    pub async fn replay(
        &mut self,
        file: &str,
        deterministic: bool,
        playback: bool,
    ) -> SpandaResult<Value> {
        let body = Self::program_body(
            file,
            serde_json::json!({
                "deterministic": deterministic,
                "playback": playback,
            }),
        );
        let resp = self
            .inner
            .replay_program(JsonBodyRequest { body_json: body })
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }

    /// List unified entities via `ListEntities`.
    pub async fn list_entities(&mut self) -> SpandaResult<Value> {
        let resp = self
            .inner
            .list_entities(spanda_v1::Empty {})
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }

    /// Fetch one entity via `GetEntity`.
    pub async fn get_entity(&mut self, entity_id: &str) -> SpandaResult<Value> {
        let resp = self
            .inner
            .get_entity(EntityIdRequest {
                entity_id: entity_id.to_string(),
            })
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }

    /// List devices via `ListDevices`.
    pub async fn list_devices(&mut self) -> SpandaResult<Value> {
        let resp = self
            .inner
            .list_devices(spanda_v1::Empty {})
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }
}
