use std::error::Error;
use rustenium_core::{
    process::Process,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};
use rustenium_core::transport::ConnectionTransportProtocol;
use serde::Deserialize;
use tokio::io;

fn is_connection_refused(e: &reqwest::Error) -> bool {
    if let Some(io_err) = e.source().and_then(|s| s.downcast_ref::<io::Error>()) {
        return io_err.kind() == io::ErrorKind::ConnectionRefused;
    }
    false
}

pub trait Driver<'a, T: ConnectionTransport<'a>> {
    fn exe_path(&self) -> &str;
    fn flags(&self) -> Vec<String>;
    async fn start(& self, connection_transport_config: &'a ConnectionTransportConfig<'a>) -> (Session<'a, WebsocketConnectionTransport<'a>>, Process) {
        let driver_process = Process::create(self.exe_path(), self.flags());
        let session =
            Session::<T>::ws_new(connection_transport_config)
                .await;
        (session, driver_process)
    }
}
