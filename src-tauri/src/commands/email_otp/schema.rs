use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailOtpResponse {
    pub verified: bool,
    pub cookie: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailOtpError {
    pub error: Box<Option<EmailOtpErrorResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailOtpErrorResponse {
    pub message: String,
    pub status_code: u16,
}
