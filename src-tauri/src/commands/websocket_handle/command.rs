use tauri::command;

use ezsockets::ClientConfig;
use url::Url;

use crate::clients::websocket::{WebsocketClient, WS_CLIENT_USER_AGENT, WS_URL};

#[command]
pub async fn open_socket_handle<'a>(
    auth_token: String,
) -> Result<bool, ()> {
    let url = format!("{}/?authToken={}", WS_URL, auth_token)
        .parse::<Url>()
        .unwrap();
    let config = ClientConfig::new(url)
        .header("User-Agent", WS_CLIENT_USER_AGENT);

    let (_, future) = ezsockets::connect(|handle| WebsocketClient { handle }, config).await;
    future.await.unwrap();

    return Ok(true);
}