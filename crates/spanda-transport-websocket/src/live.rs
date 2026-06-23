//! Live WebSocket broker integration via tungstenite.
//!
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::net::TcpStream;
use std::sync::Mutex;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

type WsStream = WebSocket<MaybeTlsStream<TcpStream>>;

#[derive(Serialize, Deserialize)]
struct WireEnvelope {
    topic: String,
    payload: String,
}

#[derive(Debug)]
pub struct LiveWebsocketBridge {
    socket: Mutex<WsStream>,
    inbound: Mutex<HashMap<String, VecDeque<String>>>,
}

impl LiveWebsocketBridge {
    pub fn connect(broker_url: &str) -> Result<Self, String> {
        let (socket, _response) =
            connect(broker_url).map_err(|e| format!("websocket connect failed: {e}"))?;
        Ok(Self {
            socket: Mutex::new(socket),
            inbound: Mutex::new(HashMap::new()),
        })
    }

    fn poll_inbound(&self) {
        let mut guard = match self.socket.lock() {
            Ok(g) => g,
            Err(_) => return,
        };

        while let Ok(Message::Text(text)) = guard.read() {
            if let Ok(frame) = serde_json::from_str::<WireEnvelope>(&text) {
                if let Ok(mut map) = self.inbound.lock() {
                    map.entry(frame.topic).or_default().push_back(frame.payload);
                }
            }
        }
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        self.poll_inbound();
        let envelope = WireEnvelope {
            topic: topic.to_string(),
            payload: payload.to_string(),
        };
        let text = serde_json::to_string(&envelope)
            .map_err(|e| format!("websocket serialize failed: {e}"))?;
        let mut guard = self
            .socket
            .lock()
            .map_err(|e| format!("websocket lock failed: {e}"))?;
        guard
            .send(Message::Text(text))
            .map_err(|e| format!("websocket send failed: {e}"))
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        let envelope = WireEnvelope {
            topic: topic.to_string(),
            payload: "__subscribe__".into(),
        };
        let text = serde_json::to_string(&envelope)
            .map_err(|e| format!("websocket subscribe serialize failed: {e}"))?;
        let mut guard = self
            .socket
            .lock()
            .map_err(|e| format!("websocket lock failed: {e}"))?;
        guard
            .send(Message::Text(text))
            .map_err(|e| format!("websocket subscribe failed: {e}"))
    }

    pub fn receive(&self, topic: &str) -> Option<String> {
        self.poll_inbound();
        let mut map = self.inbound.lock().ok()?;
        map.get_mut(topic).and_then(|q| q.pop_front())
    }
}
