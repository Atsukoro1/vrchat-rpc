use crate::{
    clients::http_client::{HTTP_CLIENT_BASE_PATH, HTTP_CLIENT_USER_AGENT},
    utils::basic_auth::get_basic_auth,
};
use reqwest::Client;

#[tauri::command]
pub async fn user_credentials_login<'a>(username: &str, password: &str) -> Result<String, String> {
    let request = Client::new()
        .get(format!("{}/auth/user", HTTP_CLIENT_BASE_PATH))
        .header(
            "Authorization",
            format!("Basic {}", get_basic_auth(username, password)),
        )
        .header("User-Agent", HTTP_CLIENT_USER_AGENT)
        .header("Accept", "application/json")
        .send();

    let response = request.await.unwrap();

    if response.status().is_success() {
        return Ok(response
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string());
    } else {
        return Err(response.text().await.unwrap().to_string());
    }
}
