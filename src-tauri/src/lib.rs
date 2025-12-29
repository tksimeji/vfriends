mod authv2;

use crate::authv2::commands::{start_auth_flow, verify_two_factor};
use crate::authv2::service::AuthState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AuthState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_auth_flow, verify_two_factor,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
