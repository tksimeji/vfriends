use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use mime_guess::{get_mime_extensions_str, MimeGuess};
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::Client;
use reqwest::Url;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

const TEMP_DIR_NAME: &str = "vfriends-toast";
const FILE_PREFIX: &str = "vfriends-toast-";

pub async fn fetch_cached_image_data_with_client(
    url: String,
    client: &Client,
    user_agent: &str,
) -> Option<String> {
    let dir = temp_dir()?;
    let base = hashed_base_name(&url);

    if let Some(path) = find_existing_file(&dir, &base) {
        let bytes = std::fs::read(&path).ok()?;
        let content_type = guess_content_type(path.to_string_lossy().as_ref())
            .unwrap_or_else(|| "application/octet-stream".to_string());
        let encoded = BASE64.encode(&bytes);
        return Some(format!("data:{};base64,{}", content_type, encoded));
    }

    let (bytes, content_type) = fetch_bytes(&url, client, user_agent).await?;
    let resolved_type = content_type
        .or_else(|| guess_content_type(&url))
        .unwrap_or_else(|| "application/octet-stream".to_string());
    let extension = resolve_extension(&resolved_type, &url);
    let path = dir.join(format!("{}.{}", base, extension));
    std::fs::write(&path, &bytes).ok()?;

    let encoded = BASE64.encode(&bytes);
    Some(format!("data:{};base64,{}", resolved_type, encoded))
}

pub async fn fetch_cached_image_file_with_client(
    url: &str,
    client: &Client,
    user_agent: &str,
) -> Option<String> {
    let dir = temp_dir()?;
    let base = hashed_base_name(url);

    if let Some(path) = find_existing_file(&dir, &base) {
        return Some(file_uri(&path));
    }

    let (bytes, content_type) = fetch_bytes(url, client, user_agent).await?;
    let resolved_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());
    let extension = resolve_extension(&resolved_type, url);
    let path = dir.join(format!("{}.{}", base, extension));
    std::fs::write(&path, &bytes).ok()?;
    Some(file_uri(&path))
}

async fn fetch_bytes(
    url: &str,
    client: &Client,
    user_agent: &str,
) -> Option<(Vec<u8>, Option<String>)> {
    let response = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    let bytes = response.bytes().await.ok()?.to_vec();
    Some((bytes, content_type))
}

fn guess_content_type(url: &str) -> Option<String> {
    MimeGuess::from_path(url)
        .first_raw()
        .map(|value| value.to_string())
}

fn resolve_extension(content_type: &str, url: &str) -> String {
    get_mime_extensions_str(content_type)
        .and_then(|exts| exts.first().copied())
        .map(|ext| ext.to_string())
        .or_else(|| {
            Url::parse(url)
                .ok()
                .and_then(|parsed| {
                    Path::new(parsed.path())
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.to_string())
                })
        })
        .unwrap_or_else(|| "img".to_string())
}

fn temp_dir() -> Option<PathBuf> {
    let dir = std::env::temp_dir().join(TEMP_DIR_NAME);
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

fn hashed_base_name(url: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{}{}", FILE_PREFIX, hash)
}

fn find_existing_file(dir: &Path, base: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            if file_name.starts_with(base) {
                return Some(path);
            }
        }
    }
    None
}

fn file_uri(path: &Path) -> String {
    Url::from_file_path(path)
        .ok()
        .map(|url| url.to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}
