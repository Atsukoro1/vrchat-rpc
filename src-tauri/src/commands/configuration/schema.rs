use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub title: String,
    pub description: String,

    #[serde(rename = "showTimestamp")]
    pub show_timestamp: bool,
    
    #[serde(rename = "showPlayerStatus")]
    pub show_player_status: bool,
}