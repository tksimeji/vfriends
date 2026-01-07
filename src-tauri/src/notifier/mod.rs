mod aumid;
mod custom_sounds;
mod friend_online;
mod windows_os;

use crate::config::AppSettings;
pub use custom_sounds::{play_custom_sound, store_custom_sound, store_custom_sound_from_path, validate_sound_path};
pub use friend_online::notify_friend_online;
use std::collections::HashSet;
use std::path::PathBuf;
use std::path::Path;
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

pub fn cleanup_unused_sounds(app: &AppHandle, settings: &AppSettings) {
    let referenced_files = collect_referenced_sound_files(app, settings);
    if referenced_files.is_empty() {
        if let Err(err) = custom_sounds::cleanup_custom_sounds(app, &referenced_files) {
            log::warn!("Failed to cleanup custom sounds: {err}");
        }
        return;
    }
    if let Err(err) = custom_sounds::cleanup_custom_sounds(app, &referenced_files) {
        log::warn!("Failed to cleanup custom sounds: {err}");
    }
}

fn collect_referenced_sound_files(app: &AppHandle, settings: &AppSettings) -> HashSet<String> {
    let base_path = match custom_sounds::sound_directory(app) {
        Ok(path) => path,
        Err(err) => {
            log::warn!("Failed to resolve sound directory: {err}");
            return HashSet::new();
        }
    };

    let mut files = HashSet::new();
    let mut push_path = |value: &Option<String>| {
        let Some(path) = value.as_ref() else { return; };
        let path = Path::new(path);
        if !path.starts_with(&base_path) {
            return;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            return;
        };
        files.insert(name.to_string());
    };

    push_path(&settings.default_sound);
    for friend_settings in settings.friend_settings.values() {
        push_path(&friend_settings.sound_override);
    }
    files
}