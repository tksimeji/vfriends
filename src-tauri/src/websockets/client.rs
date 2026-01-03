use crate::notifier::notify_friend_online;
use crate::utils::AppResult;
use crate::websockets::types::{FriendOnlineEvent, WebsocketMessage};
use futures_util::StreamExt;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use tauri::AppHandle;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

const PIPELINE_URL: &str = "wss://pipeline.vrchat.cloud/";

pub async fn listen(app: &AppHandle, auth_token: &str, user_agent: &str) -> AppResult<()> {
    eprintln!("Websocket: connecting to pipeline...");
    listen_raw(auth_token, user_agent, |raw| {
        let message = match WebsocketMessage::from_raw_message(raw) {
            Some(message) => message,
            None => return,
        };

        // friend-online
        if message.is_friend_online_message() {
            let event = match message.content_as::<FriendOnlineEvent>() {
                Some(event) => event,
                None => {
                    if let Ok(raw) = serde_json::to_string(&message.content) {
                        eprintln!("Websocket: friend-online raw: {}", raw);
                    }
                    return;
                }
            };
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(err) = notify_friend_online(&app, event).await {
                    eprintln!("Failed to notify friend online: {}", err);
                }
            });
            return;
        }
    })
    .await
}

async fn listen_raw<L>(auth_token: &str, user_agent: &str, mut listener: L) -> AppResult<()>
where
    L: FnMut(&str) + Send,
{
    let url = Url::parse_with_params(PIPELINE_URL, [("authToken", auth_token)])
        .map_err(|err| err.to_string())?;

    let mut request = url.into_client_request().map_err(|err| err.to_string())?;
    let user_agent_value =
        HeaderValue::from_str(user_agent).map_err(|err| err.to_string())?;
    request.headers_mut().insert(USER_AGENT, user_agent_value);

    let (mut socket, _) = tokio_tungstenite::connect_async(request)
        .await
        .map_err(|err| err.to_string())?;
    eprintln!("Websocket: connected.");

    while let Some(message) = socket.next().await {
        match message {
            Ok(Message::Text(text)) => {
                listener(&text);
            }
            Ok(Message::Binary(binary)) => {
                if let Ok(text) = String::from_utf8(binary) {
                    listener(&text);
                }
            }
            Ok(_) => {}
            Err(err) => return Err(err.to_string()),
        }
    }

    eprintln!("Websocket: connection closed.");
    Ok(())
}
