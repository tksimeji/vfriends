use log::warn;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use vrchatapi::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketMessage {
    #[serde(rename = "type")]
    pub kind: String,
    pub content: String,
}

impl WebsocketMessage {
    pub fn from_str(s: &str) -> Option<Self> {
        serde_json::from_str(s)
            .map_err(|err| {
                warn!("Failed to parse websocket message: {} (message={})", err, s);
                err
            })
            .ok()
    }

    pub fn content_as<E>(&self) -> Option<E>
    where
        E: DeserializeOwned,
    {
        serde_json::from_str::<E>(&self.content)
            .map_err(|err| {
                warn!(
                    "Failed to parse websocket content: {} (type={}; content={})",
                    err, self.kind, self.content
                );
                err
            })
            .ok()
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
    pub user: models::User,
}
