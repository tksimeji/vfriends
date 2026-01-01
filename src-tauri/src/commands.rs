use crate::authv2::state::AuthState;
use crate::notifications::{
    FriendNotificationPreference, FriendNotificationPreferencePatch, NotificationSettings,
    NotificationStore,
};
use crate::utils::{fetch_all_friends, AppResult};
use tauri::{AppHandle, Manager, State};
use vrchatapi::models::LimitedUserFriend;

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
pub async fn restore_session(app: AppHandle) -> AppResult<()> {
    let state = app.state::<AuthState>();
    state.restore_session(&app).await
}

#[tauri::command]
pub fn logout(app: AppHandle, state: State<'_, AuthState>) -> AppResult<()> {
    state.logout(&app)
}

#[tauri::command]
pub fn fetch_notification_preferences(
    state: State<'_, NotificationStore>,
) -> AppResult<std::collections::HashMap<String, FriendNotificationPreference>> {
    Ok(state.all())
}

#[tauri::command]
pub fn set_notification_preference(
    state: State<'_, NotificationStore>,
    friend_id: String,
    patch: FriendNotificationPreferencePatch,
) -> AppResult<()> {
    state.set_preference(friend_id, patch);
    Ok(())
}

#[tauri::command]
pub fn fetch_notification_settings(
    state: State<'_, NotificationStore>,
) -> AppResult<NotificationSettings> {
    Ok(state.settings())
}

#[tauri::command]
pub fn set_notification_settings(
    state: State<'_, NotificationStore>,
    settings: NotificationSettings,
) -> AppResult<()> {
    state.set_settings(settings);
    Ok(())
}

#[tauri::command]
pub async fn preview_notification_sound(
    app: AppHandle,
    sound: Option<String>,
) -> AppResult<()> {
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
