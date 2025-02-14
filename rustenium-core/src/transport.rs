use std::{error::Error, future::Future};

use fastwebsockets::{handshake, WebSocket};
use hyper::{
    body::Bytes,
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Request,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
enum ConnectionProtocol {
    Http,
    Https,
    Ws,
    Wss,
}

#[derive(Debug, Clone)]
struct ConnectionConfig {
    pub protocol: ConnectionProtocol,
    pub host: String,
    pub port: u16,
}

impl From<ConnectionConfig> for String {
    fn from(config: ConnectionConfig) -> Self {
        let protocol = match config.protocol {
            ConnectionProtocol::Http => "http",
            ConnectionProtocol::Ws => "ws",
            ConnectionProtocol::Https => "https",
            ConnectionProtocol::Wss => "wss",
        };
        format!("{}://{}:{}", protocol, config.host, config.port)
    }
}

pub trait ConnectionTransport {
    fn send(&self, message: String) -> ();
    fn on_message(&self, message: String) -> ();
    fn close(&self) -> ();
    fn on_close(&self) -> ();
}

pub struct WebsocketConnectionTransport {
    pub config: ConnectionConfig,
    client: WebSocket<TokioIo<Upgraded>>,
}
impl ConnectionTransport for WebsocketConnectionTransport {
    fn send(&self, message: String) -> () {
        todo!()
    }

    fn on_message(&self, message: String) -> () {
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
    async fn new(connection_config: ConnectionConfig) -> Result<Self, Box<dyn Error>> {
        let ws_url: String = connection_config.clone().into();
        let stream = TcpStream::connect(ws_url.clone()).await?;

        let req = Request::builder()
            .method("GET")
            .uri(ws_url)
            .header("Host", "localhost:9001")
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
