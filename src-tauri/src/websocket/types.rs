use log::warn;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use vrchatapi::models;

/// Represents the message format provided by the VRChat API's websocket API.
/// See also, [vrchat.community](https://vrchat.community/websocket).
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

    pub fn is_friend_add_message(&self) -> bool {
        self.kind == "friend-add"
    }

    pub fn is_friend_delete_message(&self) -> bool {
        self.kind == "friend-delete"
    }

    pub fn is_friend_active_message(&self) -> bool {
        self.kind == "friend-active"
    }

    pub fn is_friend_online_message(&self) -> bool {
        self.kind == "friend-online"
    }

    pub fn is_friend_offline_message(&self) -> bool {
        self.kind == "friend-offline"
    }

    pub fn is_friend_update_message(&self) -> bool {
        self.kind == "friend-update"
    }

    pub fn is_friend_location_message(&self) -> bool {
        self.kind == "friend-location"
    }

    pub fn is_friend_message(&self) -> bool {
        self.is_friend_add_message()
            || self.is_friend_delete_message()
            || self.is_friend_active_message()
            || self.is_friend_online_message()
            || self.is_friend_offline_message()
            || self.is_friend_update_message()
            || self.is_friend_location_message()
    }
}

/// This event is sent when the user has either accepted a friend request, or has had one of their friend requests accepted.
/// See also, [vrchat.community](https://vrchat.community/websocket#friend-add)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendAddEvent {
    pub user_id: String,
    pub user: models::User,
}

/// This event is sent when the user has either been removed as a friend, or has removed someone else as a friend.
/// See also, [vrchat.community](https://vrchat.community/websocket#friend-delete)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendDeleteEvent {
    pub user_id: String,
}

/// This event is sent when one of the user's friends is active on the website.
/// See also, [vrchat.community](https://vrchat.community/websocket#friend-active)
#[derive(Debug, Serialize, Deserialize)]
pub struct FriendActiveEvent {
    #[serde(rename = "userid", alias = "userId")]
    pub user_id: String,
    pub platform: Option<String>,
    pub user: models::User,
}

/// This event is sent when one of the user's friends has gone online in-game.
/// See also, [vrchat.community](https://vrchat.community/websocket#friend-online)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendOnlineEvent {
    pub user_id: String,
    pub platform: String,
    pub location: String,
    pub can_request_invite: bool,
    pub user: models::User,
}

/// This event is sent when one of the user's friend has gone offline.
/// See also, [vrchat.community](https://vrchat.community/websocket#friend-offline)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendOfflineEvent {
    pub user_id: String,
    pub platform: Option<String>,
}

/// This event is sent when something about one of the user's friends profile has changed.
/// See also, [vrchat.community](https://vrchat.community/websocket#friend-update)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendUpdateEvent {
    pub user_id: String,
    pub user: models::User,
}

/// This event is sent when one of the user's friends has changed instances.
/// See also, [vrchat.community](https://vrchat.community/websocket#friend-location)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendLocationEvent {
    pub user_id: String,
    pub location: String,
    pub traveling_to_location: Option<String>,
    pub world_id: Option<String>,
    pub can_request_invite: Option<bool>,
    pub user: models::User,
}
