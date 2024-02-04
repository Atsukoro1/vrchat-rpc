use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub static WS_URL: &str = "ws://your_websocket_server_url";

pub struct WebSocketState {
    pub stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketState {
    pub async fn connect(&mut self) -> Result<(), tokio_tungstenite::tungstenite::error::Error> {
        let (mut socket, _) = connect_async(
            Url::parse(WS_URL).expect("Can't connect to case count URL"),
        )
        .await?;

        while let Some(msg) = socket.next().await {
            let msg = msg?;
            println!("Received a message: {:?}", msg);
        }

        Ok(())
    }

    pub async fn close_connection(&mut self) {
        if let Some(mut stream) = self.stream.take() {
            // Close the WebSocket connection
            let _ = stream.close(None).await;
        }
    }
}
