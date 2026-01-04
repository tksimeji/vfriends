use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            default_message: String::from("%s is now online!"),
            default_sound: None,
            friend_settings: HashMap::new(),
        }
    }
}

impl AppSettings {
    pub fn friend_settings_of(&self, friend_id: &str) -> Option<&FriendSettings> {
        self.friend_settings.get(friend_id)
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
