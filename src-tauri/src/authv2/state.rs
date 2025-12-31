use crate::authv2::events::{AuthAction, AuthEvent};
use crate::utils::{dispatch_2fa_verification, AppResult};
use keyring::Error as KeyringError;
use reqwest::cookie::Jar;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use vrchatapi::apis::authentication_api::get_current_user;
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::models;

pub struct AuthSession {
    pub config: Configuration,
    pub cookie_jar: Arc<Jar>,
    pub is_pending_2fa: bool,
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

    pub fn with_session<T>(&self, f: impl FnOnce(&AuthSession) -> T) -> AppResult<T> {
        let session = self
            .session
            .lock()
            .map_err(|_| String::from("Failed to lock auth state (Mutex poisoned)"))?;
        Ok(f(&session))
    }

    pub fn with_session_mut<T>(&self, f: impl FnOnce(&mut AuthSession) -> T) -> AppResult<T> {
        let mut session = self
            .session
            .lock()
            .map_err(|_| String::from("Failed to lock auth state (Mutex poisoned)"))?;
        Ok(f(&mut session))
    }

    pub fn set_basic_auth(&self, username: String, password: String) -> AppResult<Configuration> {
        self.with_session_mut(|session| {
            session.reset();
            session.config.basic_auth = Some((username, Some(password)));
            session.config.clone()
        })
    }

    pub fn clear_basic_auth(&self) {
        self.with_session_mut(|session| {
            session.config.basic_auth = None;
        })
        .unwrap();
    }

    pub fn finish_auth(
        &self,
        app: &AppHandle,
        current_user: &models::CurrentUser,
    ) -> AppResult<()> {
        self.with_session_mut(|session| {
            session.is_pending_2fa = false;
            session.config.basic_auth = None;
            if let Err(err) = session.save_cookies() {
                eprintln!("Failed to save auth cookies to keychain: {err}");
            }
        })?;

        AuthEvent::Success {
            user: current_user.clone(),
        }
        .emit(&app);

        Ok(())
    }

    pub async fn begin_auth(
        &self,
        app: &AppHandle,
        username: String,
        password: String,
    ) -> AppResult<()> {
        let username = username.trim().to_string();
        let password = password;

        if username.is_empty() || password.is_empty() {
            emit_failure(app, "Please enter your username and password.");
            return Ok(());
        }

        emit_started(app, AuthAction::Credentials);

        let config = self.set_basic_auth(username, password)?;

        let login_result = match get_current_user(&config).await {
            Ok(login_result) => login_result,
            Err(err) => {
                self.clear_basic_auth();
                emit_failure(app, err.to_string());
                return Ok(());
            }
        };

        match login_result {
            models::EitherUserOrTwoFactor::CurrentUser(current_user) => {
                self.finish_auth(app, &current_user)?;
            }
            models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
                self.with_session_mut(|session| {
                    session.is_pending_2fa = true;
                })?;
                emit_two_factor_required(
                    app,
                    requires_auth.requires_two_factor_auth.clone(),
                    "Please enter your 2FA code",
                );
            }
        }

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

        if code.is_empty() {
            emit_failure(app, "Please enter your 2FA code");
            return Ok(());
        }

        if method.is_empty() {
            emit_failure(app, "Please select a 2FA method");
            return Ok(());
        }

        let (config, is_pending_2fa) =
            self.with_session(|session| (session.config.clone(), session.is_pending_2fa))?;

        if !is_pending_2fa {
            emit_failure(app, "2FA session not found. Please log in again");
            return Ok(());
        }

        emit_started(app, AuthAction::TwoFactor);

        if let Err(error) = dispatch_2fa_verification(&config, code, method).await {
            emit_failure(app, error);
            return Ok(());
        }

        let login_result = match get_current_user(&config).await {
            Ok(login_result) => login_result,
            Err(error) => {
                emit_failure(app, error.to_string());
                return Ok(());
            }
        };

        match login_result {
            models::EitherUserOrTwoFactor::CurrentUser(current_user) => {
                self.finish_auth(app, &current_user)?;
            }
            models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
                self.with_session_mut(|session| {
                    session.is_pending_2fa = true;
                })?;
                emit_two_factor_required(
                    app,
                    requires_auth.requires_two_factor_auth.clone(),
                    "Another 2FA method is required",
                );
            }
        }

        Ok(())
    }

    pub async fn restore_session(&self, app: &AppHandle) -> AppResult<()> {
        let (config, has_cookies) = self
            .with_session(|session| (session.config.clone(), session.cookie_header().is_some()))?;

        if !has_cookies {
            return Ok(());
        }

        let login_result = match get_current_user(&config).await {
            Ok(login_result) => login_result,
            Err(error) => {
                if is_auth_error(&error.to_string()) {
                    self.reset_session();
                    clear_saved_cookies();
                }
                return Ok(());
            }
        };

        match login_result {
            models::EitherUserOrTwoFactor::CurrentUser(current_user) => {
                self.finish_auth(app, &current_user)?;
            }
            models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
                self.reset_session();
                clear_saved_cookies();
            }
        }

        Ok(())
    }

    fn reset_session(&self) {
        let _ = self.with_session_mut(|session| {
            session.reset();
        });
    }
}

fn clear_saved_cookies() {
    match AuthSession::clear_saved_cookies() {
        Ok(()) | Err(KeyringError::NoEntry) => {}
        Err(err) => {
            eprintln!("Failed to clear auth cookies from keychain: {err}");
        }
    }
}

fn is_auth_error(message: &str) -> bool {
    message.contains("HTTP 401") || message.contains("HTTP 403")
}

fn emit_started(app: &AppHandle, action: AuthAction) {
    AuthEvent::Started { action }.emit(app);
}

fn emit_failure(app: &AppHandle, message: impl Into<String>) {
    AuthEvent::Failure {
        message: message.into(),
        code: None,
    }
    .emit(app);
}

fn emit_two_factor_required(app: &AppHandle, methods: Vec<String>, message: &str) {
    AuthEvent::TwoFactorRequired {
        methods,
        message: Some(message.to_string()),
    }
    .emit(app);
}
