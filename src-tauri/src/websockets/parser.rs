use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct PipelineMessage {
    #[serde(rename = "type")]
    kind: String,
    content: Value,
}

pub fn parse_message(payload: &str) -> Option<crate::websockets::PipelineEvent> {
    let message: PipelineMessage = serde_json::from_str(payload).ok()?;
    let content = parse_content(message.content);

    if message.kind == "friend-online" {
        let display_name = extract_display_name(&content);
        let image_url = extract_image_url(&content);
        return Some(crate::websockets::PipelineEvent::FriendOnline(
            crate::websockets::FriendOnline {
                display_name,
                image_url,
            },
        ));
    }

    Some(crate::websockets::PipelineEvent::Other {
        kind: message.kind,
        content,
    })
}

fn parse_content(content: Value) -> Value {
    match content {
        Value::String(raw) => serde_json::from_str(&raw).unwrap_or(Value::String(raw)),
        other => other,
    }
}

fn extract_display_name(content: &Value) -> String {
    extract_string(content, &["user", "displayName"])
        .or_else(|| extract_string(content, &["displayName"]))
        .unwrap_or_else(|| "Friend".to_string())
}

fn extract_image_url(content: &Value) -> Option<String> {
    let candidates: &[&[&str]] = &[
        &["user", "profilePicOverride"],
        &["user", "userIcon"],
        &["user", "currentAvatarImageUrl"],
        &["user", "currentAvatarThumbnailImageUrl"],
        &["profilePicOverride"],
        &["userIcon"],
        &["currentAvatarImageUrl"],
        &["currentAvatarThumbnailImageUrl"],
    ];

    for path in candidates {
        if let Some(value) = extract_string(content, path) {
            if !value.is_empty() {
                return Some(value);
            }
        }
    }

    None
}

fn extract_string(content: &Value, path: &[&str]) -> Option<String> {
    let mut current = content;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str().map(|value| value.to_string())
}
