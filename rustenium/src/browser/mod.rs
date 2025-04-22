mod browser;
mod bidi;

use std::error::Error;
use rustenium_core::{process::Process, transport::ConnectionTransport, Session};

pub use bidi::chrome::ChromeDriver;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::WebsocketConnectionTransport;

pub struct Driver<'a, T: ConnectionTransport<'a>>  {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    pub session: Option<Session<'a, T>>,
    driver_process: Option<Process>,
}

impl<'a, T: ConnectionTransport<'a>> Driver<'a, T> {
    async fn new_session(&mut self, connection_type: SessionConnectionType) -> Result<(), Box<dyn Error>> {
        self.session.as_mut().unwrap().create_new_bidi_session(connection_type).await;
        Ok(())
    }
}