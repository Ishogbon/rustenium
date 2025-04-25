use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct Listener {
    receiver: UnboundedReceiver<String>
    command_result_sender: UnboundedSender<C>
}

pub struct CommandResultListener {

}
