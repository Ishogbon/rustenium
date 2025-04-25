use rand::Rng;
use rustenium_bidi_commands::{Command, CommandData};
use rustenium_bidi_commands::session::commands::{New as SessionNew, NewMethod as SessionNewMethod, NewParameters as SessionNewParameters, SessionCommand};
use rustenium_bidi_commands::session::types::CapabilitiesRequest;
use crate::{
    connection::Connection,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
};

pub struct Session<'a, T: ConnectionTransport<'a>> {
    id: Option<String>,
    connection: Connection<'a, T>,
}

pub enum SessionConnectionType {
    WebSocket
}
impl<'a, T: ConnectionTransport<'a>> Session<'a, T> {
    pub async fn ws_new(
        connection_config: &'a ConnectionTransportConfig<'a>,
        pre_session: bool,
    ) -> Session<WebsocketConnectionTransport<'a>> {
        let connection_transport = WebsocketConnectionTransport::new(connection_config)
            .await
            .unwrap();
        let connection = Connection::new(connection_transport);
        Session { id: None, connection }
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
                self.send(CommandData::SessionCommand(SessionCommand::New(command))).await;
            }
        }
    }

    async fn send(&mut self, command_data: CommandData)  {
        let command = Command {
            id : rand::rng().random::<u32>(),
            command_data,
            extension: None
        };
        let raw_message = serde_json::to_string(&command).unwrap();
        self.connection.send(raw_message).await;
    }
}
