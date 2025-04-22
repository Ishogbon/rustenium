use std::net::TcpListener;
use crate::transport::ConnectionTransport;

pub struct Connection<'a, T: ConnectionTransport<'a>> {
    transport: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

pub fn find_free_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

impl<'a, T> Connection<'a, T>
where
    T: ConnectionTransport<'a>,
{
    pub fn new(connection_transport: T) -> Self {
        Self {
            transport: connection_transport,
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn send(&mut self, data: String) {
        self.transport.send(data).await;
    }
}
