//! Transport adapter trait, configuration, and shared stub state.

use crate::security::{TlsTransportSession, TransportSecurityConfig};
use spanda_ast::comm_decl::TransportKind;
use spanda_runtime::security_types::EncryptionMode;
use spanda_runtime::RuntimeValue;
use std::collections::{HashMap, VecDeque};

/// Serialize a runtime value into a ROS2-style service request payload string.
pub fn payload_string_for_service(value: &RuntimeValue) -> String {
    // Description:
    //     Payload string for service.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: String
    //         Return value from `payload_string_for_service`.
    //
    // Example:
    //     let result = spanda_transport::adapter::payload_string_for_service(value);

    // Match on value and handle each case.
    match value {
        RuntimeValue::String { value } => {
            format!(
                "{{data: \"{}\"}}",
                value.replace('\\', "\\\\").replace('"', "\\\"")
            )
        }
        RuntimeValue::Number { value, .. } => format!("{{value: {value}}}"),
        RuntimeValue::Bool { value } => format!("{{ok: {value}}}"),
        other => format!("{{raw: \"{other:?}\"}}"),
    }
}

/// Connection and security settings shared by all transport adapters.
#[derive(Debug, Clone, Default)]
pub struct TransportConfig {
    pub broker_url: Option<String>,
    pub node_name: Option<String>,
    pub namespace: Option<String>,
    pub domain_id: Option<u32>,
    pub client_id: Option<String>,
    pub security: TransportSecurityConfig,
    pub tls: TlsTransportSession,
}

/// One published message recorded by a transport adapter stub.
#[derive(Debug, Clone)]
pub struct AdapterMessage {
    pub topic: String,
    pub message_type: String,
    pub value: RuntimeValue,
}

/// Pluggable backend for ROS2, MQTT, DDS, and WebSocket transports.
pub trait TransportAdapter {
    fn kind(&self) -> TransportKind;
    fn connect(&mut self, config: &TransportConfig) -> Result<(), String>;
    fn disconnect(&mut self);
    fn is_connected(&self) -> bool;
    fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue);
    fn subscribe(&mut self, topic: &str);
    fn receive(&mut self, topic: &str) -> Option<RuntimeValue>;
    fn call_service(
        &mut self,
        service: &str,
        service_type: &str,
        request: Option<RuntimeValue>,
    ) -> RuntimeValue;
    fn send_action(&mut self, action: &str, action_type: &str, goal: RuntimeValue) -> RuntimeValue;
    fn published(&self) -> Vec<AdapterMessage>;
}

/// In-memory publish/subscribe state used by stub and live adapter wrappers.
#[derive(Debug, Default)]
pub struct StubTransportState {
    pub connected: bool,
    pub config: TransportConfig,
    subscriptions: HashMap<String, VecDeque<RuntimeValue>>,
    pub published: Vec<AdapterMessage>,
}

