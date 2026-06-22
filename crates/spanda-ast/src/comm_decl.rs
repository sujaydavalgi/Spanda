//! Communication declaration types shared by the Spanda AST.
//!
use crate::foundations::FieldDecl;
use crate::nodes::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Local,
    Ros2,
    Mqtt,
    Dds,
    Websocket,
    Sim,
}

impl TransportKind {
    pub fn from_ident(s: &str) -> Option<Self> {
        // Construct from ident.
        //
        // Parameters:
        // - `s` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::comm::from_ident(s);

        // Match on s and handle each case.
        match s {
            "local" => Some(Self::Local),
            "ros2" => Some(Self::Ros2),
            "mqtt" => Some(Self::Mqtt),
            "dds" => Some(Self::Dds),
            "websocket" => Some(Self::Websocket),
            "sim" => Some(Self::Sim),
            "ble" | "bluetooth" => Some(Self::Mqtt),
            "wifi" => Some(Self::Local),
            "cellular" => Some(Self::Mqtt),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Text result.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.as_str();

        // Dispatch based on the enum variant or current state.
        match self {
            Self::Local => "local",
            Self::Ros2 => "ros2",
            Self::Mqtt => "mqtt",
            Self::Dds => "dds",
            Self::Websocket => "websocket",
            Self::Sim => "sim",
        }
    }

    pub fn supports_encryption(self) -> bool {
        !matches!(self, Self::Local | Self::Sim)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QosReliability {
    Reliable,
    BestEffort,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QosDecl {
    pub reliability: Option<QosReliability>,
    pub rate_hz: Option<f64>,
    pub deadline_ms: Option<f64>,
    pub history: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TopicRole {
    #[default]
    Publish,
    Subscribe,
    Both,
}

// ── Message schema registry ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum MessageDecl {
    MessageDecl {
        name: String,
        fields: Vec<FieldDecl>,
        version: Option<u32>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageSchema {
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub version: Option<u32>,
}
// ── Communication declarations ─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum BusDecl {
    BusDecl {
        name: String,
        transport: TransportKind,
        transport_name: Option<String>,
        broker_url: Option<String>,
        encryption: Option<String>,
        authentication: Option<String>,
        integrity: Option<String>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum PeerRobotDecl {
    PeerRobotDecl { name: String, span: Span },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DeviceDecl {
    DeviceDecl {
        name: String,
        device_type: String,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum AgentChannelDecl {
    AgentChannelDecl {
        from_agent: String,
        to_agent: String,
        message_type: String,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum TwinSyncDecl {
    TwinSyncDecl {
        telemetry: bool,
        replay: bool,
        faults: bool,
        events: bool,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DiscoverTarget {
    Robots,
    Agents,
    Devices,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoverFilter {
    pub capability: Option<String>,
}

// ── Enhanced service/action shapes (used by parser) ──────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedServiceFields {
    pub request_type: String,
    pub response_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedActionFields {
    pub request_type: String,
    pub feedback_type: String,
    pub result_type: String,
}
