use std::net::TcpListener;
use crate::transport::ConnectionTransport;

pub struct Connection<T: ConnectionTransport> {
    transport: T,
}

pub fn find_free_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    // Drop the listener so something else can use the port
    drop(listener);
    Ok(port)
}
impl<T: ConnectionTransport> Connection<T> {
    pub fn new(connection_transport: T) -> Connection<T> {
        return Connection {
            transport: connection_transport,
        };
    }

    pub async fn send(&mut self, data: String) {
        self.transport.send(data).await;
    }
}
