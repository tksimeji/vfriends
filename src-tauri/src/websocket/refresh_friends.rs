use crate::auth::AuthState;
use crate::vrchat_utils;
use crate::vrchat_utils::AppResult;
use tauri::{AppHandle, Emitter, Manager};

pub async fn emit_refresh_friends(app: &AppHandle) -> AppResult<()> {
    let auth_state = app.state::<AuthState>();
    let friends = vrchat_utils::fetch_all_friends(auth_state.inner()).await?;
    app.emit("vrc:friends-refresh", friends)
        .map_err(|err| err.to_string())
}
