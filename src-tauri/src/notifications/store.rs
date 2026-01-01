use super::model::{
    FriendNotificationPreference, FriendNotificationPreferencePatch, NotificationSettings,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
struct NotificationPreferenceFile {
    #[serde(default, deserialize_with = "deserialize_friend_preferences")]
    friends: HashMap<String, FriendNotificationPreference>,
    #[serde(default)]
    settings: NotificationSettings,
}

impl Default for NotificationPreferenceFile {
    fn default() -> Self {
        Self {
            friends: HashMap::new(),
            settings: NotificationSettings::default(),
        }
    }
}

pub struct NotificationStore {
    path: PathBuf,
    friends: Mutex<HashMap<String, FriendNotificationPreference>>,
    settings: Mutex<NotificationSettings>,
}

impl NotificationStore {
    pub fn load(app: &AppHandle) -> Self {
        let path = preferences_path(app);
        let file = read_preferences(&path);
        Self {
            path,
            friends: Mutex::new(file.friends),
            settings: Mutex::new(file.settings),
        }
    }

    pub fn all(&self) -> HashMap<String, FriendNotificationPreference> {
        self.friends
            .lock()
            .map(|prefs| prefs.clone())
            .unwrap_or_default()
    }

    pub fn preference(&self, friend_id: &str) -> Option<FriendNotificationPreference> {
        self.friends
            .lock()
            .ok()
            .and_then(|prefs| prefs.get(friend_id).cloned())
    }

    pub fn set_preference(&self, friend_id: String, patch: FriendNotificationPreferencePatch) {
        let (friends_snapshot, settings_snapshot) = {
            let mut prefs = match self.friends.lock() {
                Ok(prefs) => prefs,
                Err(poisoned) => poisoned.into_inner(),
            };
            let entry = prefs
                .entry(friend_id)
                .or_insert_with(FriendNotificationPreference::default);
            if let Some(enabled) = patch.enabled {
                entry.enabled = enabled;
            }
            if let Some(message_template) = patch.message_template {
                entry.message_template = normalize_optional(message_template);
            }
            if let Some(sound) = patch.sound {
                entry.sound = normalize_optional(sound);
            }

            let settings = self
                .settings
                .lock()
                .map(|settings| settings.clone())
                .unwrap_or_default();
            (prefs.clone(), settings)
        };

        if let Err(err) = write_preferences(&self.path, &friends_snapshot, &settings_snapshot) {
            eprintln!("Failed to save notification preferences: {err}");
        }
    }

    pub fn settings(&self) -> NotificationSettings {
        self.settings
            .lock()
            .map(|settings| settings.clone())
            .unwrap_or_default()
    }

    pub fn set_settings(&self, settings: NotificationSettings) {
        let (friends_snapshot, settings_snapshot) = {
            let mut current = match self.settings.lock() {
                Ok(settings_guard) => settings_guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *current = NotificationSettings {
                message_template: settings.message_template,
                sound: normalize_optional(settings.sound.unwrap_or_default()),
            };
            let friends = self
                .friends
                .lock()
                .map(|friends| friends.clone())
                .unwrap_or_default();
            (friends, current.clone())
        };

        if let Err(err) = write_preferences(&self.path, &friends_snapshot, &settings_snapshot) {
            eprintln!("Failed to save notification preferences: {err}");
        }
    }
}

fn preferences_path(app: &AppHandle) -> PathBuf {
    let base = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("vfriends"));
    base.join("notification_prefs.json")
}

fn read_preferences(path: &Path) -> NotificationPreferenceFile {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return NotificationPreferenceFile::default();
    };

    serde_json::from_str::<NotificationPreferenceFile>(&contents)
        .unwrap_or_else(|_| NotificationPreferenceFile::default())
}

fn normalize_optional(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn write_preferences(
    path: &Path,
    friends: &HashMap<String, FriendNotificationPreference>,
    settings: &NotificationSettings,
) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let data = NotificationPreferenceFile {
        friends: friends.clone(),
        settings: settings.clone(),
    };
    let json = serde_json::to_string_pretty(&data)
        .unwrap_or_else(|_| "{\"friends\":{},\"settings\":{}}".to_string());
    std::fs::write(path, json)
}

fn deserialize_friend_preferences<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, FriendNotificationPreference>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = HashMap::<String, serde_json::Value>::deserialize(deserializer)?;
    let mut output = HashMap::new();

    for (key, value) in raw {
        let preference = match value {
            serde_json::Value::Bool(enabled) => FriendNotificationPreference {
                enabled,
                ..FriendNotificationPreference::default()
            },
            serde_json::Value::Object(_) => {
                serde_json::from_value::<FriendNotificationPreference>(value)
                    .unwrap_or_default()
            }
            _ => continue,
        };
        output.insert(key, preference);
    }

    Ok(output)
}
