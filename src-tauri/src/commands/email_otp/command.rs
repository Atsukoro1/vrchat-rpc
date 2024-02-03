use super::schema::{EmailOtpErrorResponse, EmailOtpRequest, EmailOtpSuccessResponse};
use crate::clients::http_client::{HTTP_CLIENT_BASE_PATH, HTTP_CLIENT_USER_AGENT};
use crate::commands::email_otp::schema::{EmailOtpError, EmailOtpResponse};
use crate::utils::cookies::{format_cookies, get_response_cookie_by_name};
use cookie::{Cookie, CookieJar};
use reqwest::header;
use reqwest::Client;

#[tauri::command]
pub async fn email_otp(
    code: String,
    auth: String,
) -> Result<EmailOtpSuccessResponse, EmailOtpErrorResponse> {
    let mut cookie_jar = CookieJar::new();

    cookie_jar.add(Cookie::new("auth", auth));

    let body = EmailOtpRequest { code: code.clone() };

    let response = Client::new()
        .post(format!(
            "{}/auth/twofactorauth/emailotp/verify",
            HTTP_CLIENT_BASE_PATH
        ))
        .header(header::USER_AGENT, HTTP_CLIENT_USER_AGENT)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::COOKIE, format_cookies(&cookie_jar))
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let cookie = get_response_cookie_by_name(&response, "twoFactorAuth");
        let response_body = response.json::<EmailOtpResponse>().await.unwrap();

        return Ok(EmailOtpSuccessResponse {
            verified: response_body.verified,
            cookie: cookie,
        });
    }

    return Err(response
        .json::<EmailOtpError>()
        .await
        .unwrap()
        .error
        .unwrap());
}
