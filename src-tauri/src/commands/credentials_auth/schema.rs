use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequiresTwoFactorAuth {
    #[serde(rename = "requiresTwoFactorAuth")]
    pub requires_two_factor_auth: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentialsLoginResponse {
    #[serde(rename = "authCookie")]
    pub auth_cookie: String,

    #[serde(rename = "requiresTwoFactorAuth")]
    pub requires_two_factor_auth: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentialsLoginError {
    pub error: Box<Option<UserCredentialsLoginErrorResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentialsLoginErrorResponse {
    pub message: String,
    pub status_code: i32,
}
