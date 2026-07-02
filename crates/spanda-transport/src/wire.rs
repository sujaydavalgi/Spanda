//! Canonical JSON wire frames for Spanda transport adapters.

use crate::adapter::TransportConfig;
use serde::{Deserialize, Serialize};
use spanda_comm::TransportKind;
use spanda_runtime::security_types::EncryptionMode;
use spanda_runtime::serialize::{runtime_from_json_string, runtime_to_json_string};
use spanda_runtime::value::RuntimeValue;

/// Versioned transport envelope exchanged between adapters and the comm bus.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportWireFrame {
    pub v: u32,
    pub topic: String,
    pub message_type: String,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    pub transport: String,
}

impl TransportWireFrame {
    pub fn new(
        topic: &str,
        message_type: &str,
        value: &RuntimeValue,
        source_id: Option<&str>,
        transport: TransportKind,
    ) -> Result<Self, String> {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     opic: &str
        //         Caller-supplied opic.
        //     essage_type: &str
        //         Caller-supplied essage type.
        //     value: &RuntimeValue
        //         Caller-supplied value.
        //     source_id: Option<&str>
        //         Caller-supplied source id.
        //     ranspor: TransportKind
        //         Caller-supplied ranspor.
        //
        // Outputs:
        //     result: Result<Self, String>
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_transport::wire::new(opic, essage_type, value, source_id, ranspor);

        Ok(Self {
            v: 1,
            topic: topic.to_string(),
            message_type: message_type.to_string(),
            payload: runtime_to_json_string(value).map_err(|e| e.to_string())?,
            source_id: source_id.map(str::to_string),
            transport: transport.as_str().to_string(),
        })
    }

    pub fn decode_payload(&self) -> Result<RuntimeValue, String> {
        // Description:
        //     Decode payload.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Result<RuntimeValue, String>
        //         Return value from `decode_payload`.
        //
        // Example:

        //     let result = spanda_transport::wire::decode_payload(&self);

        runtime_from_json_string(&self.payload).map_err(|e| e.to_string())
    }
}

pub fn encode_wire_value(
    config: &TransportConfig,
    topic: &str,
    message_type: &str,
    value: &RuntimeValue,
    source_id: Option<&str>,
    transport: TransportKind,
) -> Result<RuntimeValue, String> {
    // Description:
    //     Encode wire value.
    //
    // Inputs:
    //     config: &TransportConfig
    //         Caller-supplied config.
    //     opic: &str
    //         Caller-supplied opic.
    //     essage_type: &str
    //         Caller-supplied essage type.
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //     source_id: Option<&str>
    //         Caller-supplied source id.
    //     ranspor: TransportKind
    //         Caller-supplied ranspor.
    //
    // Outputs:
    //     result: Result<RuntimeValue, String>
    //         Return value from `encode_wire_value`.
    //
    // Example:

    //     let result = spanda_transport::wire::encode_wire_value(config, opic, essage_type, value, source_id, ranspor);

    let frame = TransportWireFrame::new(topic, message_type, value, source_id, transport)?;
    let json = serde_json::to_string(&frame).map_err(|e| e.to_string())?;
    if config.security.encryption == EncryptionMode::None {
        return Ok(RuntimeValue::String { value: json });
    }
    let ciphertext = config.tls.encrypt_frame(&json)?;
    Ok(RuntimeValue::String { value: ciphertext })
}

pub fn decode_wire_value(
    config: &TransportConfig,
    value: RuntimeValue,
) -> Result<(RuntimeValue, Option<String>), String> {
    // Description:
    //     Decode wire value.
    //
    // Inputs:
    //     config: &TransportConfig
    //         Caller-supplied config.
    //     value: RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: Result<(RuntimeValue, Option<String>), String>
    //         Return value from `decode_wire_value`.
    //
    // Example:

    //     let result = spanda_transport::wire::decode_wire_value(config, value);

    let wire_text = match value {
        RuntimeValue::String { value } => {
            if config.security.encryption == EncryptionMode::None {
                value
            } else {
                config.tls.decrypt_frame(&value)?
            }
        }
        other => return Ok((other, None)),
    };

    let frame: TransportWireFrame = serde_json::from_str(&wire_text)
        .map_err(|e| format!("invalid transport wire frame: {e}"))?;
    let payload = frame.decode_payload()?;
    Ok((payload, frame.source_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::{TlsTransportSession, TransportSecurityConfig};
    use spanda_runtime::security_types::{AuthenticationMode, EncryptionMode, IntegrityMode};

    #[test]
    fn wire_frame_roundtrip_with_encryption() {
        // Description:
        //     Wire frame roundtrip with encryption.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport::wire::wire_frame_roundtrip_with_encryption();

        let mut tls = TlsTransportSession::default();
        let security = TransportSecurityConfig {
            encryption: EncryptionMode::Required,
            authentication: AuthenticationMode::None,
            integrity: IntegrityMode::None,
            cert_path: Some("certs/a.pem".into()),
            key_secret: Some("k1".into()),
            key_path: None,
        };
        tls.connect(&security, None).unwrap();
        let config = TransportConfig {
            security,
            tls,
            ..Default::default()
        };
        let value = RuntimeValue::Velocity {
            linear: 1.0,
            angular: 0.0,
        };
        let wire = encode_wire_value(
            &config,
            "/cmd",
            "Velocity",
            &value,
            Some("Navigator"),
            TransportKind::Mqtt,
        )
        .unwrap();
        let (decoded, source) = decode_wire_value(&config, wire).unwrap();
        assert_eq!(source.as_deref(), Some("Navigator"));
        assert!(matches!(decoded, RuntimeValue::Velocity { .. }));
    }
}
