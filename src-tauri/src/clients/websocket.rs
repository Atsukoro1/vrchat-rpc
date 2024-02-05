use ezsockets::client::ClientCloseMode;
use ezsockets::WSError;

use ezsockets::Error;

use crate::utils::ws_message_handler;

pub static WS_URL: &str = "wss://pipeline.vrchat.cloud";
pub static WS_CLIENT_USER_AGENT: &str = "VRChat-Rpc-Client/1.0.0 (Tauri)";

pub struct WebsocketClient {
    pub handle: ezsockets::Client<Self>,
}

#[async_trait::async_trait]
impl ezsockets::ClientExt for WebsocketClient {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        ws_message_handler::handle_message(text).expect("Failed to handle the message");
        Ok(())
    }

    async fn on_binary(&mut self, _: Vec<u8>) -> Result<(), Error> {
        println!("Binary unsupported");
        Ok(())
    }

    async fn on_connect(&mut self) -> Result<(), Error> {
        println!("Websocket connected");
        Ok(())
    }

    async fn on_connect_fail(&mut self, error: WSError) -> Result<ClientCloseMode, Error> {
        println!("Websocket threw an error: {}", error.to_string());
        Ok(ClientCloseMode::Reconnect)
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
        let () = call;
        Ok(())
    }
}