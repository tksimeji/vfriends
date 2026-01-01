use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub message_template: String,
    pub sound: Option<String>,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            message_template: String::from("{name} is online"),
            sound: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendNotificationPreference {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub message_template: Option<String>,
    #[serde(default)]
    pub sound: Option<String>,
}

impl Default for FriendNotificationPreference {
    fn default() -> Self {
        Self {
            enabled: true,
            message_template: None,
            sound: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendNotificationPreferencePatch {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub message_template: Option<String>,
    #[serde(default)]
    pub sound: Option<String>,
}

fn default_enabled() -> bool {
    true
}
