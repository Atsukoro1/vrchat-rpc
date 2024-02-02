use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionLogEntry {
    message: String,
    #[serde(rename = "deletionScheduled")]
    deletion_scheduled: String,
    #[serde(rename = "dateTime")]
    date_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayNameUpdate {
    display_name: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Presence {
    avatar_thumbnail: String,
    display_name: String,
    groups: Vec<String>,
    id: String,
    instance: String,
    #[serde(rename = "instanceType")]
    instance_type: String,
    #[serde(rename = "isRejoining")]
    is_rejoining: String,
    platform: String,
    #[serde(rename = "profilePicOverride")]
    profile_pic_override: String,
    status: String,
    #[serde(rename = "travelingToInstance")]
    traveling_to_instance: String,
    #[serde(rename = "travelingToWorld")]
    traveling_to_world: String,
    world: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "acceptedTOSVersion")]
    accepted_tos_version: u32,
    #[serde(rename = "acceptedPrivacyVersion")]
    accepted_privacy_version: u32,
    #[serde(rename = "accountDeletionDate")]
    account_deletion_date: String,
    #[serde(rename = "accountDeletionLog")]
    account_deletion_log: Vec<DeletionLogEntry>,
    #[serde(rename = "activeFriends")]
    active_friends: Vec<String>,
    #[serde(rename = "allowAvatarCopying")]
    allow_avatar_copying: bool,
    bio: String,
    #[serde(rename = "bioLinks")]
    bio_links: Vec<String>,
    #[serde(rename = "currentAvatar")]
    current_avatar: String,
    #[serde(rename = "currentAvatarAssetUrl")]
    current_avatar_asset_url: String,
    #[serde(rename = "currentAvatarImageUrl")]
    current_avatar_image_url: String,
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    current_avatar_thumbnail_image_url: String,
    #[serde(rename = "currentAvatarTags")]
    current_avatar_tags: Vec<String>,
    #[serde(rename = "date_joined")]
    date_joined: String,
    #[serde(rename = "developerType")]
    developer_type: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    #[serde(rename = "fallbackAvatar")]
    fallback_avatar: String,
    #[serde(rename = "friendKey")]
    friend_key: String,
    friends: Vec<String>,
    #[serde(rename = "hasBirthday")]
    has_birthday: bool,
    #[serde(rename = "hideContentFilterSettings")]
    hide_content_filter_settings: bool,
    #[serde(rename = "userLanguage")]
    user_language: String,
    #[serde(rename = "userLanguageCode")]
    user_language_code: String,
    #[serde(rename = "hasEmail")]
    has_email: bool,
    #[serde(rename = "hasLoggedInFromClient")]
    has_logged_in_from_client: bool,
    #[serde(rename = "hasPendingEmail")]
    has_pending_email: bool,
    #[serde(rename = "homeLocation")]
    home_location: String,
    id: String,
    #[serde(rename = "isFriend")]
    is_friend: bool,
    #[serde(rename = "last_activity")]
    last_activity: String,
    #[serde(rename = "last_login")]
    last_login: String,
    #[serde(rename = "last_platform")]
    last_platform: String,
    #[serde(rename = "obfuscatedEmail")]
    obfuscated_email: String,
    #[serde(rename = "obfuscatedPendingEmail")]
    obfuscated_pending_email: String,
    #[serde(rename = "oculusId")]
    oculus_id: String,
    #[serde(rename = "googleId")]
    google_id: String,
    #[serde(rename = "picoId")]
    pico_id: String,
    #[serde(rename = "viveId")]
    vive_id: String,
    #[serde(rename = "offlineFriends")]
    offline_friends: Vec<String>,
    #[serde(rename = "onlineFriends")]
    online_friends: Vec<String>,
    #[serde(rename = "pastDisplayNames")]
    past_display_names: Vec<DisplayNameUpdate>,
    presence: Presence,
    #[serde(rename = "profilePicOverride")]
    profile_pic_override: String,
    state: String,
    status: String,
    #[serde(rename = "statusDescription")]
    status_description: String,
    #[serde(rename = "statusFirstTime")]
    status_first_time: bool,
    #[serde(rename = "statusHistory")]
    status_history: Vec<String>,
    #[serde(rename = "steamDetails")]
    steam_details: serde_json::Value,
    #[serde(rename = "steamId")]
    steam_id: String,
    tags: Vec<String>,
    #[serde(rename = "twoFactorAuthEnabled")]
    two_factor_auth_enabled: bool,
    #[serde(rename = "twoFactorAuthEnabledDate")]
    two_factor_auth_enabled_date: String,
    unsubscribe: bool,
    #[serde(rename = "updated_at")]
    updated_at: String,
    #[serde(rename = "userIcon")]
    user_icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequiresTwoFactorAuth {
    #[serde(rename = "requiresTwoFactorAuth")]
    requires_two_factor_auth: Vec<String>,
}
