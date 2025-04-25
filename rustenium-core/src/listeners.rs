use std::{collections::HashMap, sync::{Arc}};

use rustenium_bidi_commands::{CommandResponse, CommandResult, ErrorResponse, Event, EventData, Message};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, oneshot, Mutex};

pub struct Listener {
    rx: UnboundedReceiver<String>,
    pub command_response_tx: UnboundedSender<CommandResponseState>,
    pub event_tx: UnboundedSender<Event>,
}

impl Listener {
    pub fn new(
        rx: UnboundedReceiver<String>,
        command_response_tx: UnboundedSender<CommandResponseState>,
        event_tx: UnboundedSender<Event>,
    ) -> Self {
        Self {
            rx,
            command_response_tx,
            event_tx,
        }
    }
    pub fn start(mut self) {
        tokio::spawn(async move {
            while let Some(message) = self.rx.recv().await {
                let parsed_message: Message = serde_json::from_str::<Message>(&message).unwrap();
                match parsed_message {
                    Message::CommandResponse(command_response) => {
                        self.command_response_tx
                            .send(CommandResponseState::Success(command_response))
                            .unwrap();
                    }
                    Message::Event(event) => {
                        self.event_tx.send(event).unwrap();
                    }
                    Message::ErrorResponse(error_response) => {
                        self.command_response_tx
                            .send(CommandResponseState::Error(error_response))
                            .unwrap();
                    }
                }
            }
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandResponseState {
    Success(CommandResponse),
    Error(ErrorResponse),
}
pub struct CommandResponseListener {
    subscriptions: Arc<Mutex<HashMap<u32, oneshot::Sender<CommandResponseState>>>>,
    rx: UnboundedReceiver<CommandResponseState>,
}

impl CommandResponseListener {
    pub fn new(rx: UnboundedReceiver<CommandResponseState>, subscriptions: Arc<Mutex<HashMap<u32, oneshot::Sender<CommandResponseState>>>>) -> Self {
        Self { rx, subscriptions }
    }
    pub fn start(mut self) {
        tokio::spawn(async move {
            while let Some(command_response) = self.rx.recv().await {
                match command_response {
                    CommandResponseState::Success(command_response) => {
                        let sender = self.subscriptions.lock().await.remove(&command_response.id);
                        if let Some(sender) = sender {
                            sender.send(CommandResponseState::Success(command_response)).unwrap();
                        }
                    }
                    CommandResponseState::Error(error_response) => {
                        let id = error_response.id;
                        if let Some(id) = id {
                            if let Some(sender) = self.subscriptions.lock().await.remove(&id) {
                                sender.send(CommandResponseState::Error(error_response)).unwrap();
                            }
                        }
                    }
                }
            }
        });
    }
}
