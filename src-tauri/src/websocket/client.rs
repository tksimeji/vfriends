use crate::notifier::notify_friend_online;
use crate::vrchat_utils::{AppResult};
use crate::websocket::refresh_friends::emit_refresh_friends;
use crate::websocket::types::{FriendOnlineEvent, WebsocketMessage};
use futures_util::StreamExt;
use log::{error, info, warn};
use reqwest::Url;
use tauri::{AppHandle};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

const SERVER_URL: &str = "wss://pipeline.vrchat.cloud/";

pub async fn listen(app: &AppHandle, auth_token: &str, user_agent: &str) -> AppResult<()> {
    listen_raw(auth_token, user_agent, |raw| {
        let message = match WebsocketMessage::from_str(raw) {
            Some(message) => message,
            None => return,
        };

        // Send online toast notification
        if message.is_friend_online_message() {
            if let Some(event) = message.content_as::<FriendOnlineEvent>() {
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = notify_friend_online(&app, event).await {
                        error!("Failed to notify friend online: {}", err);
                    }
                });
            }
        }

        // Prompts the front end to update friends view
        if message.is_friend_message() {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(err) = emit_refresh_friends(&app).await {
                    error!("Failed to refresh friends: {}", err);
                }
            });
        }
    })
    .await
}

async fn listen_raw<L>(auth_token: &str, user_agent: &str, mut listener: L) -> AppResult<()>
where
    L: FnMut(&str) + Send,
{
    info!("Connecting to '{}'...", SERVER_URL);

    let url = Url::parse_with_params(SERVER_URL, [("authToken", auth_token)])
        .map_err(|err| err.to_string())?;

    let mut request = url.into_client_request().map_err(|err| err.to_string())?;

    let user_agent_value = HeaderValue::from_str(user_agent).map_err(|err| err.to_string())?;
    request
        .headers_mut()
        .insert(reqwest::header::USER_AGENT, user_agent_value);

    let (mut stream, _) = tokio_tungstenite::connect_async(request)
        .await
        .map_err(|err| err.to_string())?;

    info!("Connected to '{}'.", SERVER_URL);

    while let Some(message) = stream.next().await {
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

    warn!("Websocket connection was closed.");

    Ok(())
}
