use serde::{Deserialize, Serialize};
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

    pub async fn create_new_session(self, connection_type: SessionConnectionType) -> Session<T> {
        match connection_type {
            SessionConnectionType::WebSocket => {
            }
        }
    }

    async fn send(&self, message: impl Serialize)  {
        let raw_message = serde_json::to_string(&message).unwrap();
        self.connection.send(raw_message);
    }
}
