use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use rustenium_bidi_commands::{CommandResult, EventData, Message};

pub struct Listener {
    receiver: UnboundedReceiver<String>,
    pub command_result_sender: UnboundedSender<CommandResult>,
    pub event_sender: UnboundedSender<EventData>
}

impl Listener {
    pub fn new(receiver: UnboundedReceiver<String>, command_result_sender: UnboundedSender<CommandResult>, event_sender: UnboundedSender<EventData>) -> Self {
        Self { receiver, command_result_sender, event_sender }
    }
    fn start(mut self) {
        tokio::spawn(async move {
            while let Some(message) = self.receiver.recv().await {
                let parsed_message: Message = serde_json::from_str::<Message>(&message).unwrap();
                match parsed_message {
                    Message::CommandResponse(command_response) => {
                        self.command_result_sender.send(command_response.result).unwrap();
                    },
                    Message::Event(event) => {
                        self.event_sender.send(event.event_data).unwrap();
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

}

