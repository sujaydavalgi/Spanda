//! DDS `TransportAdapter` implementation with optional live UDP multicast bridge.
//!
use spanda_runtime::security_types::EncryptionMode;
use spanda_runtime::RuntimeValue;
use spanda_transport::{AdapterMessage, StubTransportState, TransportAdapter, TransportConfig};

use crate::LiveDdsBridge;

/// DDS transport adapter with stub state and optional live multicast forwarding.
#[derive(Debug, Default)]
pub struct DdsTransportAdapterLive {
    state: StubTransportState,
    live: Option<LiveDdsBridge>,
}

/// Alias used by routing comm bus and provider bootstrap.
pub type DdsTransportAdapter = DdsTransportAdapterLive;

impl TransportAdapter for DdsTransportAdapterLive {
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

        //     let result = spanda_transport_dds::adapter::kind(&self);

        spanda_ast::comm_decl::TransportKind::Dds
    }

    fn connect(&mut self, config: &TransportConfig) -> Result<(), String> {
        // Description:
        //     Connect.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     config: &TransportConfig
        //         Caller-supplied config.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `connect`.
        //
        // Example:

        //     let result = spanda_transport_dds::adapter::connect(&mut self, config);

        config.security.validate(self.kind().as_str())?;

        // Require a negotiated TLS session when encryption is enabled.
        if config.security.encryption != EncryptionMode::None && !config.tls.negotiated {
            return Err("dds adapter requires negotiated TLS session".into());
        }

        self.state.connected = true;
        self.state.config = config.clone();

        // Connect a live DDS domain when SPANDA_LIVE_DDS is set.
        if std::env::var("SPANDA_LIVE_DDS").ok().as_deref() == Some("1") {
            let domain = config.domain_id.unwrap_or(0);
            self.live = LiveDdsBridge::connect(domain).ok();
        }
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

        //     let result = spanda_transport_dds::adapter::disconnect(&mut self);

        self.state.connected = false;
        self.live = None;
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

        //     let result = spanda_transport_dds::adapter::is_connected(&self);

        self.state.connected
    }

    fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue) {
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

        //     let result = spanda_transport_dds::adapter::publish(&mut self, opic, essage_type, value);

        if !self.state.connected {
            return;
        }

        // Forward string payloads to the live bridge when connected.
        if let RuntimeValue::String { value: payload } = &value {
            if let Some(live) = &self.live {
                let _ = live.publish(topic, payload);
            }
        }
        self.state.publish(topic, message_type, value);
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

        //     let result = spanda_transport_dds::adapter::subscribe(&mut self, opic);

        if self.state.connected {
            if let Some(live) = &self.live {
                let _ = live.subscribe(topic);
            }
            self.state.subscribe(topic);
        }
    }

    fn receive(&mut self, topic: &str) -> Option<RuntimeValue> {
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

        //     let result = spanda_transport_dds::adapter::receive(&mut self, opic);

        if !self.state.connected {
            return None;
        }

        // Prefer inbound messages from the live bridge.
        if let Some(live) = &self.live {
            if let Some(val) = live.receive(topic) {
                return Some(RuntimeValue::String { value: val });
            }
        }
        self.state.receive(topic)
    }

    fn call_service(
        &mut self,
        _service: &str,
        service_type: &str,
        _request: Option<RuntimeValue>,
    ) -> RuntimeValue {
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
        //     request: Option<RuntimeValue>
        //         Caller-supplied request.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `call_service`.
        //
        // Example:

        //     let result = spanda_transport_dds::adapter::call_service(&mut self, _service, service_type, _reques);

        StubTransportState::service_result(service_type)
    }

    fn send_action(
        &mut self,
        _action: &str,
        action_type: &str,
        _goal: RuntimeValue,
    ) -> RuntimeValue {
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
        //     _goal: RuntimeValue
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `send_action`.
        //
        // Example:

        //     let result = spanda_transport_dds::adapter::send_action(&mut self, _action, action_type, _goal);

        StubTransportState::action_result(action_type)
    }

    fn published(&self) -> Vec<AdapterMessage> {
        // Description:
        //     Published.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<AdapterMessage>
        //         Return value from `published`.
        //
        // Example:

        //     let result = spanda_transport_dds::adapter::published(&self);

        self.state.published.clone()
    }
}
