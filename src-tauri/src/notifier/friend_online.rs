use crate::config::SettingsStore;
use crate::notifier::aumid::ensure_app_user_model_id;
use crate::notifier::{custom_sounds, windows_os};
use crate::vrchat_utils::AppResult;
use crate::websocket::FriendOnlineEvent;
use crate::{auth, vrchat_utils};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub async fn notify_friend_online(app: &AppHandle, event: FriendOnlineEvent) -> AppResult<()> {
    println!("PLATFORM = {}", event.platform);
    if event.platform == "web" {
        return Ok(());
    }

    ensure_app_user_model_id(&app)?;

    let app_settings = app.state::<SettingsStore>().snapshot();
    let friend_settings = app_settings.friend_settings_of(&event.user_id);

    if let Some(friend_settings) = friend_settings {
        if !friend_settings.enabled {
            return Ok(());
        }
    }

    let title = &event.user.display_name;
    let body = friend_settings
        .filter(|fs| fs.use_override)
        .and_then(|fs| fs.message_override.as_deref())
        .unwrap_or(&app_settings.default_message)
        .replace("%s", &event.user.display_name);
    let icon_src = match vrchat_utils::resolve_user_icon_url(&event.user) {
        Some(url) => {
            let auth_state = app.state::<auth::AuthState>();
            let (client, user_agent) = auth_state.with_session(|session| {
                (
                    session.config.client.clone(),
                    session
                        .config
                        .user_agent
                        .clone()
                        .unwrap_or_else(|| "vfriends".to_string()),
                )
            })?;
            vrchat_utils::fetch_user_icon_file_uri(&url, &client, &user_agent).await
        }
        None => None,
    };

    let silent_mode = windows_os::is_silent_mode(&app).unwrap_or(false);
    let should_play_override_sound = !silent_mode
        && friend_settings
            .as_ref()
            .is_some_and(|s| s.use_override && s.sound_override.is_some());

    if should_play_override_sound {
        if let Some(sound_path) = friend_settings
            .as_ref()
            .and_then(|s| s.sound_override.as_deref())
        {
            custom_sounds::play_custom_sound(PathBuf::from(sound_path));
        }
    }

    windows_os::notify(app, title, &body, icon_src, should_play_override_sound)
        .map_err(|err| err.to_string())
}
