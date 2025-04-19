use std::error::Error;
use rustenium_core::{find_free_port, process::Process, transport::WebsocketConnectionTransport, Session};
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::Http;
use crate::{Browser, browser::browser::Browser as BrowserTrait};
use crate::browser::browser::DebuggerTarget;
use reqwest::Client;
use serde::Deserialize;

pub struct ChromeBrowser {
    pub browser: Browser<WebsocketConnectionTransport>,
}

impl BrowserTrait<WebsocketConnectionTransport> for ChromeBrowser {
    fn exe_path(&self) -> &str {
        return &self.browser.exe_path;
    }

    fn flags(&self, port: u16) -> Vec<String> {
        return vec![
            "--allow-pre-commit-input",
            "--disable-background-networking",
            "--disable-background-timer-throttling",
            "--disable-backgrounding-occluded-windows",
            "--disable-breakpad",
            "--disable-client-side-phishing-detection",
            "--disable-component-extensions-with-background-pages",
            "--disable-crash-reporter", // No crash reporting in CfT.
            "--disable-default-apps",
            "--disable-dev-shm-usage",
            "--disable-extensions",
            "--disable-hang-monitor",
            "--disable-infobars",
            "--disable-ipc-flooding-protection",
            "--disable-popup-blocking",
            "--disable-prompt-on-repost",
            "--disable-renderer-backgrounding",
            "--disable-search-engine-choice-screen",
            "--disable-sync",
            // "--enable-automation",
            "--export-tagged-pdf",
            "--force-color-profile=srgb",
            "--generate-pdf-document-outline",
            "--metrics-recording-only",
            "--no-first-run",
            "--password-store=basic",
            "--use-mock-keychain",
            &format!("--remote-debugging-port={}", port),
            "--enable-logging",
        ] .into_iter()
            .map(String::from)
            .collect();
    }

    async fn get_websocket_url(
        &self,
        scheme: Option<Http>,
        host: Option<&str>,
        port: u16,
    ) -> Result<String, reqwest::Error> {
        let host_to_use = host.unwrap_or("localhost");
        let scheme_to_use = scheme.unwrap_or(Http::Http);
        let base_url = format!("{}://{}:{}/json", scheme_to_use.proto(), host_to_use, port);

        let client = Client::new();
        let response = client.get(&base_url).send().await?;

        let targets: Vec<DebuggerTarget> = response.json().await?;

        let ws_url = targets
            .get(0)
            .ok_or("No targets found at /json").unwrap()
            .web_socket_debugger_url
            .clone();

        Ok(ws_url)
    }
}

impl Default for ChromeBrowser {
    fn default() -> Self {
        return ChromeBrowser {
            browser: Browser {
                exe_path: "google-chrome",
                flags: vec![],
                session: None,
                sessions: vec![],
                browser_process: None,
            },
        };
    }
}

impl ChromeBrowser {
    pub async fn launch(&mut self) -> () {
        let port = find_free_port().unwrap();
        let result = self.open(port, None).await;
        self.browser.session = Some(result.0);
        self.browser.browser_process = Some(result.1);
    }

    pub async fn create_new_session(&mut self, session_connection_type: SessionConnectionType) -> Result<(), Box<dyn Error>> {
        let session = self.browser.new_session(session_connection_type).await?;
        Ok(())
    }
}
