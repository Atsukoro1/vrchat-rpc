use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketEvent {
    pub r#type: String,
    // This field is JSON structured as a string (needs to parse)
    pub content: String,
}