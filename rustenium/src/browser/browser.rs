use std::error::Error;
use rustenium_core::{
    process::Process,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};
use rustenium_core::session::SessionConnectionType;

const CDP_WEBSOCKET_ENDPOINT_REGEX: &str = r"^DevTools listening on (ws:\/\/.*)$";

pub trait Browser<T: ConnectionTransport> {
    fn exe_path(&self) -> &str;
    fn flags(&self) -> Vec<&str>;
    async fn open(&mut self) -> (Session<WebsocketConnectionTransport>, Process) {
        let mut browser_process = Process::create(self.exe_path(), self.flags());
        let browser_ws_endpoint = browser_process
            .wait_for_pattern(CDP_WEBSOCKET_ENDPOINT_REGEX, None)
            .await;
        let session = Some(
            Session::<T>::ws_new(ConnectionTransportConfig {
                endpoint: browser_ws_endpoint,
            })
            .await,
        )
        .expect("Something wong with created session");
        return (session, browser_process);
    }

    async fn new_session(&mut self, session: &mut Session<T>, connection_type: SessionConnectionType) -> Result<Ok(), dyn Error> {
        session.create_new_bidi_session(connection_type).await;
    }
}
