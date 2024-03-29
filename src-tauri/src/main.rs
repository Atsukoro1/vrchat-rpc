#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clients;
mod commands;
mod utils;
mod schema;

use commands::{
    credentials_auth::command::user_credentials_login, 
    email_otp::command::email_otp, 
    check_user::command::check_user,
    configuration::command::{
        get_configuration,
        set_configuration
    },
    settings::command::{
        get_settings,
        set_settings
    },
    websocket_handle::command::open_socket_handle
};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            user_credentials_login, 
            email_otp, 
            check_user, 
            get_configuration, 
            set_configuration,
            get_settings, 
            set_settings,
            open_socket_handle,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
