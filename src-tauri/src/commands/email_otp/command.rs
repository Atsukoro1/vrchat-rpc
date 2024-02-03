use super::schema::{EmailOtpErrorResponse, EmailOtpResponse};
use crate::clients::http_client::{HTTP_CLIENT_BASE_PATH, HTTP_CLIENT_USER_AGENT};
use crate::utils::cookies::{format_cookies, get_response_cookie_by_name};
use cookie::{Cookie, CookieJar};
use reqwest::header;
use reqwest::Client;

#[tauri::command]
pub async fn email_otp(
    code: String,
    auth: String,
) -> Result<EmailOtpResponse, EmailOtpErrorResponse> {
    let mut cookie_jar = CookieJar::new();

    cookie_jar.add(Cookie::new("auth", auth));

    let response = Client::new()
        .get(format!(
            "{}/auth/twofactorauth/emailotp/verify",
            HTTP_CLIENT_BASE_PATH
        ))
        .header(header::USER_AGENT, HTTP_CLIENT_USER_AGENT)
        .header(header::ACCEPT, "application/json")
        .header(header::COOKIE, format_cookies(&cookie_jar))
        .body(
            serde_json::to_string(&serde_json::json!({
                "code": code,
            }))
            .unwrap(),
        )
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let cookie = get_response_cookie_by_name(&response, "twoFactorAuth");

        return Ok(EmailOtpResponse {
            verified: true,
            cookie: cookie,
        });
    }

    return Err(EmailOtpErrorResponse {
        message: "Email OTP verification failed".to_string(),
        status_code: response.status().as_u16(),
    });
}
