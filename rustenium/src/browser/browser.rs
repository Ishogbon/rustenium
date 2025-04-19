use std::error::Error;
use rustenium_core::{
    process::Process,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};
use rustenium_core::transport::Http;
use serde::Deserialize;
use tokio::io;
use tokio::time::{Duration, sleep};

const CDP_WEBSOCKET_ENDPOINT_REGEX: &str = r"^DevTools listening on (ws:\/\/.*)$";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebuggerTarget {
    pub web_socket_debugger_url: String,
}

fn is_connection_refused(e: &reqwest::Error) -> bool {
    if let Some(io_err) = e.source().and_then(|s| s.downcast_ref::<io::Error>()) {
        return io_err.kind() == io::ErrorKind::ConnectionRefused;
    }
    false
}

pub trait Browser<T: ConnectionTransport> {
    fn exe_path(&self) -> &str;
    fn flags(&self, port: u16) -> Vec<String>;
    async fn open(&mut self, debug_port: u16, timeout: Option<Duration>) -> (Session<WebsocketConnectionTransport>, Process) {
        let mut browser_process = Process::create(self.exe_path(), self.flags(debug_port));
        let wait_for = timeout.unwrap_or(Duration::from_secs(2)) / 5;
        let mut browser_ws_endpoint = None;
        for _ in 0..5 {
            sleep(wait_for).await;

            match self.get_websocket_url(None, None, debug_port).await {
                Ok(url) => {
                    browser_ws_endpoint = Some(url);
                    break;
                }
                Err(_) => continue
            }
        }
        let session = Some(
            Session::<T>::ws_new(ConnectionTransportConfig {
                endpoint: browser_ws_endpoint.expect("Unable to find browser debugger endpoint within specified time"),
            })
            .await,
        )
        .expect("Something wong with created session");
        return (session, browser_process);
    }

    async fn get_websocket_url(&self, scheme: Option<Http>, host: Option<&str>, debug_port: u16) -> Result<String, reqwest::Error>;
}
