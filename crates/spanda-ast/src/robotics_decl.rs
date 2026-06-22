//! Program-level robotics platform declaration types for the Spanda AST.
//!
use crate::nodes::Span;
use serde::{Deserialize, Serialize};

/// Known safety certification standards referenced by program metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CertificationStandard {
    Iso13849,
    Iec61508,
    Iso26262,
}

impl CertificationStandard {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iso13849 => "ISO13849",
            Self::Iec61508 => "IEC61508",
            Self::Iso26262 => "ISO26262",
        }
    }

    pub fn parse_ident(name: &str) -> Option<Self> {
        match name {
            "ISO13849" => Some(Self::Iso13849),
            "IEC61508" => Some(Self::Iec61508),
            "ISO26262" => Some(Self::Iso26262),
            _ => None,
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Iso13849, Self::Iec61508, Self::Iso26262]
    }
}

/// Program-level certification metadata (`certify ISO13849;` or block with level).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CertifyDecl {
    CertifyDecl {
        standard: CertificationStandard,
        level: Option<String>,
        span: Span,
    },
}

/// Program-level fleet grouping of robot names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FleetDecl {
    FleetDecl {
        name: String,
        members: Vec<String>,
        span: Span,
    },
}

/// Swarm coordination policy applied to a declared fleet group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwarmPolicy {
    RoundRobin,
    Broadcast,
    LeaderFollow,
}

impl SwarmPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoundRobin => "round_robin",
            Self::Broadcast => "broadcast",
            Self::LeaderFollow => "leader_follow",
        }
    }

    pub fn parse_ident(name: &str) -> Option<Self> {
        match name {
            "round_robin" => Some(Self::RoundRobin),
            "broadcast" => Some(Self::Broadcast),
            "leader_follow" => Some(Self::LeaderFollow),
            _ => None,
        }
    }
}

/// Program-level swarm coordinator binding a fleet to a coordination policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SwarmDecl {
    SwarmDecl {
        name: String,
        fleet_name: String,
        policy: SwarmPolicy,
        span: Span,
    },
}

/// Program-level safety zone policy with optional speed cap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ProgramSafetyZoneDecl {
    ProgramSafetyZoneDecl {
        name: String,
        max_speed_mps: Option<f64>,
        span: Span,
    },
}
