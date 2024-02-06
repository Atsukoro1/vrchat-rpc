use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub author_id: Option<String>,
    pub author_name: Option<String>,
    pub capacity: Option<i32>,
    pub recommended_capacity: Option<i32>,
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub favorites: Option<i32>,
    pub featured: Option<bool>,
    pub heat: Option<i32>,
    pub id: Option<String>,
    pub image_url: Option<String>,
    pub instances: Option<Vec<Vec<Option<serde_json::Value>>>>,
    pub labs_publication_date: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub occupants: Option<i32>,
    pub organization: Option<String>,
    pub popularity: Option<i32>,
    pub preview_youtube_id: Option<String>,
    pub private_occupants: Option<i32>,
    pub public_occupants: Option<i32>,
    pub publication_date: Option<String>,
    pub release_status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_image_url: Option<String>,
    pub unity_packages: Option<Vec<UnityPackage>>,
    pub updated_at: Option<String>,
    pub version: Option<i32>,
    pub visits: Option<i32>,
    pub udon_products: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnityPackage {
    pub asset_url: Option<String>,
    pub asset_url_object: Option<serde_json::Value>,
    pub asset_version: Option<i32>,
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub platform: Option<String>,
    pub plugin_url: Option<String>,
    pub plugin_url_object: Option<serde_json::Value>,
    pub unity_sort_number: Option<i64>,
    pub unity_version: Option<String>,
}
