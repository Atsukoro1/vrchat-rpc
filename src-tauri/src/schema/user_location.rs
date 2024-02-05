use serde::{Deserialize, Serialize};
use super::{user::User, world::World};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLocationContent {
    user_id: Option<String>,
    user: Option<User>,
    location: Option<String>,
    instance: Option<String>,
    world_id: Option<String>,
    world: Option<World>,
}
