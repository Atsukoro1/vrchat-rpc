use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub allow_avatar_copying: Option<bool>,
    pub bio: Option<String>,
    pub bio_links: Option<Vec<String>>,
    pub current_avatar_image_url: Option<String>,
    pub current_avatar_thumbnail_image_url: Option<String>,
    pub date_joined: Option<String>,
    pub developer_type: Option<String>,
    pub display_name: Option<String>,
    pub friend_key: Option<String>,
    pub friend_request_status: Option<String>,
    pub id: Option<String>,
    pub instance_id: Option<String>,
    pub is_friend: Option<bool>,
    pub last_activity: Option<String>,
    pub last_login: Option<String>,
    pub last_platform: Option<String>,
    pub location: Option<String>,
    pub note: Option<String>,
    pub profile_pic_override: Option<String>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub status_description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub traveling_to_instance: Option<String>,
    pub traveling_to_location: Option<String>,
    pub traveling_to_world: Option<String>,
    pub user_icon: Option<String>,
    pub world_id: Option<String>,
}
