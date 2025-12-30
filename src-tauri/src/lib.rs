mod authv2;
mod utils;

use crate::authv2::commands::{restore_session, start_auth_flow, verify_two_factor};
use crate::authv2::service::AuthState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AuthState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_auth_flow,
            verify_two_factor,
            restore_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
