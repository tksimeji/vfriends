mod aumid;
mod custom_sounds;
mod friend_online;
mod windows_os;

pub use custom_sounds::{play_custom_sound, store_custom_sound};
pub use friend_online::notify_friend_online;
use std::path::PathBuf;
use tauri::AppHandle;

pub async fn preview_sound(_app: &AppHandle, sound: Option<String>) {
    let Some(value) = sound else {
        return;
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return;
    }
    play_custom_sound(PathBuf::from(trimmed));
}
