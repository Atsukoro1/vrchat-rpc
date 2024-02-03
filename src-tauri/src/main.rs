#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clients;
mod commands;
mod utils;

use commands::{
    credentials_auth::command::user_credentials_login, 
    email_otp::command::email_otp, 
    check_user::command::check_user,
    configuration::command::{
        get_configuration,
        set_configuration
    }
};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            user_credentials_login, 
            email_otp, 
            check_user, 
            get_configuration, 
            set_configuration
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
