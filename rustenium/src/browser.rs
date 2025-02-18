use std::{
    io::{self, Write},
    process::Command,
    thread,
    time::Duration,
};

use rustenium_core::{
    process::Process,
    session,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};

const CDP_WEBSOCKET_ENDPOINT_REGEX: &str = r"^DevTools listening on (ws:\/\/.*)$";

pub struct Browser {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    session: Option<Session<WebsocketConnectionTransport>>,
    browser_process: Option<Process>,
}

impl Default for Browser {
    fn default() -> Self {
        return Browser {
            exe_path: "google-chrome",
            flags: vec![],
            session: None,
            browser_process: None,
        };
    }
}

impl Browser {
    pub async fn open(&mut self) {
        let mut browser_process = Process::create(self.exe_path, self.get_flags());
        let browserWsEndpoint = browser_process
            .wait_for_pattern(CDP_WEBSOCKET_ENDPOINT_REGEX, None)
            .await;
        println!("ws endpoint: {}", browserWsEndpoint);
        self.session = Some(
            Session::<WebsocketConnectionTransport>::ws_new(ConnectionTransportConfig {
                protocol: rustenium_core::transport::ConnectionTransportProtocol::Ws,
                host: String::from("127.0.0.1"),
                path: String::from("/devtools/browser"),
                port: 0,
            })
            .await,
        );
        self.browser_process = Some(browser_process);
    }

    fn get_flags(&self) -> Vec<&str> {
        return vec!["--remote-debugging-port=0"];
    }
}
