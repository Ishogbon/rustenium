use crate::transport::ConnectionTransport;

pub struct Connection<T: ConnectionTransport> {
    transport: T,
}

impl<T: ConnectionTransport> Connection<T> {
    pub fn new(connection_transport: T) -> Connection<T> {
        return Connection {
            transport: connection_transport,
        };
    }

    pub fn send(&mut self, data: String) {
        self.transport.send(data);
    }
}
