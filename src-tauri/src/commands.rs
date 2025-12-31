use crate::authv2::state::AuthState;
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
