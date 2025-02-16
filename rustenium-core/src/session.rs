use crate::{
    connection::Connection,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
};

pub struct Session<T: ConnectionTransport> {
    connection: Connection<T>,
}

impl<T: ConnectionTransport> Session<T> {
    pub async fn ws_new(
        connection_config: ConnectionTransportConfig,
    ) -> Session<WebsocketConnectionTransport> {
        let connection_transport = WebsocketConnectionTransport::new(connection_config)
            .await
            .unwrap();
        let connection = Connection::new(connection_transport);
        return Session { connection };
    }
}
