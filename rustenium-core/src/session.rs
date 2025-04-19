use serde::{Deserialize, Serialize};
use rustenium_bidi_commands::session::commands::{New as SessionNew, NewMethod as SessionNewMethod, NewParameters as SessionNewParameters};
use rustenium_bidi_commands::session::types::CapabilitiesRequest;
use crate::{
    connection::Connection,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
};

pub struct Session<T: ConnectionTransport> {
    id: String,
    connection: Connection<T>,
}

pub enum SessionConnectionType {
    WebSocket
}
impl<T: ConnectionTransport> Session<T> {
    pub async fn ws_new(
        connection_config: ConnectionTransportConfig,
    ) -> Session<WebsocketConnectionTransport> {
        let session_id = connection_config.extract_session_id().unwrap();
        let connection_transport = WebsocketConnectionTransport::new(connection_config)
            .await
            .unwrap();
        let connection = Connection::new(connection_transport);
        return Session { id: session_id, connection };
    }

    pub async fn create_new_bidi_session(&mut self, connection_type: SessionConnectionType) -> () {
        match connection_type {
            SessionConnectionType::WebSocket => {
                let command = SessionNew {
                    method: SessionNewMethod::SessionNew,
                    params: SessionNewParameters {
                        capabilities: CapabilitiesRequest {
                            always_match: None,
                            first_match: None,
                        },
                    }
                };
                self.send(command).await;
            }
        }
    }

    async fn send(&mut self, message: impl Serialize)  {
        let raw_message = serde_json::to_string(&message).unwrap();
        self.connection.send(raw_message).await;
    }
}