impl StubTransportState {
    pub fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue) {
        // Description:
        //     Publish.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     opic: &str
        //         Caller-supplied opic.
        //     essage_type: &str
        //         Caller-supplied essage type.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_transport::adapter::publish(&mut self, opic, essage_type, value);

        // Append into self.
        self.published.push(AdapterMessage {
            topic: topic.to_string(),
            message_type: message_type.to_string(),
            value: value.clone(),
        });

        // Emit output when get mut provides a buf.
        if let Some(buf) = self.subscriptions.get_mut(topic) {
            buf.push_back(value);
        }
    }

    pub fn subscribe(&mut self, topic: &str) {
        // Description:
        //     Subscribe.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_transport::adapter::subscribe(&mut self, opic);

        // Call entry on the current instance.
        self.subscriptions.entry(topic.to_string()).or_default();
    }

    pub fn receive(&mut self, topic: &str) -> Option<RuntimeValue> {
        // Description:
        //     Receive.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Option<RuntimeValue>
        //         Return value from `receive`.
        //
        // Example:
        //     let result = spanda_transport::adapter::receive(&mut self, opic);

        // Call subscriptions on the current instance.
        self.subscriptions
            .get_mut(topic)
            .and_then(|q| q.pop_front())
    }

    pub fn service_result(service_type: &str) -> RuntimeValue {
        // Description:
        //     Service result.
        //
        // Inputs:
        //     service_type: &str
        //         Caller-supplied service type.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `service_result`.
        //
        // Example:
        //     let result = spanda_transport::adapter::service_result(service_type);

        // Build a Object runtime value.
        RuntimeValue::Object {
            type_name: service_type.to_string(),
            fields: HashMap::from([("ok".into(), RuntimeValue::Bool { value: true })]),
        }
    }

    pub fn action_result(action_type: &str) -> RuntimeValue {
        // Description:
        //     Action result.
        //
        // Inputs:
        //     action_type: &str
        //         Caller-supplied action type.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `action_result`.
        //
        // Example:
        //     let result = spanda_transport::adapter::action_result(action_type);

        // Build a Object runtime value.
        RuntimeValue::Object {
            type_name: action_type.to_string(),
            fields: HashMap::from([("success".into(), RuntimeValue::Bool { value: true })]),
        }
    }
}

