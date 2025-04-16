use std::{error::Error, future::Future};
use fastwebsockets::{handshake, Frame, OpCode, Role, WebSocket, WebSocketError,};
use hyper::{
    body::Bytes,
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Request,
};
use hyper::client::conn::http1::handshake;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::Url;

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
    pub endpoint: String,
}

impl ConnectionTransportConfig {
    // These two methods below might need optimization, calling url::parse multiple times might not be efficient
    pub fn endpoint_to_host_port(&self) -> Option<String> {
        if let Ok(url) = Url::parse(&self.endpoint) {
            if let Some(host) = url.host_str() {
                if let Some(port) = url.port() {
                    return Some(format!("{}:{}", host, port));
                }
            }
        }
        None
    }
    pub fn get_endpoint_path(&self) -> String {
        let url = Url::parse(&self.endpoint);
        return url.unwrap().path().to_owned();
    }

    pub fn extract_session_id(&self) -> Option<String> {
        if let Ok(url) = Url::parse(&self.endpoint) {
            let path = url.path();
            if !path.is_empty() {
                let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
                if let Some(last_segment) = segments.last() {
                    return Some(last_segment.to_string());
                }
            }
        }
        None
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
        let addr_host = connection_config.endpoint_to_host_port().unwrap();
        let stream = TcpStream::connect(&addr_host).await.unwrap();
        let uri = connection_config.get_endpoint_path();
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .header("Host", &addr_host)
            .header(UPGRADE, "websocket")
            .header(CONNECTION, "upgrade")
            .header(
                "Sec-WebSocket-Key",
                fastwebsockets::handshake::generate_key(),
            )
            .header("Sec-WebSocket-Version", "13")
            .body(http_body_util::Empty::<Bytes>::new()).unwrap();

        let (client, _) = handshake::client(&SpawnExecutor, req, stream).await.unwrap();

        println!("Successfully connected to browser");

        Ok(Self {
            config: connection_config,
            client,
        })
    }

    async fn listener_loop(&self, socket: &mut TcpStream) -> Result<(), WebSocketError> {
        let mut ws = WebSocket::after_handshake(socket, Role::Client);
        ws.set_writev(true);
        ws.set_auto_close(true);
        ws.set_auto_pong(true);

        loop {
            let frame = ws.read_frame().await?;

            match frame.opcode {
                OpCode::Close => break,
                OpCode::Text | OpCode::Binary => {
                    let frame = Frame::new(true, frame.opcode, None, frame.payload);
                }
                _ => {}
            }
        }

        Ok(())
    }

} //

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
