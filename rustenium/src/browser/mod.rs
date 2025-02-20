mod browser;
mod chrome;
use rustenium_core::{process::Process, transport::ConnectionTransport, Session};

pub use chrome::ChromeBrowser;

pub struct Browser<T: ConnectionTransport> {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    session: Option<Session<T>>,
    browser_process: Option<Process>,
}
