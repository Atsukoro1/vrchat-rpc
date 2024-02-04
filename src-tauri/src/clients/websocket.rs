use futures_util::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message;
use url::Url;
use tokio_tungstenite::tungstenite::http::Uri;
use tokio::sync::Mutex;

pub static WS_URL: &str = "wss://pipeline.vrchat.cloud";
pub static WS_CLIENT_USER_AGENT: &str = "VRChat-Rpc-Client/1.0.0 (Tauri)";

pub struct WebSocketState {
    pub stream: Option<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub auth_token: Option<String>,
}
use tokio::time::sleep;
use std::time::Duration;

impl WebSocketState {
    pub async fn connect(&mut self) -> Result<(), tokio_tungstenite::tungstenite::error::Error> {
        println!("1");
        let url = Url::parse(
            format!(
                "{}/?authToken={}", 
                WS_URL, 
                self.auth_token.as_ref().unwrap()
            )
            .as_str(),
        )
        .unwrap();

        println!("2");
        // Convert Url to String and then parse it as Uri
        let uri: Uri = url.to_string().parse().unwrap();

        // Create a custom request builder with user agent header
        let request = tokio_tungstenite::tungstenite::handshake::client::Request::builder()
            .header("User-Agent", WS_CLIENT_USER_AGENT)
            .uri(&uri)
            .body(())
            .unwrap();

        println!("3");

        let (socket, _) = connect_async(request).await?;
        let socket = Mutex::new(socket);

        println!("4");

        self.stream = Some(socket);

        // Start processing messages
        loop {
            match self.stream.as_mut().unwrap().try_lock() {
                Ok(mut locked_stream) => {
                    while let Some(msg) = locked_stream.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                println!("Received a message: {}", text);
                            }
                            Ok(Message::Binary(bin)) => {
                                println!("Received binary data: {:?}", bin);
                            }
                            Ok(Message::Close(reason)) => {
                                println!("Received close signal: {:?}", reason);
                                break;
                            }
                            _ => {
                                // Handle other message types as needed
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error acquiring lock: {:?}", e);
                }
            }

            // Introduce a small delay to yield control to Tokio runtime
            sleep(Duration::from_millis(10)).await;
        }
    }
}
