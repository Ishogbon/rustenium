use rustenium_core::{
    process::Process,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};

const CDP_WEBSOCKET_ENDPOINT_REGEX: &str = r"^DevTools listening on (ws:\/\/.*)$";

pub trait Browser<T: ConnectionTransport> {
    fn exe_path(&self) -> &str;
    fn flags(&self) -> Vec<&str>;
    async fn open(&mut self) -> (Session<WebsocketConnectionTransport>, Process) {
        let mut browser_process = Process::create(self.exe_path(), self.flags());
        let browserWsEndpoint = browser_process
            .wait_for_pattern(CDP_WEBSOCKET_ENDPOINT_REGEX, None)
            .await;
        println!("ws endpoint: {}", browserWsEndpoint);
        let session = Some(
            Session::<T>::ws_new(ConnectionTransportConfig {
                protocol: rustenium_core::transport::ConnectionTransportProtocol::Ws,
                host: String::from("127.0.0.1"),
                path: String::from("/devtools/browser"),
                port: 0,
            })
            .await,
        )
        .expect("Something wong with created session");
        return (session, browser_process);
    }
}
