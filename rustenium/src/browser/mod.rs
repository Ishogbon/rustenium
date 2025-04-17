mod browser;
mod bidi;

use std::error::Error;
use rustenium_core::{process::Process, transport::ConnectionTransport, Session};

pub use bidi::chrome::ChromeBrowser;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::WebsocketConnectionTransport;

pub struct Browser<T: ConnectionTransport> {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    pub session: Option<Session<T>>,
    pub sessions: Vec<Session<T>>,
    browser_process: Option<Process>,
}

impl<T: ConnectionTransport> Browser<T> {
    async fn new_session(&mut self, connection_type: SessionConnectionType) -> Result<(), Box<dyn Error>> {
        self.session.as_mut().unwrap().create_new_bidi_session(connection_type).await;
        Ok(())
    }
}