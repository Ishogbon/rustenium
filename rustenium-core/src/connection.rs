use std::{collections::HashMap, net::TcpListener};
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;
use rustenium_bidi_commands::{CommandResult, Event};
use tokio::sync::{oneshot, Mutex};

use crate::{listeners::CommandResponseState, transport::ConnectionTransport};
use crate::listeners::{CommandResponseListener, Listener};
use crate::transport::WebsocketConnectionTransport;

pub struct Connection<'a, T: ConnectionTransport<'a>> {
    transport: T,
    pub commands_response_subscriptions: Arc<Mutex<HashMap<u32, oneshot::Sender<CommandResponseState>>>>,
    _marker: std::marker::PhantomData<&'a ()>,
}

pub fn find_free_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

impl<'a, T> Connection<'a, T>
where
    T: ConnectionTransport<'a>,
{
    pub fn new(connection_transport: T) -> Self {
        Self {
            transport: connection_transport,
            commands_response_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn start_listeners(&self) -> () {
        let (listener_tx, listener_rx) = unbounded_channel::<String>();
        let (command_response_tx, command_response_rx) = unbounded_channel::<CommandResponseState>();
        let (event_tx, event_rx) = unbounded_channel::<Event>();

        self.transport.listen(listener_tx);
        
        let listener = Listener::new(listener_rx, command_response_tx, event_tx);
        listener.start();
        
        let commands_response_listener = CommandResponseListener::new(command_response_rx, self.commands_response_subscriptions.clone());
        commands_response_listener.start();
    }

    pub async fn send(&mut self, data: String) {
        self.transport.send(data).await;
    }
}
