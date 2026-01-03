use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketMessage {
    #[serde(rename = "type")]
    pub kind: String,
    pub content: Value,
}

impl WebsocketMessage {
    pub fn from_raw_message(raw_message: &str) -> Option<Self> {
        serde_json::from_str(raw_message).ok()
    }

    pub fn content_as<C>(&self) -> Option<C>
    where
        C: DeserializeOwned,
    {
        match &self.content {
            Value::String(s) => serde_json::from_str::<C>(s).ok(),
            _ => serde_json::from_value::<C>(self.content.clone()).ok(),
        }
    }
    pub fn is_friend_online_message(&self) -> bool {
        self.kind == "friend-online"
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendOnlineEvent {
    pub user_id: String,
    pub platform: String,
    pub location: String,
    pub can_request_invite: bool,
    pub user: FriendOnlineUser,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendOnlineUser {
    pub display_name: String,
    #[serde(default)]
    pub profile_pic_override: String,
    #[serde(default)]
    pub user_icon: String,
    #[serde(default)]
    pub current_avatar_image_url: String,
    #[serde(default)]
    pub current_avatar_thumbnail_image_url: String,
}
