use serde_json::Value;
use vrchatapi::apis;
use vrchatapi::apis::authentication_api::{
    get_current_user, verify2_fa, verify2_fa_email_code, verify_recovery_code,
};
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode};

pub async fn fetch_user_or_two_factor(
    config: &Configuration,
) -> Result<EitherUserOrTwoFactor, String> {
    get_current_user(config).await.map_err(map_api_error)
}

pub async fn dispatch_2fa_verification(
    config: &Configuration,
    code: &str,
    method: &str,
) -> Result<(), String> {
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
        _ => Err("Unsupported 2FA method.".to_string()),
    }
}

fn map_api_error<T>(error: apis::Error<T>) -> String {
    match error {
        apis::Error::ResponseError(response) => {
            let message = extract_error_message(&response.content)
                .unwrap_or_else(|| format!("HTTP {}", response.status));
            message
        }
        other => other.to_string(),
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
