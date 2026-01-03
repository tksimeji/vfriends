use crate::authv2::state::AuthState;
use crate::notifier::aumid::ensure_app_user_model_id;
use crate::notifier::fetch_cached_image_file_with_client;
use crate::notifier::windows::notify;
use crate::settings::SettingsStore;
use crate::utils::AppResult;
use crate::websockets::FriendOnlineEvent;
use tauri::{AppHandle, Manager};

pub async fn notify_friend_online(app: &AppHandle, event: FriendOnlineEvent) -> AppResult<()> {
    if !ensure_app_user_model_id(app) {
        eprintln!("Toast notify aborted: AppUserModelID registration failed.");
        return Err(String::from("AppUserModelID cannot be used."));
    }

    let app_settings = app.state::<SettingsStore>().snapshot();
    let friend_settings = app_settings.friend_settings_of(&event.user_id);

    let title = &event.user.display_name;
    let body = friend_settings
        .filter(|fs| fs.use_override)
        .and_then(|fs| fs.message_override.as_deref())
        .unwrap_or(&app_settings.default_message)
        .replace("%s", &event.user.display_name);
    let icon_src = match resolve_user_icon_url(&event.user) {
        Some(url) => {
            let auth_state = app.state::<AuthState>();
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
            fetch_cached_image_file_with_client(&url, &client, &user_agent).await
        }
        None => None,
    };

    notify(app, title, &body, icon_src).map_err(|err| err.to_string())
}

fn resolve_user_icon_url(user: &crate::websockets::FriendOnlineUser) -> Option<String> {
    let not_empty: fn(&str) -> Option<&str> = |s: &str| (!s.is_empty()).then_some(s);
    not_empty(&user.profile_pic_override)
        .or_else(|| not_empty(&user.user_icon))
        .or_else(|| not_empty(&user.current_avatar_image_url))
        .or_else(|| not_empty(&user.current_avatar_thumbnail_image_url))
        .map(|url| url.to_string())
}
