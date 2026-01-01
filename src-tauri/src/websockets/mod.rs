use crate::notifications::notify_friend_online;
use serde_json::Value;
use tauri::AppHandle;

mod client;
mod parser;

#[derive(Debug, Clone)]
pub struct FriendOnline {
    pub id: Option<String>,
    pub display_name: String,
    pub image_url: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PipelineEvent {
    FriendOnline(FriendOnline),
    Other { kind: String, content: Value },
}

pub async fn listen(app: &AppHandle, auth_token: &str, user_agent: &str) -> Result<(), String> {
    client::listen_raw(auth_token, user_agent, |payload| {
        if let Some(event) = parser::parse_message(payload) {
            handle_event(app, user_agent, event);
        }
    })
    .await
}

fn handle_event(app: &AppHandle, user_agent: &str, event: PipelineEvent) {
    match event {
        PipelineEvent::FriendOnline(friend) => {
            let app = app.clone();
            let agent = user_agent.to_string();
            tauri::async_runtime::spawn(async move {
                notify_friend_online(
                    &app,
                    friend.id,
                    &friend.display_name,
                    friend.image_url,
                    &agent,
                )
                .await;
            });
        }
        PipelineEvent::Other { .. } => {}
    }
}
