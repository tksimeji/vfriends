use super::platform;
use tauri::{AppHandle, Manager};
use crate::settings::SettingsStore;

#[derive(Debug, Clone)]
pub struct FriendOnlinePayload {
    pub display_name: String,
    pub title: String,
    pub body: String,
    pub image_url: Option<String>,
}

impl FriendOnlinePayload {
    pub fn new(display_name: &str, image_url: Option<String>, body: String) -> Self {
        let title = String::from("Friend Online");
        Self {
            display_name: display_name.to_string(),
            title,
            body,
            image_url,
        }
    }
}

pub async fn notify_friend_online(
    app: &AppHandle,
    friend_id: Option<String>,
    display_name: &str,
    image_url: Option<String>,
    user_agent: &str,
) {
    let prefs = app.state::<SettingsStore>();
    let resolved = resolve_settings(&prefs, friend_id.as_deref());
    let Some((message_template, sound)) = resolved else {
        return;
    };

    let body = format_message(&message_template, display_name);
    let payload = FriendOnlinePayload::new(display_name, image_url, body);
    platform::notify(app, &payload, user_agent, sound).await;
}

pub async fn preview_sound(app: &AppHandle, sound: Option<String>) {
    let payload = FriendOnlinePayload {
        display_name: String::from("vfriends"),
        title: String::from("通知音テスト"),
        body: String::from("設定した通知音のプレビューです。"),
        image_url: None,
    };
    platform::notify(app, &payload, "vfriends", sound).await;
}

fn resolve_settings(
    prefs: &SettingsStore,
    friend_id: Option<&str>,
) -> Option<(String, Option<String>)> {
    let settings = prefs.snapshot();
    let mut message_template = settings.default_message;
    let mut sound = settings.default_sound;

    if let Some(friend_id) = friend_id {
        if let Some(friend_pref) = settings.friend_settings.get(friend_id) {
            if !friend_pref.enabled {
                return None;
            }
            if friend_pref.use_override {
                if let Some(template) = friend_pref.message_override.clone() {
                    message_template = template;
                }
                if let Some(friend_sound) = friend_pref.sound_override.clone() {
                    sound = Some(friend_sound);
                }
            }
        }
    }

    Some((message_template, sound))
}

fn format_message(template: &str, display_name: &str) -> String {
    let trimmed = template.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    trimmed
        .replace("{name}", display_name)
        .replace("{displayName}", display_name)
}
