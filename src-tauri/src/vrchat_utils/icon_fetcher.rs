use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use mime_guess::MimeGuess;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Url};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use vrchatapi::models;

const TEMP_DIR_NAME: &str = "VFriends.UserIconCache";
const FILE_PREFIX: &str = "Icon.";

pub async fn fetch_user_icon_data_uri(
    url: &str,
    client: &Client,
    user_agent: &str,
) -> Option<String> {
    let (path, content_type) = ensure_cached_icon_file(&url, client, user_agent).await?;
    let bytes = std::fs::read(&path).ok()?;
    let encoded = BASE64.encode(&bytes);
    Some(format!("data:{content_type};base64,{encoded}"))
}

pub async fn fetch_user_icon_file_uri(
    url: &str,
    client: &Client,
    user_agent: &str,
) -> Option<String> {
    let (path, _) = ensure_cached_icon_file(url, client, user_agent).await?;
    Url::from_file_path(path)
        .ok()
        .map(|url| url.to_string())
}

pub fn resolve_user_icon_url(user: &models::User) -> Option<String> {
    let not_empty: fn(&str) -> Option<&str> = |s: &str| (!s.is_empty()).then_some(s);
    not_empty(&user.profile_pic_override)
        .or_else(|| not_empty(&user.user_icon))
        .or_else(|| not_empty(&user.current_avatar_image_url))
        .or_else(|| not_empty(&user.current_avatar_thumbnail_image_url))
        .map(|url| url.to_string())
}

async fn download_bytes(
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

async fn ensure_cached_icon_file(
    url: &str,
    client: &Client,
    user_agent: &str,
) -> Option<(PathBuf, String)> {
    let dir = icon_cache_dir()?;
    let base = hashed_file_stem(url);

    if let Some(path) = find_cached_file(&dir, &base) {
        let content_type = MimeGuess::from_path(&path)
            .first_raw()
            .map(|value| value.to_string())
            .unwrap_or_else(|| String::from("application/octet-stream"));
        return Some((path, content_type));
    }

    let (bytes, content_type) = download_bytes(url, client, user_agent).await?;
    let resolved_type = content_type
        .or_else(|| {
            MimeGuess::from_path(url)
                .first_raw()
                .map(|value| value.to_string())
        })
        .unwrap_or_else(|| String::from("application/octet-stream"));

    let ext = mime_guess::get_mime_extensions_str(&resolved_type)
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
        .unwrap_or(String::from("img"));

    let path = dir.join(format!("{base}.{ext}"));
    std::fs::write(&path, &bytes).ok()?;
    Some((path, resolved_type))
}

fn icon_cache_dir() -> Option<PathBuf> {
    let dir = std::env::temp_dir().join(TEMP_DIR_NAME);
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

fn hashed_file_stem(url: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{FILE_PREFIX}{hash}")
}

fn find_cached_file(dir: &Path, base: &str) -> Option<PathBuf> {
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
