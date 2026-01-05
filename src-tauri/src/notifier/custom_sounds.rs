use crate::vrchat_utils::AppResult;
use rodio::{Decoder, OutputStream, Sink};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

const SOUND_DIR_NAME: &str = "Sounds";
const ALLOWED_EXTENSIONS: [&str; 5] = ["mp3", "wav", "ogg", "flac", "m4a"];

pub fn play_custom_sound(path: PathBuf) {
    std::thread::spawn(move || {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                log::warn!("Failed to open sound file '{}': {err}", path.display());
                return;
            }
        };

        let reader = BufReader::new(file);
        let decoder = match Decoder::new(reader) {
            Ok(decoder) => decoder,
            Err(err) => {
                log::warn!("Failed to decode sound file '{}': {err}", path.display());
                return;
            }
        };

        let (_stream, handle) = match OutputStream::try_default() {
            Ok(value) => value,
            Err(err) => {
                log::warn!("Failed to open audio output: {err}");
                return;
            }
        };
        let sink = match Sink::try_new(&handle) {
            Ok(sink) => sink,
            Err(err) => {
                log::warn!("Failed to create audio sink: {err}");
                return;
            }
        };

        sink.append(decoder);
        sink.sleep_until_end();
    });
}

pub fn store_custom_sound(app: &AppHandle, name: &str, bytes: &[u8]) -> Result<PathBuf, String> {
    let ext = sound_extension(Path::new(name))?;
    let hash = hash_bytes(bytes);
    let file_name = format!("{hash}.{ext}");
    let base_path = sound_directory(app)?;
    let path = base_path.join(file_name);
    if !path.exists() {
        std::fs::write(&path, bytes).map_err(|err| err.to_string())?;
    }
    Ok(path)
}

pub(crate) fn sound_directory(app: &AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join(SOUND_DIR_NAME);
    std::fs::create_dir_all(&base).map_err(|err| err.to_string())?;
    Ok(base)
}

pub fn cleanup_custom_sounds(
    app: &AppHandle,
    referenced_files: &std::collections::HashSet<String>,
) -> Result<(), String> {
    let base_path = sound_directory(app)?;
    let entries = std::fs::read_dir(&base_path).map_err(|err| err.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if referenced_files.contains(name) {
            continue;
        }
        if let Err(err) = std::fs::remove_file(&path) {
            log::warn!("Failed to remove unused sound '{}': {err}", path.display());
        }
    }
    Ok(())
}

fn sound_extension(path: &Path) -> AppResult<String> {
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .filter(|ext| !ext.is_empty())
        .ok_or_else(|| String::from("Unsupported audio format."))?;

    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(String::from("Unsupported audio format."));
    }

    Ok(ext)
}

fn hash_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{:x}", digest)
}
