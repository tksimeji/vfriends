mod auth;
mod commands;
mod config;
mod notifier;
mod shell;
mod vrchat_utils;
mod websocket;

use crate::config::SettingsStore;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(auth::AuthState::new())
        .manage(websocket::WebsocketState::default())
        .plugin(
            tauri_plugin_frame::FramePluginBuilder::new()
                .titlebar_height(48)
                .button_width(46)
                .auto_titlebar(true)
                .snap_overlay_delay_ms(10)
                .close_hover_bg("rgb(196, 43, 28)")
                .button_hover_bg("rgba(106,227,249,0.12)")
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .args([shell::AUTOSTART_ARG])
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(SettingsStore::load(app.handle()));
            shell::setup(app.handle())?;
            Ok(())
        })
        .on_window_event(shell::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            commands::begin_auth,
            commands::verify_two_factor,
            commands::restore_session,
            commands::logout,
            commands::fetch_friends,
            commands::fetch_world,
            commands::fetch_friend_settings,
            commands::set_friend_settings,
            commands::fetch_app_settings,
            commands::set_app_settings,
            commands::preview_notification_sound,
            commands::save_notification_sound,
            commands::save_notification_sound_path,
            commands::fetch_icon_data_uri,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app.");
}
