use serde::Serialize;
use tauri::{AppHandle, Emitter};
use vrchatapi::models;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthUser {
    pub id: String,
    pub display_name: String,
    pub username: Option<String>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AuthAction {
    Credentials,
    TwoFactor,
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AuthEvent {
    Started {
        action: AuthAction,
    },
    TwoFactorRequired {
        methods: Vec<String>,
        message: Option<String>,
    },
    Success {
        user: models::CurrentUser,
    },
    Failure {
        message: String,
        code: Option<String>,
    },
}

impl AuthEvent {
    pub fn emit(&self, app: &AppHandle) {
        let _ = app.emit("vrc:auth", self);
    }
}
