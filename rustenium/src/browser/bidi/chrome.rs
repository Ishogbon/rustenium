use std::error::Error;
use rustenium_core::{process::Process, transport::WebsocketConnectionTransport, Session};
use rustenium_core::session::SessionConnectionType;
use crate::{Browser, browser::browser::Browser as BrowserTrait};

pub struct ChromeBrowser {
    pub browser: Browser<WebsocketConnectionTransport>,
}

impl BrowserTrait<WebsocketConnectionTransport> for ChromeBrowser {
    fn exe_path(&self) -> &str {
        return &self.browser.exe_path;
    }

    fn flags(&self) -> Vec<&str> {
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
            "--remote-debugging-port=0",
            "--enable-logging",
        ];
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
        let result = self.open().await;
        self.browser.session = Some(result.0);
        self.browser.browser_process = Some(result.1);
    }

    pub async fn create_new_session(&mut self, session_connection_type: SessionConnectionType) -> Result<(), Box<dyn Error>> {
        let session = self.browser.new_session(session_connection_type).await?;
        self.launch().await;
        Ok(())
    }
}
