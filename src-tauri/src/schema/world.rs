use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    author_id: Option<String>,
    author_name: Option<String>,
    capacity: Option<i32>,
    recommended_capacity: Option<i32>,
    created_at: Option<String>,
    description: Option<String>,
    favorites: Option<i32>,
    featured: Option<bool>,
    heat: Option<i32>,
    id: Option<String>,
    image_url: Option<String>,
    instances: Option<Vec<Vec<Option<serde_json::Value>>>>,
    labs_publication_date: Option<String>,
    name: Option<String>,
    namespace: Option<String>,
    occupants: Option<i32>,
    organization: Option<String>,
    popularity: Option<i32>,
    preview_youtube_id: Option<String>,
    private_occupants: Option<i32>,
    public_occupants: Option<i32>,
    publication_date: Option<String>,
    release_status: Option<String>,
    tags: Option<Vec<String>>,
    thumbnail_image_url: Option<String>,
    unity_packages: Option<Vec<UnityPackage>>,
    updated_at: Option<String>,
    version: Option<i32>,
    visits: Option<i32>,
    udon_products: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityPackage {
    asset_url: Option<String>,
    asset_url_object: Option<serde_json::Value>,
    asset_version: Option<i32>,
    created_at: Option<String>,
    id: Option<String>,
    platform: Option<String>,
    plugin_url: Option<String>,
    plugin_url_object: Option<serde_json::Value>,
    unity_sort_number: Option<i64>,
    unity_version: Option<String>,
}
