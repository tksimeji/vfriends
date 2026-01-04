use crate::auth::event::{AuthAction, AuthEvent};
use crate::vrchat_utils::AppResult;
use crate::websocket;
use keyring::Error as KeyringError;
use reqwest::cookie::Jar;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use vrchatapi::apis::authentication_api::{get_current_user, verify2_fa, verify2_fa_email_code, verify_recovery_code};
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::models;

pub struct AuthState {
    pub session: Mutex<AuthSession>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(AuthSession::new()),
        }
    }

    pub fn with_session<T>(&self, f: impl FnOnce(&AuthSession) -> T) -> AppResult<T> {
        let session = self
            .session
            .lock()
            .map_err(|_| String::from("Auth state lock poisoned."))?;
        Ok(f(&session))
    }

    pub fn with_session_mut<T>(&self, f: impl FnOnce(&mut AuthSession) -> T) -> AppResult<T> {
        let mut session = self
            .session
            .lock()
            .map_err(|_| String::from("Auth state lock poisoned."))?;
        Ok(f(&mut session))
    }

    pub async fn begin_auth_flow(
        &self,
        app: &AppHandle,
        username: String,
        password: String,
    ) -> AppResult<()> {
        AuthEvent::Started {
            action: AuthAction::Credentials,
        }
        .emit(&app);

        let config = self.set_basic_auth(username, password)?;
        match get_current_user(&config).await {
            Ok(models::EitherUserOrTwoFactor::CurrentUser(current_user)) => {
                self.finish_auth_flow(app, &current_user)?;
            }
            Ok(models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth)) => {
                self.with_session_mut(|session| {
                    session.is_pending_2fa = true;
                })?;

                AuthEvent::TwoFactorRequired {
                    methods: requires_auth.requires_two_factor_auth.clone(),
                    message: None,
                }
                .emit(&app);
            }
            Err(err) => {
                self.clear_basic_auth();
                AuthEvent::Failure {
                    message: err.to_string(),
                    code: None,
                }
                .emit(&app);
            }
        }

        Ok(())
    }

    pub fn finish_auth_flow(
        &self,
        app: &AppHandle,
        current_user: &models::CurrentUser,
    ) -> AppResult<()> {
        let (cookie_header, user_agent) = self.with_session_mut(|session| {
            session.is_pending_2fa = false;
            session.config.basic_auth = None;
            if let Err(err) = session.save_cookies() {
                log::warn!("Failed to save auth cookies to keychain: {err}");
            }
            (session.cookie_header(), session.config.user_agent.clone())
        })?;

        let websocket = app.state::<websocket::WebsocketState>();
        websocket.start_with_cookie_header(app, cookie_header, user_agent);

        AuthEvent::Success {
            user: current_user.clone(),
        }
        .emit(&app);

        Ok(())
    }

    pub async fn verify_two_factor(
        &self,
        app: &AppHandle,
        two_factor_code: String,
        two_factor_method: String,
    ) -> AppResult<()> {
        let code = two_factor_code.trim();
        let method = two_factor_method.trim();

        let (config, is_pending_2fa) =
            self.with_session(|session| (session.config.clone(), session.is_pending_2fa))?;

        if !is_pending_2fa {
            AuthEvent::Failure {
                message: String::from("2FA session not found. Please log in again"),
                code: None,
            }
            .emit(&app);
            return Ok(());
        }

        AuthEvent::Started {
            action: AuthAction::TwoFactor,
        }
        .emit(&app);

        if let Err(err) = dispatch_2fa_verification(&config, code, method).await {
            AuthEvent::Failure {
                message: err,
                code: None,
            }
            .emit(&app);
            return Ok(());
        }

        let login_result = match get_current_user(&config).await {
            Ok(login_result) => login_result,
            Err(err) => {
                AuthEvent::Failure {
                    message: err.to_string(),
                    code: None,
                }
                .emit(&app);
                return Ok(());
            }
        };

        match login_result {
            models::EitherUserOrTwoFactor::CurrentUser(current_user) => {
                self.finish_auth_flow(app, &current_user)?;
            }
            models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
                self.with_session_mut(|session| {
                    session.is_pending_2fa = true;
                })?;
                AuthEvent::TwoFactorRequired {
                    methods: requires_auth.requires_two_factor_auth.clone(),
                    message: None,
                }
                .emit(&app);
            }
        }

        Ok(())
    }

    pub async fn restore_session(&self, app: &AppHandle) -> AppResult<Option<models::CurrentUser>> {
        let (config, has_cookies) = self
            .with_session(|session| (session.config.clone(), session.cookie_header().is_some()))?;

        if !has_cookies {
            return Ok(None);
        }

        match get_current_user(&config).await {
            Ok(models::EitherUserOrTwoFactor::CurrentUser(current_user)) => {
                self.finish_auth_flow(app, &current_user)?;
                return Ok(Some(current_user));
            }
            Ok(models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(_)) => {
                let websocket = app.state::<websocket::WebsocketState>();
                websocket.stop();
                self.reset_session();
                clear_saved_cookies();
            }
            Err(err) => {
                if is_auth_error(&err.to_string()) {
                    let websocket = app.state::<websocket::WebsocketState>();
                    websocket.stop();
                    self.reset_session();
                    clear_saved_cookies();
                }
            }
        }

        Ok(None)
    }

    pub fn logout(&self, app: &AppHandle) -> AppResult<()> {
        let websocket = app.state::<websocket::WebsocketState>();
        websocket.stop();
        self.reset_session();
        clear_saved_cookies();
        AuthEvent::LoggedOut.emit(app);
        Ok(())
    }

    fn set_basic_auth(&self, username: String, password: String) -> AppResult<Configuration> {
        self.with_session_mut(|session| {
            session.reset();
            session.config.basic_auth = Some((username, Some(password)));
            session.config.clone()
        })
    }

    fn clear_basic_auth(&self) {
        self.with_session_mut(|session| {
            session.config.basic_auth = None;
        })
        .unwrap();
    }

    fn reset_session(&self) {
        let _ = self.with_session_mut(|session| {
            session.reset();
        });
    }
}

pub struct AuthSession {
    pub config: Configuration,
    pub cookie_jar: Arc<Jar>,
    pub is_pending_2fa: bool,
}

async fn dispatch_2fa_verification(
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

fn clear_saved_cookies() {
    match AuthSession::clear_saved_cookies() {
        Ok(()) | Err(KeyringError::NoEntry) => {}
        Err(err) => {
            log::warn!("Failed to clear auth cookies from keychain: {err}");
        }
    }
}

fn is_auth_error(message: &str) -> bool {
    message.contains("HTTP 401") || message.contains("HTTP 403")
}
