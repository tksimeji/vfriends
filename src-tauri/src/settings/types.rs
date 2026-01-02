use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vrchatapi::models;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub default_message: String,
    pub default_sound: Option<String>,
    pub friend_settings: HashMap<String, FriendSettings>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_message: String::from("{name} is online"),
            default_sound: None,
            friend_settings: HashMap::new(),
        }
    }
}

impl AppSettings {
    pub fn friend_settings_of(&self, friend: &models::LimitedUserFriend) -> Option<&FriendSettings> {
        self.friend_settings.get(&friend.id)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendSettings {
    pub enabled: bool,
    pub use_override: bool,
    pub message_override: Option<String>,
    pub sound_override: Option<String>,
}

impl Default for FriendSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            use_override: false,
            message_override: None,
            sound_override: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotificationConfig {
    pub message_template: String,
    pub sound: Option<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            message_template: String::from("{name} is online"),
            sound: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendNotification {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_use_override")]
    pub use_custom: bool,
    #[serde(default)]
    pub message_template: Option<String>,
    #[serde(default)]
    pub sound: Option<String>,
}

impl Default for FriendNotification {
    fn default() -> Self {
        Self {
            enabled: true,
            use_custom: default_use_override(),
            message_template: None,
            sound: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendNotificationPatch {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub use_custom: Option<bool>,
    #[serde(default)]
    pub message_template: Option<String>,
    #[serde(default)]
    pub sound: Option<String>,
}

fn default_enabled() -> bool {
    true
}

fn default_use_override() -> bool {
    false
}
