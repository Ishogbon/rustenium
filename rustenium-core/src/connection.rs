use crate::transport::ConnectionTransport;

enum ConnectionProtocol {
    HTTP(String),
    HTTPS(String),
}

struct Connection<T: ConnectionTransport> {
    protocol: ConnectionProtocol,
    host: String,
    port: u16,
    transport: T,
}

impl<T: ConnectionTransport> Connection<T> {
    fn new(
        protocol: ConnectionProtocol,
        host: String,
        port: u16,
        connection_transport: T,
    ) -> Connection<T> {
        return Connection {
            protocol,
            host,
            port,
            transport: connection_transport,
        };
    }
}
