use std::{error::Error, future::Future};

use fastwebsockets::{handshake, Frame, WebSocket};
use hyper::{
    body::Bytes,
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Request,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub enum ConnectionTransportProtocol {
    Http,
    Https,
    Ws,
    Wss,
}

pub enum UrlFormat {
    HostPort,
    ProtocolHostPort,
    Full, // protocol://host:port/path
}

#[derive(Debug, Clone)]
pub struct ConnectionTransportConfig {
    pub protocol: ConnectionTransportProtocol,
    pub host: String,
    pub path: String,
    pub port: u16,
}

impl ConnectionTransportConfig {
    pub fn to_string(&self, format: UrlFormat) -> String {
        match format {
            UrlFormat::HostPort => format!("{}:{}", self.host, self.port),
            UrlFormat::ProtocolHostPort => {
                format!("{}://{}:{}", self.protocol_str(), self.host, self.port)
            }
            UrlFormat::Full => format!(
                "{}://{}:{}{}",
                self.protocol_str(),
                self.host,
                self.port,
                self.path
            ),
        }
    }

    fn protocol_str(&self) -> &str {
        match self.protocol {
            ConnectionTransportProtocol::Http => "http",
            ConnectionTransportProtocol::Ws => "ws",
            ConnectionTransportProtocol::Https => "https",
            ConnectionTransportProtocol::Wss => "wss",
        }
    }
}

pub trait ConnectionTransport {
    fn send(&mut self, message: String) -> ();
    fn listen(&self) -> ();
    fn close(&self) -> ();
    fn on_close(&self) -> ();
}

pub struct WebsocketConnectionTransport {
    pub config: ConnectionTransportConfig,
    client: WebSocket<TokioIo<Upgraded>>,
}
impl ConnectionTransport for WebsocketConnectionTransport {
    fn send(&mut self, message: String) -> () {
        let frame = Frame::text(fastwebsockets::Payload::from(message.as_bytes()));
        self.client.write_frame(frame);
    }

    fn listen(&self) -> () {
        todo!()
    }

    fn close(&self) -> () {
        todo!()
    }

    fn on_close(&self) -> () {
        todo!()
    }
}

impl WebsocketConnectionTransport {
    pub async fn new(connection_config: ConnectionTransportConfig) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(connection_config.to_string(UrlFormat::HostPort)).await?;

        let req = Request::builder()
            .method("GET")
            .uri(connection_config.to_string(UrlFormat::Full))
            .header(UPGRADE, "websocket")
            .header(CONNECTION, "upgrade")
            .header(
                "Sec-WebSocket-Key",
                fastwebsockets::handshake::generate_key(),
            )
            .header("Sec-WebSocket-Version", "13")
            .body(http_body_util::Empty::<Bytes>::new())?;

        let (client, _) = handshake::client(&SpawnExecutor, req, stream).await?;
        Ok(Self {
            config: connection_config,
            client,
        })
    }
}

struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}
