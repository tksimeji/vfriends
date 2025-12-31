mod authv2;
mod commands;
mod notifications;
mod pipeline;
mod shell;
mod utils;
mod websockets;

use crate::authv2::state::AuthState;
use crate::commands::{begin_auth, fetch_friends, restore_session, verify_two_factor};
use crate::pipeline::PipelineState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AuthState::new())
        .manage(PipelineState::default())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::Builder::new().args([shell::AUTOSTART_ARG]).build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            shell::setup(app.handle())?;
            Ok(())
        })
        .on_window_event(shell::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            begin_auth,
            verify_two_factor,
            restore_session,
            fetch_friends,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
