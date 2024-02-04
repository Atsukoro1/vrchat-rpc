use crate::clients::file_system::FileSystemClient;

use super::schema::Settings;

#[tauri::command] 
pub fn get_settings() -> Settings {
    let fs_client = FileSystemClient::new();

    match fs_client.read_json::<Settings>("settings.json", None) {
        Ok(settings) => settings,
        Err(_) => {
            let new_settings = Settings {
                minimize_to_tray: true,
                start_on_windows_startup: false,
            };

            fs_client.write_json("settings.json", &new_settings, None).unwrap();
            new_settings
        },
    }
}

#[tauri::command] 
pub fn set_settings() {
    let fs_client = FileSystemClient::new();
    let new_settings = Settings {
        minimize_to_tray: true,
        start_on_windows_startup: false,
    };

    fs_client.write_json("settings.json", &new_settings, None).unwrap();
}