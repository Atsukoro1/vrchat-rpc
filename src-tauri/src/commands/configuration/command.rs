use crate::clients::file_system::FileSystemClient;

use super::schema::Configuration;

#[tauri::command] 
pub async fn get_configuration() -> Configuration {
    let fs_client = FileSystemClient::new();
    
    match fs_client.read_json::<Configuration>("configuration.json", None) {
        Ok(configuration) => configuration,
        Err(_) => {
            let new_config = Configuration {
                title: "VRCHAT".to_string(),
                description: "Playing rest & sleep lol".to_string(),
                show_timestamp: true,
                show_player_status: true,
                small_image_key: "This is a small image key".to_string(),
                large_image_key: "This is a large image key".to_string(),
            };

            fs_client.write_json("configuration.json", &new_config, None).unwrap();
            new_config
        },
    }
}

#[tauri::command]
pub async fn set_configuration(data: Configuration) -> bool {
    let fs_client = FileSystemClient::new();
    fs_client.write_json("configuration.json", &data, None).is_ok()
}