use crate::vrchat_utils::AppResult;
use rodio::{Decoder, OutputStream, Sink, Source};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, OnceLock};
use std::time::Duration;
use tauri::{AppHandle, Manager};

const SOUND_DIR_NAME: &str = "Sounds";
const ALLOWED_EXTENSIONS: [&str; 5] = ["mp3", "wav", "ogg", "flac", "m4a"];
const MAX_SOUND_SECONDS: u64 = 15;

static SOUND_QUEUE: OnceLock<mpsc::Sender<PathBuf>> = OnceLock::new();

fn sound_sender() -> mpsc::Sender<PathBuf> {
    SOUND_QUEUE
        .get_or_init(|| {
            let (tx, rx) = mpsc::channel::<PathBuf>();
            std::thread::spawn(move || {
                let (stream, handle) = match OutputStream::try_default() {
                    Ok(value) => value,
                    Err(err) => {
                        log::warn!("Failed to open audio output: {err}");
                        return;
                    }
                };
                let _stream = stream;
                for path in rx {
                    let file = match File::open(&path) {
                        Ok(file) => file,
                        Err(err) => {
                            log::warn!("Failed to open sound file '{}': {err}", path.display());
                            continue;
                        }
                    };
                    let reader = BufReader::new(file);
                    let decoder = match Decoder::new(reader) {
                        Ok(decoder) => decoder,
                        Err(err) => {
                            log::warn!("Failed to decode sound file '{}': {err}", path.display());
                            continue;
                        }
                    };
                    let sink = match Sink::try_new(&handle) {
                        Ok(sink) => sink,
                        Err(err) => {
                            log::warn!("Failed to create audio sink: {err}");
                            continue;
                        }
                    };
                    sink.append(decoder);
                    sink.sleep_until_end();
                }
            });
            tx
        })
        .clone()
}

pub fn validate_sound_path(path: &str) -> AppResult<()> {
    let path = Path::new(path);
    let _ = sound_extension(path)?;
    let file = File::open(path).map_err(|err| format!("Failed to open sound file: {err}"))?;
    let reader = BufReader::new(file);
    let decoder = Decoder::new(reader).map_err(|_| String::from("Unsupported audio format."))?;
    ensure_sound_duration(decoder)
}

pub fn play_custom_sound(path: PathBuf) {
    let sender = sound_sender();
    if let Err(err) = sender.send(path) {
        log::warn!("Failed to queue sound playback: {err}");
    }
}

pub fn store_custom_sound(app: &AppHandle, name: &str, bytes: &[u8]) -> Result<PathBuf, String> {
    validate_sound_bytes(name, bytes)?;
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

pub fn store_custom_sound_from_path(app: &AppHandle, path: &Path) -> Result<PathBuf, String> {
    let _ = validate_sound_path(path.to_string_lossy().as_ref())?;
    let ext = sound_extension(path)?;
    let base_path = sound_directory(app)?;
    if path.starts_with(&base_path) && path.is_file() {
        return Ok(path.to_path_buf());
    }
    let hash = hash_file(path)?;
    let file_name = format!("{hash}.{ext}");
    let target = base_path.join(file_name);
    if !target.exists() {
        std::fs::copy(path, &target).map_err(|err| err.to_string())?;
    }
    Ok(target)
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

fn validate_sound_bytes(name: &str, bytes: &[u8]) -> AppResult<()> {
    let _ = sound_extension(Path::new(name))?;
    let cursor = Cursor::new(bytes.to_vec());
    let reader = BufReader::new(cursor);
    let decoder = Decoder::new(reader).map_err(|_| String::from("Unsupported audio format."))?;
    ensure_sound_duration(decoder)
}

fn ensure_sound_duration<R: Read + Seek + Send + Sync + 'static>(
    mut decoder: Decoder<R>,
) -> AppResult<()> {
    let max_duration = Duration::from_secs(MAX_SOUND_SECONDS);
    if let Some(duration) = decoder.total_duration() {
        if duration >= max_duration {
            return Err(String::from("Sound must be shorter than 15 seconds."));
        }
        return Ok(());
    }

    let channels = decoder.channels().max(1) as u64;
    let sample_rate = decoder.sample_rate().max(1) as u64;
    let max_samples = sample_rate * channels * MAX_SOUND_SECONDS;
    let mut samples = 0u64;
    while let Some(_sample) = decoder.next() {
        samples += 1;
        if samples >= max_samples {
            return Err(String::from("Sound must be shorter than 15 seconds."));
        }
    }
    Ok(())
}

fn hash_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{:x}", digest)
}

fn hash_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|err| err.to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let read = file.read(&mut buffer).map_err(|err| err.to_string())?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}