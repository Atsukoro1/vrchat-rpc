#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clients;
mod commands;
mod utils;

use commands::{credentials_auth::command::user_credentials_login, email_otp::command::email_otp};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![user_credentials_login, email_otp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
