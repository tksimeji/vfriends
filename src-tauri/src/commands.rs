use crate::auth::AuthState;
use crate::config::{AppSettings, FriendSettings, SettingsStore};
use crate::vrchat_utils::AppResult;
use crate::{auth, notifier, vrchat_utils};
use serde::Deserialize;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use vrchatapi::apis::worlds_api;
use vrchatapi::models;
use vrchatapi::models::{CurrentUser, LimitedUserFriend};

#[tauri::command]
pub async fn fetch_friends(
    state: State<'_, AuthState>,
) -> AppResult<Vec<LimitedUserFriend>> {
    vrchat_utils::fetch_all_friends(state.inner()).await
}

#[tauri::command]
pub async fn fetch_world(
    state: State<'_, AuthState>,
    world_id: String,
) -> AppResult<models::World> {
    let config = state.with_session(|session| session.config.clone())?;
    match worlds_api::get_world(&config, &world_id).await {
        Ok(world) => Ok(world),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn begin_auth(
    app: AppHandle,
    state: State<'_, AuthState>,
    username: String,
    password: String,
) -> AppResult<()> {
    state.inner().begin_auth_flow(&app, username, password).await
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
pub fn fetch_friend_settings(
    state: State<'_, SettingsStore>,
) -> AppResult<HashMap<String, FriendSettings>> {
    let app_settings = state.snapshot();
    Ok(app_settings.friend_settings)
}

#[tauri::command]
pub fn set_friend_settings(
    state: State<'_, SettingsStore>,
    friend_id: String,
    patch: FriendSettingsPatch,
) -> AppResult<()> {
    state.consume(|settings| {
        let entry = settings
            .friend_settings
            .entry(friend_id)
            .or_insert_with(FriendSettings::default);

        if let Some(enabled) = patch.enabled {
            entry.enabled = enabled;
        }
        if let Some(use_override) = patch.use_override {
            entry.use_override = use_override;
        }
        if let Some(message_override) = patch.message_override {
            entry.message_override = normalize_optional(message_override);
        }
        if let Some(sound_override) = patch.sound_override {
            entry.sound_override = normalize_optional(sound_override);
        }
    });
    Ok(())
}

#[tauri::command]
pub fn fetch_app_settings(
    state: State<'_, SettingsStore>,
) -> AppResult<AppSettings> {
    Ok(state.snapshot())
}

#[tauri::command]
pub fn set_app_settings(
    state: State<'_, SettingsStore>,
    settings: AppSettingsPatch,
) -> AppResult<()> {
    state.consume(|current| {
        if let Some(default_message) = settings.default_message {
            current.default_message = default_message;
        }
        if let Some(default_sound) = settings.default_sound {
            current.default_sound = normalize_optional(default_sound);
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn preview_notification_sound(app: AppHandle, sound: Option<String>) -> AppResult<()> {
    notifier::preview_sound(&app, sound).await;
    Ok(())
}

#[tauri::command]
pub fn save_notification_sound(app: AppHandle, name: String, bytes: Vec<u8>) -> AppResult<String> {
    let path = notifier::store_custom_sound(&app, &name, &bytes)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn fetch_icon_data_uri(
    state: State<'_, auth::AuthState>,
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

    let data = vrchat_utils::fetch_user_icon_data_uri(&url, &client, &user_agent).await;
    Ok(data)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendSettingsPatch {
    pub enabled: Option<bool>,
    pub use_override: Option<bool>,
    pub message_override: Option<String>,
    pub sound_override: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsPatch {
    pub default_message: Option<String>,
    pub default_sound: Option<String>,
}

fn normalize_optional(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
