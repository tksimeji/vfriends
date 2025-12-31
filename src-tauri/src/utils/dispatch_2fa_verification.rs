use crate::utils::AppResult;
use vrchatapi::apis::authentication_api::{
    verify2_fa, verify2_fa_email_code, verify_recovery_code,
};
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::models;

pub async fn dispatch_2fa_verification(
    config: &Configuration,
    code: &str,
    method: &str,
) -> AppResult<()> {
    match method {
        "totp" => {
            let auth_code = models::TwoFactorAuthCode::new(code.to_string());
            verify2_fa(&config, auth_code)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        "emailOtp" => {
            let email_code = models::TwoFactorEmailCode::new(code.to_string());
            verify2_fa_email_code(&config, email_code)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        "otp" => {
            let auth_code = models::TwoFactorAuthCode::new(code.to_string());
            verify_recovery_code(&config, auth_code)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        _ => Err("Unsupported 2FA method.".to_string()),
    }
}
