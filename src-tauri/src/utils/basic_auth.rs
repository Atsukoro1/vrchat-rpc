use base64::{engine::general_purpose, Engine as _};
use urlencoding::encode;

pub fn get_basic_auth(username: &str, password: &str) -> String {
    let encoded_username = encode(username);
    let encoded_password = encode(password);

    let combined_credentials = format!("{}:{}", encoded_username, encoded_password);

    general_purpose::STANDARD.encode(&combined_credentials)
}
