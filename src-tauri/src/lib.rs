mod authv2;
mod commands;
mod utils;

use crate::authv2::state::AuthState;
use crate::commands::{begin_auth, fetch_friends, restore_session, verify_two_factor};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AuthState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            begin_auth,
            verify_two_factor,
            restore_session,
            fetch_friends,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
