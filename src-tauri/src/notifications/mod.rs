mod image;
mod platform;

#[derive(Debug, Clone)]
pub struct FriendOnlinePayload {
    pub display_name: String,
    pub title: String,
    pub body: String,
    pub image_url: Option<String>,
}

impl FriendOnlinePayload {
    pub fn new(display_name: &str, image_url: Option<String>) -> Self {
        let title = String::from("Friend Online");
        let body = format!("{display_name} is online");
        Self {
            display_name: display_name.to_string(),
            title,
            body,
            image_url,
        }
    }
}

pub async fn notify_friend_online(
    app: &tauri::AppHandle,
    display_name: &str,
    image_url: Option<String>,
    user_agent: &str,
) {
    let payload = FriendOnlinePayload::new(display_name, image_url);
    platform::notify(app, &payload, user_agent).await;
}
