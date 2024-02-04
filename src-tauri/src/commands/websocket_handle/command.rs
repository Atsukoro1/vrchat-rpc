use tauri::command;

use crate::clients::websocket::WebSocketState;

use tokio::sync::Mutex;

// Define a mutable static variable to store WebSocketState
static mut WS_STATE: Option<Mutex<WebSocketState>> = None;

#[command]
pub async fn open_socket_handle<'a>(
    auth_token: String,
) -> Result<bool, ()> {
    unsafe {
        WS_STATE = Some(Mutex::new(WebSocketState {
            stream: None,
            auth_token: Some(auth_token.clone()),
        }));

        let mut ws_state = WS_STATE.as_mut().unwrap().lock().await;

        ws_state.auth_token = Some(auth_token.clone());

        let result = ws_state.connect().await.is_ok();

        Ok(result)
    }
}
