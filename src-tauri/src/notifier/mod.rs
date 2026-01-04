mod aumid;
mod custom_sounds;
mod friend_online;
mod windows_os;

pub use custom_sounds::{play_custom_sound, store_custom_sound};
pub use friend_online::notify_friend_online;
use std::path::PathBuf;
use tauri::AppHandle;

pub async fn preview_sound(app: &AppHandle, sound: Option<String>) {
    let Some(sound_path) = sound else {
        let _ = windows_os::show_notification_sound_preview(app);
        return;
    };
    let trimmed_path = sound_path.trim();
    if trimmed_path.is_empty() {
        return;
    }
    play_custom_sound(PathBuf::from(trimmed_path));
}
