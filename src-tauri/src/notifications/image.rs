use base64::{engine::general_purpose, Engine as _};
use mime_guess::MimeGuess;
use reqwest::{header::USER_AGENT, Client, Url};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

pub async fn fetch_image(image_url: Option<String>, user_agent: &str) -> Option<PathBuf> {
    let client = Client::builder().user_agent(user_agent).build().ok()?;
    fetch_image_with_client(image_url, &client, user_agent).await
}

pub async fn fetch_image_with_client(
    image_url: Option<String>,
    client: &Client,
    user_agent: &str,
) -> Option<PathBuf> {
    let url = image_url?;
    let parsed_url = Url::parse(&url).ok();
    let ext = parsed_url
        .as_ref()
        .and_then(extension_from_url)
        .unwrap_or("png");
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();

    let mut path = std::env::temp_dir();
    path.push(format!("vfriends-toast-{hash}.{ext}"));
    if path.exists() {
        return Some(path);
    }

    let response = client
        .get(&url)
        .header(USER_AGENT, user_agent)
        .send()
        .await
        .ok()?;
    if !response.status().is_success() {
        return None;
    }
    let bytes = response.bytes().await.ok()?;
    if bytes.is_empty() {
        return None;
    }

    tokio::fs::write(&path, &bytes).await.ok()?;
    Some(path)
}

pub async fn fetch_image_data_with_client(
    image_url: Option<String>,
    client: &Client,
    user_agent: &str,
) -> Option<String> {
    let path = fetch_image_with_client(image_url, client, user_agent).await?;
    let bytes = std::fs::read(&path).ok()?;
    let mime = MimeGuess::from_path(&path).first_or_octet_stream();
    let encoded = general_purpose::STANDARD.encode(bytes);
    Some(format!("data:{};base64,{}", mime, encoded))
}

fn extension_from_url(url: &Url) -> Option<&str> {
    let segment = url.path_segments()?.last()?;
    let ext = segment.rsplit('.').next()?;
    if ext == segment {
        None
    } else {
        Some(ext)
    }
}
