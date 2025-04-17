mod browser;
mod chrome;

use std::error::Error;
use rustenium_core::{process::Process, transport::ConnectionTransport, Session};

pub use chrome::ChromeBrowser;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::WebsocketConnectionTransport;

pub struct Browser<T: ConnectionTransport> {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    session: Option<Session<T>>,
    pub sessions: Vec<Session<T>>,
    browser_process: Option<Process>,
}