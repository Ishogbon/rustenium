use std::{error::Error, future::Future};
use std::sync::Arc;
use fastwebsockets::{handshake, FragmentCollector, Frame, OpCode, Role, WebSocket, WebSocketError};
use hyper::{
    body::Bytes,
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Request,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use url::Url;

#[derive(Debug, Clone)]
pub enum Http {
    Http,
    Https,
}


impl Http {
    pub fn proto(&self) -> &'static str {
        match self {
            Http::Http => "http",
            Http::Https => "https",
        }
    }
}

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
    async fn send(&mut self, message: String) -> ();
    fn listen(&self) -> ();
    fn close(&self) -> ();
    fn on_close(&self) -> ();
}

pub struct WebsocketConnectionTransport {
    pub config: ConnectionTransportConfig,
    client: Arc<Mutex<FragmentCollector<TokioIo<Upgraded>>>>,
}
impl ConnectionTransport for WebsocketConnectionTransport {
    async fn send(&mut self, message: String) -> () {
        let frame = Frame::text(fastwebsockets::Payload::from(message.as_bytes()));
        self.client.lock().await.write_frame(frame).await.unwrap();
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

        let (mut ws, _) = handshake::client(&SpawnExecutor, req, stream).await.unwrap();
        ws = Self::configure_client(ws);
        let client = Arc::new(Mutex::new(FragmentCollector::new(ws)));
        WebsocketConnectionTransport::listener_loop(client.clone()).await.unwrap();
        println!("Successfully connected to browser");

        Ok(Self {
            config: connection_config,
            client,
        })
    }

    fn configure_client(mut ws: WebSocket<TokioIo<Upgraded>>) -> WebSocket<TokioIo<Upgraded>> {
        ws.set_writev(true);
        ws.set_auto_close(true);
        ws.set_auto_pong(true);

        ws
    }
    async fn listener_loop(ws: Arc<Mutex<FragmentCollector<TokioIo<Upgraded>>>>) -> Result<(), WebSocketError>
    {
        tokio::spawn(async move {
            loop {
                let frame = ws.lock().await.read_frame().await.unwrap();

                match frame.opcode {
                    OpCode::Close => break,
                    OpCode::Text | OpCode::Binary => {
                        let incoming = Frame::new(true, frame.opcode, None, frame.payload);
                        assert!(incoming.fin);
                    }
                    _ => {}
                }
            }
        });
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
