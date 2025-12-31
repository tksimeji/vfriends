use reqwest::{Client, Url};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

pub async fn fetch_image(image_url: Option<String>, user_agent: &str) -> Option<PathBuf> {
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

    let client = Client::builder().user_agent(user_agent).build().ok()?;
    let response = client.get(&url).send().await.ok()?;
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

fn extension_from_url(url: &Url) -> Option<&str> {
    let segment = url.path_segments()?.last()?;
    let ext = segment.rsplit('.').next()?;
    if ext == segment {
        None
    } else {
        Some(ext)
    }
}
