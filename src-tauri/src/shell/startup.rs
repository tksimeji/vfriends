use super::tray::hide_main_window;
use super::AUTOSTART_ARG;
use tauri::{AppHandle, Manager};

use tauri_plugin_autostart::ManagerExt as AutostartManagerExt;
use crate::auth;

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    if is_autostart() {
        hide_main_window(app);
    }

    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        let state = app_handle.state::<auth::AuthState>();
        let _ = state.restore_session(&app_handle).await;
    });

    if let Err(err) = app.autolaunch().enable() {
        log::error!("Failed to enable autostart: {err}");
    }

    Ok(())
}

fn is_autostart() -> bool {
    std::env::args().any(|arg| arg == AUTOSTART_ARG)
}
