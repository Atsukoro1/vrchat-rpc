use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub title: String,
    pub description: String,

    #[serde(rename = "showTimestamp")]
    pub show_timestamp: bool,
    
    #[serde(rename = "showPlayerStatus")]
    pub show_player_status: bool,

    #[serde(rename = "smallImageKey")]
    pub small_image_key: String,

    #[serde(rename = "largeImageKey")]
    pub large_image_key: String,
}