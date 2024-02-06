use serde::{Deserialize, Serialize};
use super::{user::User, world::World};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLocationContent {
    pub user_id: Option<String>,
    pub user: Option<User>,
    pub location: Option<String>,
    pub instance: Option<String>,
    pub world_id: Option<String>,
    pub world: Option<World>,
}
