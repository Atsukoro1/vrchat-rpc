use cookie::{Cookie, CookieJar};
use reqwest::{Client, header};

use crate::{clients::http_client::{HTTP_CLIENT_BASE_PATH, HTTP_CLIENT_USER_AGENT}, utils::cookies::format_cookies};

use super::schema::{VerifyUserError, VerifyUserErrorResponse, VerifyUserResponse};

#[tauri::command] 
pub async fn check_user(auth: String) -> Result<VerifyUserResponse, VerifyUserErrorResponse> {
    let mut cookie_jar = CookieJar::new();

    cookie_jar.add(Cookie::new("auth", auth));

    let response = Client::new()
        .get(format!(
            "{}/auth",
            HTTP_CLIENT_BASE_PATH
        ))
        .header(header::USER_AGENT, HTTP_CLIENT_USER_AGENT)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::COOKIE, format_cookies(&cookie_jar))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        return Ok(response.json::<VerifyUserResponse>().await.unwrap());
    }

    let error = response.json::<VerifyUserError>().await.unwrap();

    let res = error.error.unwrap();

    return Err(res);
}