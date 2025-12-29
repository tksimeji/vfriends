use std::sync::Mutex;

use serde_json::Value;
use vrchatapi::apis;
use vrchatapi::apis::authentication_api::{
    get_current_user, verify2_fa, verify2_fa_email_code, verify_recovery_code,
};
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode};

const USER_AGENT: &str = "vfriends/authv2";

#[derive(Debug, Clone)]
pub struct AuthError {
    pub message: String,
    pub code: Option<String>,
}

impl AuthError {
    pub fn new(message: impl Into<String>, code: Option<String>) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }
}

pub struct AuthSession {
    pub config: Configuration,
    pub is_pending_two_factor: bool,
}

impl AuthSession {
    pub fn new() -> Self {
        let mut config = Configuration::default();
        config.user_agent = Some(String::from(USER_AGENT));
        Self {
            config,
            is_pending_two_factor: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

pub struct AuthState {
    pub session: Mutex<AuthSession>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(AuthSession::new()),
        }
    }
}

pub async fn fetch_user_or_two_factor(
    config: &Configuration,
) -> Result<EitherUserOrTwoFactor, AuthError> {
    get_current_user(config).await.map_err(map_api_error)
}

pub async fn dispatch_2fa_verification(
    config: &Configuration,
    code: &str,
    method: &str,
) -> Result<(), AuthError> {
    match method {
        "totp" => {
            let auth_code = TwoFactorAuthCode::new(code.to_string());
            verify2_fa(&config, auth_code)
                .await
                .map(|_| ())
                .map_err(map_api_error)
        }
        "emailOtp" => {
            let email_code = TwoFactorEmailCode::new(code.to_string());
            verify2_fa_email_code(&config, email_code)
                .await
                .map(|_| ())
                .map_err(map_api_error)
        }
        "otp" => {
            let auth_code = TwoFactorAuthCode::new(code.to_string());
            verify_recovery_code(&config, auth_code)
                .await
                .map(|_| ())
                .map_err(map_api_error)
        }
        _ => {
            Err(AuthError::new("未対応の2FA方式です。", None))
        }
    }
}

fn map_api_error<T>(error: apis::Error<T>) -> AuthError {
    match error {
        apis::Error::ResponseError(response) => {
            let message = extract_error_message(&response.content)
                .unwrap_or_else(|| format!("HTTP {}", response.status));
            AuthError::new(message, Some(response.status.as_u16().to_string()))
        }
        other => AuthError::new(other.to_string(), None),
    }
}

fn extract_error_message(content: &str) -> Option<String> {
    let value: Value = serde_json::from_str(content).ok()?;
    if let Some(message) = value.get("message").and_then(|v| v.as_str()) {
        return Some(message.to_string());
    }
    value
        .get("error")
        .and_then(|err| err.get("message"))
        .and_then(|v| v.as_str())
        .map(|m| m.to_string())
}
