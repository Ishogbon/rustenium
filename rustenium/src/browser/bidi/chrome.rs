use std::error::Error;
use rustenium_core::{find_free_port, process::Process, transport::WebsocketConnectionTransport, Session};
use rustenium_core::session::SessionConnectionType;
use crate::{Driver, browser::browser::Driver as DriverTrait};
use reqwest::Client;
use serde::Deserialize;
use rustenium_core::transport::ConnectionTransportConfig;

pub struct ChromeDriver<'a> {
    connection_transport_config: ConnectionTransportConfig,
    pub driver: Driver<WebsocketConnectionTransport<'a>>,
}

impl <'a>DriverTrait<WebsocketConnectionTransport<'a>> for ChromeDriver<'a> {
    fn exe_path(&self) -> &str {
        return &self.driver.exe_path;
    }

    fn flags(&self) -> Vec<String> {
        return vec![
            format!("--host={}", self.connection_transport_config.host),
            format!("--port={}", self.connection_transport_config.port),
        ] .into_iter()
            .map(String::from)
            .collect();
    }
}

impl <'a>Default for ChromeDriver<'a> {
    fn default() -> Self {
        return ChromeDriver {
            connection_transport_config: Default::default(),
            driver: Driver {
                exe_path: "google-chrome",
                flags: vec![],
                session: None,
                driver_process: None,
            },
        };
    }
}

impl <'a>ChromeDriver<'a> {
    pub async fn launch(&mut self, host: Option<&str>, port: Option<u16>) -> () {
        let port = port.unwrap_or(find_free_port().unwrap());
        self.connection_transport_config.port = port;
        let result = self.start(&self.connection_transport_config).await;
        self.driver.session = Some(result.0);
        self.driver.driver_process = Some(result.1);
    }

    pub async fn create_new_session(&mut self, session_connection_type: SessionConnectionType) -> Result<(), Box<dyn Error>> {
        let session = self.driver.new_session(session_connection_type).await?;
        Ok(())
    }
}
