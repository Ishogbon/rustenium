use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use rustenium_bidi_commands::{CommandResponse, CommandResult, EventData, Message};

pub struct Listener {
    rx: UnboundedReceiver<String>,
    pub command_result_tx: UnboundedSender<CommandResponse>,
    pub event_tx: UnboundedSender<EventData>
}

impl Listener {
    pub fn new(rx: UnboundedReceiver<String>, command_result_tx: UnboundedSender<CommandResponse>, event_tx: UnboundedSender<EventData>) -> Self {
        Self { rx, command_result_tx, event_tx }
    }
    fn start(mut self) {
        tokio::spawn(async move {
            while let Some(message) = self.rx.recv().await {
                let parsed_message: Message = serde_json::from_str::<Message>(&message).unwrap();
                match parsed_message {
                    Message::CommandResponse(command_response) => {
                        self.command_result_tx.send(command_response).unwrap();
                    },
                    Message::Event(event) => {
                        self.event_tx.send(event.event_data).unwrap();
                    },
                    Message::ErrorResponse(error_response) => {
                        todo!();
                    }
                }
            }
        });
    }
}

pub struct CommandResultListener {
    rx: UnboundedReceiver<CommandResponse>,
}

impl CommandResultListener {
    pub fn new(rx: UnboundedReceiver<CommandResponse>, ) -> Self {
        Self { rx }
    }
    fn start(mut self) {
        tokio::spawn(async move {
            while let Some(command_response) = self.rx.recv().await {
                match command_response.result {
                    CommandResult::BrowsingContextResult(browsing_context_result) => {
                        todo!()
                    }
                    CommandResult::NetworkResult(network_result) => {
                        todo!()
                    }
                    CommandResult::ScriptResult(script_result) => {
                        todo!()
                    }
                    CommandResult::SessionResult(session_result) => {
                        todo!()
                    }
                    CommandResult::StorageResult(storage_result) => {
                        todo!()
                    }
                    CommandResult::WebExtensionResult(web_extension_result) => {
                        todo!()
                    }
                    CommandResult::EmptyResult(empty_result) => {
                        todo!()
                    }
                }
            }
        });
    }
}

