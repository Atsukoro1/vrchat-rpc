#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clients;
mod commands;
mod utils;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::auth::command::user_credentials_login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