/// Generate a stub `TransportAdapter` implementation for simulation backends.
#[macro_export]
macro_rules! stub_adapter {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Default)]
        pub struct $name {
            state: $crate::StubTransportState,
        }

        impl $crate::TransportAdapter for $name {
            fn kind(&self) -> spanda_ast::comm_decl::TransportKind {
                // Description:
                //     Kind.
                //
                // Inputs:
                //     &self: input value
                //         Caller-supplied &self.
                //
                // Outputs:
                //     result: spanda_ast::comm_decl::TransportKind
                //         Return value from `kind`.
                //
                // Example:
                //     let result = spanda_transport::adapter::kind(&self);

                // Produce $kind as the result.
                $kind
            }

            fn connect(&mut self, config: &$crate::TransportConfig) -> Result<(), String> {
                // Description:
                //     Connect.
                //
                // Inputs:
                //     &mut self: input value
                //         Caller-supplied &mut self.
                //     config: &$crate::TransportConfig
                //         Caller-supplied config.
                //
                // Outputs:
                //     result: Result<(), String>
                //         Return value from `connect`.
                //
                // Example:
                //     let result = spanda_transport::adapter::connect(&mut self, config);

                // Call connected = true; on the current instance.
                config.security.validate(self.kind().as_str())?;
                if config.security.encryption != EncryptionMode::None && !config.tls.negotiated {
                    return Err(format!(
                        "{} adapter requires negotiated TLS session",
                        self.kind().as_str()
                    ));
                }
                self.state.connected = true;
                self.state.config = config.clone();
                Ok(())
            }

            fn disconnect(&mut self) {
                // Description:
                //     Disconnect.
                //
                // Inputs:
                //     &mut self: input value
                //         Caller-supplied &mut self.
                //
                // Outputs:
                //     None.
                //
                // Example:
                //     let result = spanda_transport::adapter::disconnect(&mut self);

                // Call connected = false; on the current instance.
                self.state.connected = false;
            }

            fn is_connected(&self) -> bool {
                // Description:
                //     Is connected.
                //
                // Inputs:
                //     &self: input value
                //         Caller-supplied &self.
                //
                // Outputs:
                //     result: bool
                //         Return value from `is_connected`.
                //
                // Example:
                //     let result = spanda_transport::adapter::is_connected(&self);

                // Call connected on the current instance.
                self.state.connected
            }

            fn publish(
                &mut self,
                topic: &str,
                message_type: &str,
                value: spanda_runtime::RuntimeValue,
            ) {
                // Description:
                //     Publish.
                //
                // Inputs:
                //     &mut self: input value
                //         Caller-supplied &mut self.
                //     opic: &str
                //         Caller-supplied opic.
                //     essage_type: &str
                //         Caller-supplied essage type.
                //     value: spanda_runtime::RuntimeValue
                //         Caller-supplied value.
                //
                // Outputs:
                //     None.
                //
                // Example:
                //     let result = spanda_transport::adapter::publish(&mut self, opic, essage_type, value);

                // take this path when self.state.connected.
                if self.state.connected {
                    self.state.publish(topic, message_type, value);
                }
            }

            fn subscribe(&mut self, topic: &str) {
                // Description:
                //     Subscribe.
                //
                // Inputs:
                //     &mut self: input value
                //         Caller-supplied &mut self.
                //     opic: &str
                //         Caller-supplied opic.
                //
                // Outputs:
                //     None.
                //
                // Example:
                //     let result = spanda_transport::adapter::subscribe(&mut self, opic);

                // take this path when self.state.connected.
                if self.state.connected {
                    self.state.subscribe(topic);
                }
            }

            fn receive(&mut self, topic: &str) -> Option<spanda_runtime::RuntimeValue> {
                // Description:
                //     Receive.
                //
                // Inputs:
                //     &mut self: input value
                //         Caller-supplied &mut self.
                //     opic: &str
                //         Caller-supplied opic.
                //
                // Outputs:
                //     result: Option<spanda_runtime::RuntimeValue>
                //         Return value from `receive`.
                //
                // Example:
                //     let result = spanda_transport::adapter::receive(&mut self, opic);

                // take this path when self.state.connected.
                if self.state.connected {
                    self.state.receive(topic)
                } else {
                    None
                }
            }

            fn call_service(
                &mut self,
                _service: &str,
                service_type: &str,
                _request: Option<spanda_runtime::RuntimeValue>,
            ) -> spanda_runtime::RuntimeValue {
                // Description:
                //     Call service.
                //
                // Inputs:
                //     &mut self: input value
                //         Caller-supplied &mut self.
                //     _service: &str
                //         Caller-supplied service.
                //     service_type: &str
                //         Caller-supplied service type.
                //     request: Option<spanda_runtime::RuntimeValue>
                //         Caller-supplied request.
                //
                // Outputs:
                //     result: spanda_runtime::RuntimeValue
                //         Return value from `call_service`.
                //
                // Example:
                //     let result = spanda_transport::adapter::call_service(&mut self, _service, service_type, _reques);

                // Produce service result as the result.
                $crate::StubTransportState::service_result(service_type)
            }

            fn send_action(
                &mut self,
                _action: &str,
                action_type: &str,
                _goal: spanda_runtime::RuntimeValue,
            ) -> spanda_runtime::RuntimeValue {
                // Description:
                //     Send action.
                //
                // Inputs:
                //     &mut self: input value
                //         Caller-supplied &mut self.
                //     _action: &str
                //         Caller-supplied action.
                //     action_type: &str
                //         Caller-supplied action type.
                //     _goal: spanda_runtime::RuntimeValue
                //         Caller-supplied goal.
                //
                // Outputs:
                //     result: spanda_runtime::RuntimeValue
                //         Return value from `send_action`.
                //
                // Example:
                //     let result = spanda_transport::adapter::send_action(&mut self, _action, action_type, _goal);

                // Produce action result as the result.
                $crate::StubTransportState::action_result(action_type)
            }

            fn published(&self) -> Vec<$crate::AdapterMessage> {
                // Description:
                //     Published.
                //
                // Inputs:
                //     &self: input value
                //         Caller-supplied &self.
                //
                // Outputs:
                //     result: Vec<$crate::AdapterMessage>
                //         Return value from `published`.
                //
                // Example:
                //     let result = spanda_transport::adapter::published(&self);

                // Call clone on the current instance.
                self.state.published.clone()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_string_for_string_value() {
        // Description:
        //     Payload string for string value.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport::adapter::payload_string_for_string_value();

        let value = RuntimeValue::String {
            value: "hello".into(),
        };
        let payload = payload_string_for_service(&value);
        assert!(payload.contains("hello"));
    }
}
