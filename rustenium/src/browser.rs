use std::process::Command;

use rustenium_core::{
    session,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};

pub struct Browser {
    pub exe_path: String,
    pub flags: Vec<String>,
    session: Option<Session<WebsocketConnectionTransport>>,
}

impl Default for Browser {
    fn default() -> Self {
        return Browser {
            exe_path: String::from("chrome"),
            flags: vec![],
            session: None,
        };
    }
}

impl Browser {
    pub async fn open(&mut self) {
        let _child = Command::new(self.exe_path.as_str())
            .args(self.get_flags())
            .spawn()
            .expect("Failed to start Chrome");
        self.session = Some(
            Session::<WebsocketConnectionTransport>::ws_new(ConnectionTransportConfig {
                protocol: rustenium_core::transport::ConnectionTransportProtocol::Http,
                host: String::from("127.0.0.1"),
                port: 9222,
            })
            .await,
        )
    }

    fn get_flags(&self) -> Vec<&str> {
        return vec![
            "--remote-debugging-port=9222",
            "--headless",
            "--disable-gpu",
        ];
    }

    fn get_debugger_url() -> &'static str {
        return "127.0.0.1:9222";
    }
}
