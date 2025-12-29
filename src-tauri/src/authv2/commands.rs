use crate::authv2::events::{AuthAction, AuthEvent, AuthUser};
use crate::authv2::service::{dispatch_2fa_verification, fetch_user_or_two_factor, AuthError, AuthState};
use tauri::{AppHandle, State};
use vrchatapi::models::EitherUserOrTwoFactor;

fn emit_failure(app: &AppHandle, error: AuthError) {
    AuthEvent::Failure {
        message: error.message,
        code: error.code,
    }
    .emit(app);
}

fn to_auth_user(user: &vrchatapi::models::CurrentUser) -> AuthUser {
    AuthUser {
        id: user.id.clone(),
        display_name: user.display_name.clone(),
        username: user.username.clone(),
    }
}

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
        emit_failure(
            &app,
            AuthError::new("ユーザー名とパスワードを入力してください。", None),
        );
        return Ok(());
    }

    AuthEvent::Started {
        action: AuthAction::Credentials,
    }
    .emit(&app);

    let config = {
        let mut session = state
            .session
            .lock()
            .map_err(|_| "Failed to lock auth state (Mutex poisoned)")?;
        session.reset();
        session.config.basic_auth = Some((username, Some(password)));
        session.config.clone()
    };

    let login_result = match fetch_user_or_two_factor(&config).await {
        Ok(login_result) => login_result,
        Err(error) => {
            emit_failure(&app, error);
            return Ok(());
        }
    };

    match login_result {
        EitherUserOrTwoFactor::CurrentUser(user) => {
            let mut session = state
                .session
                .lock()
                .map_err(|_| "Failed to lock auth state (Mutex poisoned)")?;
            session.is_pending_two_factor = false;
            AuthEvent::Success {
                user: to_auth_user(&user),
            }
            .emit(&app);
        }
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
            let mut session = state
                .session
                .lock()
                .map_err(|_| "Failed to lock auth state (Mutex poisoned)")?;
            session.is_pending_two_factor = true;
            AuthEvent::TwoFactorRequired {
                methods: requires_auth.requires_two_factor_auth.clone(),
                message: Some("2FAコードを入力してください。".to_string()),
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
        emit_failure(&app, AuthError::new("2FAコードを入力してください。", None));
        return Ok(());
    }

    if method.is_empty() {
        emit_failure(&app, AuthError::new("2FA方式を選択してください。", None));
        return Ok(());
    }

    let (config, is_pending_two_factor) = {
        let session = state
            .session
            .lock()
            .map_err(|_| "Failed to lock auth state (Mutex poisoned)")?;
        (session.config.clone(), session.is_pending_two_factor)
    };

    if !is_pending_two_factor {
        emit_failure(
            &app,
            AuthError::new(
                "2FAセッションが見つかりません。もう一度ログインしてください。",
                None,
            ),
        );
        return Ok(());
    }

    AuthEvent::Started {
        action: AuthAction::TwoFactor,
    }
    .emit(&app);

    if let Err(e) = dispatch_2fa_verification(
        &config,
        code,
        method,
    )
    .await
    {
        emit_failure(&app, e);
        return Ok(());
    }

    let login_result = match fetch_user_or_two_factor(&config).await {
        Ok(login_result) => login_result,
        Err(error) => {
            emit_failure(&app, error);
            return Ok(());
        }
    };

    match login_result {
        EitherUserOrTwoFactor::CurrentUser(user) => {
            let mut session = state
                .session
                .lock()
                .map_err(|_| "Failed to lock auth state (Mutex poisoned)")?;
            session.is_pending_two_factor = false;
            AuthEvent::Success {
                user: to_auth_user(&user),
            }
            .emit(&app);
        }
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
            let mut session = state
                .session
                .lock()
                .map_err(|_| "Failed to lock auth state (Mutex poisoned)")?;
            session.is_pending_two_factor = true;
            AuthEvent::TwoFactorRequired {
                methods: requires_auth.requires_two_factor_auth.clone(),
                message: Some("別の2FA方式が必要です。".to_string()),
            }
            .emit(&app);
        }
    }

    Ok(())
}
