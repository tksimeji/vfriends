use crate::authv2::state::AuthState;
use crate::settings::{
    FriendNotification, FriendNotificationPatch, FriendSettings, NotificationConfig, SettingsStore,
};
use crate::utils::{fetch_all_friends, AppResult};
use tauri::{AppHandle, Manager, State};
use vrchatapi::models::{CurrentUser, LimitedUserFriend};

#[tauri::command]
pub async fn fetch_friends(state: State<'_, AuthState>) -> AppResult<Vec<LimitedUserFriend>> {
    fetch_all_friends(state.inner()).await
}

#[tauri::command]
pub async fn begin_auth(
    app: AppHandle,
    state: State<'_, AuthState>,
    username: String,
    password: String,
) -> AppResult<()> {
    state.inner().begin_auth(&app, username, password).await
}

#[tauri::command]
pub async fn verify_two_factor(
    app: AppHandle,
    state: State<'_, AuthState>,
    two_factor_code: String,
    two_factor_method: String,
) -> AppResult<()> {
    state
        .inner()
        .verify_two_factor(&app, two_factor_code, two_factor_method)
        .await
}

#[tauri::command]
pub async fn restore_session(app: AppHandle) -> AppResult<Option<CurrentUser>> {
    let state = app.state::<AuthState>();
    state.restore_session(&app).await
}

#[tauri::command]
pub fn logout(app: AppHandle, state: State<'_, AuthState>) -> AppResult<()> {
    state.logout(&app)
}

#[tauri::command]
pub fn fetch_notification_preferences(
    state: State<'_, SettingsStore>,
) -> AppResult<std::collections::HashMap<String, FriendNotification>> {
    let settings = state.snapshot();
    let mapped = settings
        .friend_settings
        .iter()
        .map(|(id, prefs)| (id.clone(), to_friend_notification(prefs)))
        .collect();
    Ok(mapped)
}

#[tauri::command]
pub fn set_notification_preference(
    state: State<'_, SettingsStore>,
    friend_id: String,
    patch: FriendNotificationPatch,
) -> AppResult<()> {
    state.consume(|settings| {
        let entry = settings
            .friend_settings
            .entry(friend_id)
            .or_insert_with(FriendSettings::default);

        if let Some(enabled) = patch.enabled {
            entry.enabled = enabled;
        }
        if let Some(use_custom) = patch.use_custom {
            entry.use_override = use_custom;
        }
        if let Some(message_template) = patch.message_template {
            entry.message_override = Some(message_template.trim().to_string());
        }
        if let Some(sound) = patch.sound {
            entry.sound_override = normalize_optional(sound);
        }
    });
    Ok(())
}

#[tauri::command]
pub fn fetch_notification_settings(
    state: State<'_, SettingsStore>,
) -> AppResult<NotificationConfig> {
    let settings = state.snapshot();
    Ok(NotificationConfig {
        message_template: settings.default_message,
        sound: settings.default_sound,
    })
}

#[tauri::command]
pub fn set_notification_settings(
    state: State<'_, SettingsStore>,
    settings: NotificationConfig,
) -> AppResult<()> {
    state.consume(|current| {
        current.default_message = settings.message_template;
        current.default_sound = normalize_optional(settings.sound.unwrap_or_default());
    });
    Ok(())
}

#[tauri::command]
pub async fn preview_notification_sound(app: AppHandle, sound: Option<String>) -> AppResult<()> {
    crate::notifications::preview_sound(&app, sound).await;
    Ok(())
}

#[tauri::command]
pub fn save_notification_sound(app: AppHandle, name: String, bytes: Vec<u8>) -> AppResult<String> {
    let path = crate::notifications::sound::store_sound_file(&app, &name, &bytes)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn fetch_cached_image_data(
    state: State<'_, AuthState>,
    url: String,
) -> AppResult<Option<String>> {
    let (client, user_agent) = state.with_session(|session| {
        (
            session.config.client.clone(),
            session
                .config
                .user_agent
                .clone()
                .unwrap_or_else(|| "vfriends".to_string()),
        )
    })?;

    let data =
        crate::notifications::fetch_cached_image_data_with_client(url, &client, &user_agent).await;
    Ok(data)
}

fn to_friend_notification(settings: &FriendSettings) -> FriendNotification {
    FriendNotification {
        enabled: settings.enabled,
        use_custom: settings.use_override,
        message_template: settings.message_override.clone(),
        sound: settings.sound_override.clone(),
    }
}

fn normalize_optional(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
