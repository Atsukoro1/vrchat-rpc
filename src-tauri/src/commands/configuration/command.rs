use super::schema::Configuration;

#[tauri::command] 
pub async fn get_configuration() -> Configuration {
    Configuration {
        title: "title".to_string(),
        description: "description".to_string(),
        show_timestamp: true,
        show_player_status: true,
    }
}

#[tauri::command]
pub async fn set_configuration(data: Configuration) -> String {
    "set_configuration".to_string()
}