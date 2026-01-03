pub(crate) mod sound;
mod images;
mod friend_online;
mod aumid;
mod windows;
pub use friend_online::notify_friend_online;
pub use images::{fetch_cached_image_data_with_client, fetch_cached_image_file_with_client};

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
    sound::play_sound_file(PathBuf::from(trimmed));
}
