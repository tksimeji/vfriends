use futures_util::StreamExt;
use reqwest::Url;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::USER_AGENT;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

const PIPELINE_URL: &str = "wss://pipeline.vrchat.cloud/";

pub async fn listen_raw<F>(
    auth_token: &str,
    user_agent: &str,
    mut on_payload: F,
) -> Result<(), String>
where
    F: FnMut(&str) + Send,
{
    let url = Url::parse_with_params(PIPELINE_URL, [("authToken", auth_token)])
        .map_err(|err| err.to_string())?;

    let mut request = url.into_client_request().map_err(|err| err.to_string())?;
    request.headers_mut().insert(
        USER_AGENT,
        HeaderValue::from_str(user_agent).map_err(|e| e.to_string())?,
    );

    let (mut socket, _) = connect_async(request)
        .await
        .map_err(|err| err.to_string())?;

    while let Some(message) = socket.next().await {
        match message {
            Ok(Message::Text(text)) => {
                on_payload(&text);
            }
            Ok(Message::Binary(binary)) => {
                if let Ok(text) = String::from_utf8(binary) {
                    on_payload(&text);
                }
            }
            Ok(_) => {}
            Err(err) => return Err(err.to_string()),
        }
    }

    Ok(())
}
