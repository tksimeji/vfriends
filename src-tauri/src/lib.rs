mod authv2;
mod commands;
mod notifications;
mod pipeline;
mod shell;
mod utils;
mod websockets;
mod settings;

use crate::authv2::state::AuthState;
use crate::commands::{
    begin_auth,
    fetch_friends,
    fetch_notification_preferences,
    fetch_notification_settings,
    fetch_cached_image_data,
    preview_notification_sound,
    save_notification_sound,
    logout,
    restore_session,
    set_notification_preference,
    set_notification_settings,
    verify_two_factor,
};
use crate::pipeline::PipelineState;
use crate::settings::SettingsStore;
use tauri::Manager;
use tauri_plugin_frame::FramePluginBuilder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AuthState::new())
        .manage(PipelineState::default())
        .plugin(
            FramePluginBuilder::new()
                .titlebar_height(48)
                .button_width(46)
                .auto_titlebar(true)
                .snap_overlay_delay_ms(10)
                .close_hover_bg("rgba(140,24,24,0.8)")
                .button_hover_bg("rgba(106,227,249,0.12)")
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::Builder::new().args([shell::AUTOSTART_ARG]).build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(SettingsStore::load(app.handle()));
            shell::setup(app.handle())?;
            Ok(())
        })
        .on_window_event(shell::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            begin_auth,
            verify_two_factor,
            restore_session,
            logout,
            fetch_friends,
            fetch_notification_preferences,
            set_notification_preference,
            fetch_notification_settings,
            set_notification_settings,
            preview_notification_sound,
            save_notification_sound,
            fetch_cached_image_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
