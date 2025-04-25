use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use rustenium_bidi_commands::{CommandResult, EventData};

pub struct Listener {
    receiver: UnboundedReceiver<String>,
    command_result_sender: UnboundedSender<CommandResult>
    event_sender: UnboundedSender<EventData>
}

pub struct CommandResultListener {

}
