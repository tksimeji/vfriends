use crate::config::SettingsStore;
use crate::notifier::aumid::ensure_app_user_model_id;
use crate::notifier::windows_os::notify;
use crate::vrchat_utils::AppResult;
use crate::websocket::FriendOnlineEvent;
use crate::{auth, vrchat_utils};
use tauri::{AppHandle, Manager};

pub async fn notify_friend_online(app: &AppHandle, event: FriendOnlineEvent) -> AppResult<()> {
    ensure_app_user_model_id(&app)?;

    let app_settings = app.state::<SettingsStore>().snapshot();
    let friend_settings = app_settings.friend_settings_of(&event.user_id);

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

    notify(app, title, &body, icon_src).map_err(|err| err.to_string())
}
