use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

pub fn play_sound_file(path: PathBuf) {
    std::thread::spawn(move || {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open sound file {}: {err}", path.display());
                return;
            }
        };

        let reader = BufReader::new(file);
        let decoder = match Decoder::new(reader) {
            Ok(decoder) => decoder,
            Err(err) => {
                eprintln!("Failed to decode sound file {}: {err}", path.display());
                return;
            }
        };

        let (_stream, handle) = match OutputStream::try_default() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Failed to open audio output: {err}");
                return;
            }
        };

        let sink = match Sink::try_new(&handle) {
            Ok(sink) => sink,
            Err(err) => {
                eprintln!("Failed to create audio sink: {err}");
                return;
            }
        };

        sink.append(decoder);
        sink.sleep_until_end();
    });
}

pub fn store_sound_file(app: &AppHandle, name: &str, bytes: &[u8]) -> Result<PathBuf, String> {
    let file_name = sanitize_file_name(name)?;
    let base = app
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join("notification_sounds");
    std::fs::create_dir_all(&base).map_err(|err| err.to_string())?;

    let path = base.join(file_name);
    std::fs::write(&path, bytes).map_err(|err| err.to_string())?;
    Ok(path)
}

fn sanitize_file_name(name: &str) -> Result<String, String> {
    let path = Path::new(name);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let allowed = ["mp3", "wav", "ogg", "flac", "m4a"];
    if !allowed.contains(&extension.as_str()) {
        return Err("unsupported audio format".to_string());
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
        "sound".to_string()
    } else {
        filtered
    };

    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_millis())
        .unwrap_or(0);

    Ok(format!("{safe_stem}-{suffix}.{extension}"))
}
