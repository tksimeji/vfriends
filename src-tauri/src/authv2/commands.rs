use crate::authv2::events::{AuthAction, AuthEvent, AuthUser};
use crate::authv2::service::{clear_persisted_cookies, AuthSession, AuthState};
use crate::utils::{dispatch_2fa_verification, fetch_user_or_two_factor};
use tauri::{AppHandle, Manager, State};
use vrchatapi::models::CurrentUser;
use vrchatapi::models::EitherUserOrTwoFactor;

#[tauri::command]
pub async fn start_auth_flow(
    app: AppHandle,
    state: State<'_, AuthState>,
    username: String,
    password: String,
) -> Result<(), String> {
    let username = username.trim().to_string();
    let password = password;

    if username.is_empty() || password.is_empty() {
        AuthEvent::Failure {
            message: "Please enter your username and password.".to_string(),
            code: None,
        }
        .emit(&app);
        return Ok(());
    }

    AuthEvent::Started {
        action: AuthAction::Credentials,
    }
    .emit(&app);

    let config = {
        let mut session = lock_session(&state)?;
        session.reset_for_login();
        session.config.basic_auth = Some((username, Some(password)));
        session.config.clone()
    };

    let login_result = match fetch_user_or_two_factor(&config).await {
        Ok(login_result) => login_result,
        Err(error) => {
            clear_basic_auth(&state);
            AuthEvent::Failure {
                message: error,
                code: None,
            }
            .emit(&app);
            return Ok(());
        }
    };

    match login_result {
        EitherUserOrTwoFactor::CurrentUser(user) => {
            handle_login_success(&app, &state, &user)?;
        }
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
            let mut session = lock_session(&state)?;
            session.is_pending_two_factor = true;
            AuthEvent::TwoFactorRequired {
                methods: requires_auth.requires_two_factor_auth.clone(),
                message: Some("Please enter your 2FA code".to_string()),
            }
            .emit(&app);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn verify_two_factor(
    app: AppHandle,
    state: State<'_, AuthState>,
    two_factor_code: String,
    two_factor_method: String,
) -> Result<(), String> {
    let code = two_factor_code.trim();
    let method = two_factor_method.trim();

    if code.is_empty() {
        AuthEvent::Failure {
            message: "Please enter your 2FA code".to_string(),
            code: None,
        }
        .emit(&app);
        return Ok(());
    }

    if method.is_empty() {
        AuthEvent::Failure {
            message: "Please select a 2FA method".to_string(),
            code: None,
        }
        .emit(&app);
        return Ok(());
    }

    let (config, is_pending_two_factor) = {
        let session = lock_session(&state)?;
        (session.config.clone(), session.is_pending_two_factor)
    };

    if !is_pending_two_factor {
        AuthEvent::Failure {
            message: "2FA session not found. Please log in again".to_string(),
            code: None,
        }
        .emit(&app);
        return Ok(());
    }

    AuthEvent::Started {
        action: AuthAction::TwoFactor,
    }
    .emit(&app);

    if let Err(error) = dispatch_2fa_verification(&config, code, method).await {
        AuthEvent::Failure {
            message: error,
            code: None,
        }
        .emit(&app);
        return Ok(());
    }

    let login_result = match fetch_user_or_two_factor(&config).await {
        Ok(login_result) => login_result,
        Err(error) => {
            AuthEvent::Failure {
                message: error,
                code: None,
            }
            .emit(&app);
            return Ok(());
        }
    };

    match login_result {
        EitherUserOrTwoFactor::CurrentUser(user) => {
            handle_login_success(&app, &state, &user)?;
        }
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
            let mut session = lock_session(&state)?;
            session.is_pending_two_factor = true;
            AuthEvent::TwoFactorRequired {
                methods: requires_auth.requires_two_factor_auth.clone(),
                message: Some("Another 2FA method is required".to_string()),
            }
            .emit(&app);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn restore_session(app: AppHandle) -> Result<(), String> {
    restore_session_inner(&app).await
}

async fn restore_session_inner(app: &AppHandle) -> Result<(), String> {
    let (config, has_cookies) = {
        let state = app.state::<AuthState>();
        let session = lock_session(&state)?;
        (session.config.clone(), session.cookie_header().is_some())
    };

    if !has_cookies {
        return Ok(());
    }

    let login_result = match fetch_user_or_two_factor(&config).await {
        Ok(login_result) => login_result,
        Err(error) => {
            if is_auth_invalid(&error) {
                let state = app.state::<AuthState>();
                clear_session_cookies(&state);
                clear_persisted_cookies_best_effort();
            }
            return Ok(());
        }
    };

    match login_result {
        EitherUserOrTwoFactor::CurrentUser(user) => {
            let state = app.state::<AuthState>();
            handle_login_success(app, &state, &user)?;
        }
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
            let state = app.state::<AuthState>();
            clear_session_cookies(&state);
            clear_persisted_cookies_best_effort();
        }
    }

    Ok(())
}

fn to_auth_user(user: &CurrentUser) -> AuthUser {
    AuthUser {
        id: user.id.clone(),
        display_name: user.display_name.clone(),
        username: user.username.clone(),
    }
}

fn lock_session(state: &AuthState) -> Result<std::sync::MutexGuard<'_, AuthSession>, String> {
    state
        .session
        .lock()
        .map_err(|_| String::from("Failed to lock auth state (Mutex poisoned)"))
}

fn handle_login_success(
    app: &AppHandle,
    state: &AuthState,
    user: &CurrentUser,
) -> Result<(), String> {
    let mut session = lock_session(state)?;
    session.is_pending_two_factor = false;
    session.config.basic_auth = None;
    if let Err(err) = session.persist_cookies() {
        eprintln!("Failed to save auth cookies to keychain: {err}");
    }
    AuthEvent::Success {
        user: to_auth_user(user),
    }
    .emit(app);
    Ok(())
}

fn clear_basic_auth(state: &AuthState) {
    if let Ok(mut session) = lock_session(state) {
        session.config.basic_auth = None;
    }
}

fn clear_session_cookies(state: &AuthState) {
    if let Ok(mut session) = lock_session(state) {
        session.reset_for_login();
    }
}

fn clear_persisted_cookies_best_effort() {
    match clear_persisted_cookies() {
        Ok(()) | Err(keyring::Error::NoEntry) => {}
        Err(err) => {
            eprintln!("Failed to clear auth cookies from keychain: {err}");
        }
    }
}

fn is_auth_invalid(message: &str) -> bool {
    message.contains("HTTP 401") || message.contains("HTTP 403")
}
