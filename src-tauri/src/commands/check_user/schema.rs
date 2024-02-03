use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VerifyUserResponse {
    pub ok: bool,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyUserError {
    pub error: Box<Option<VerifyUserErrorResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyUserErrorResponse {
    pub message: String,
    pub status_code: i32,
}
