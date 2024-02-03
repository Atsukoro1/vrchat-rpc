use crate::{
    clients::http_client::{HTTP_CLIENT_BASE_PATH, HTTP_CLIENT_USER_AGENT},
    utils::{basic_auth::get_basic_auth, cookies::get_response_cookie_by_name},
};
use reqwest::{header, Client, Response};

use super::schema::{
    RequiresTwoFactorAuth, UserCredentialsLoginError, UserCredentialsLoginErrorResponse,
    UserCredentialsLoginResponse,
};

#[tauri::command]
pub async fn user_credentials_login<'a>(
    username: &str,
    password: &str,
) -> Result<UserCredentialsLoginResponse, UserCredentialsLoginErrorResponse> {
    let request = Client::new()
        .get(format!("{}/auth/user", HTTP_CLIENT_BASE_PATH))
        .header(
            header::AUTHORIZATION,
            format!("Basic {}", get_basic_auth(username, password)),
        )
        .header(header::USER_AGENT, HTTP_CLIENT_USER_AGENT)
        .header(header::ACCEPT, "application/json")
        .send();

    let response = request.await.unwrap();

    if response.status().is_success() {
        let auth_cookie = get_response_cookie_by_name(&response, "auth");
        let requires_two_factor_auth = get_2fa_methods(response).await.requires_two_factor_auth;

        return Ok(UserCredentialsLoginResponse {
            auth_cookie,
            requires_two_factor_auth,
        });
    }

    let error = response.json::<UserCredentialsLoginError>().await.unwrap();
    return Err(error.error.unwrap());
}

/// This function is used to get the 2FA methods required for the user to login.
/// If 2FA is required then the array will contain one of the following methods: "otp", "totp", "emailOtp"
/// Will return empty array if no 2FA is required.
async fn get_2fa_methods(response: Response) -> RequiresTwoFactorAuth {
    let res_text = response.text().await.unwrap();

    if res_text.contains("requiresTwoFactorAuth") {
        return serde_json::from_str(&res_text).unwrap();
    } else {
        return RequiresTwoFactorAuth {
            requires_two_factor_auth: vec![],
        };
    }
}
