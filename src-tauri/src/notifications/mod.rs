mod image;
mod platform;
mod service;
pub(crate) mod sound;

pub use service::{notify_friend_online, preview_sound, FriendOnlinePayload};

pub async fn fetch_cached_image_data_with_client(
    url: String,
    client: &reqwest::Client,
    user_agent: &str,
) -> Option<String> {
    image::fetch_image_data_with_client(Some(url), client, user_agent).await
}
