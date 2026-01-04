use crate::vrchat_utils::AppResult;
use rodio::{Decoder, OutputStream, Sink};
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
    let file_name = custom_sound_file_name(Path::new(name))?;
    let base_path = sound_directory(app)?;
    let path = base_path.join(file_name);
    std::fs::write(&path, bytes).map_err(|err| err.to_string())?;
    Ok(path)
}

fn sound_directory(app: &AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join(SOUND_DIR_NAME);
    std::fs::create_dir_all(&base).map_err(|err| err.to_string())?;
    Ok(base)
}

fn custom_sound_file_name(path: &Path) -> AppResult<String> {
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .filter(|ext| !ext.is_empty())
        .ok_or_else(|| String::from("Unsupported audio format."))?;

    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(String::from("Unsupported audio format."));
    }

    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("sound");
    let filtered: String = stem
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect();
    let safe_stem = if filtered.is_empty() {
        String::from("sound")
    } else {
        filtered
    };
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_millis())
        .unwrap_or(0);

    Ok(format!("{safe_stem}-{suffix}.{ext}"))
}
