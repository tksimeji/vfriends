use crate::vrchat_utils::AppResult;
use std::sync::OnceLock;
use tauri::AppHandle;
use windows_registry::CURRENT_USER;

static APP_USER_ID_READY: OnceLock<bool> = OnceLock::new();

pub fn ensure_app_user_model_id(app: &AppHandle) -> AppResult<()> {
    if let Some(ready) = APP_USER_ID_READY.get() {
        return if *ready {
            Ok(())
        } else {
            Err(String::from("AppUserModelID is not ready."))
        };
    }

    if tauri::is_dev() {
        register_app_user_model_id(app).map_err(|err| {
            let _ = APP_USER_ID_READY.set(false);
            err
        })?;
    }

    let _ = APP_USER_ID_READY.set(true);
    Ok(())
}

fn register_app_user_model_id(app: &AppHandle) -> AppResult<()> {
    let app_id = app.config().identifier.clone();

    let key = CURRENT_USER
        .create(format!(r"SOFTWARE\Classes\AppUserModelId\{app_id}"))
        .map_err(|err| err.to_string())?;

    let display_name = app
        .config()
        .product_name
        .clone()
        .ok_or_else(|| String::from("Product name is not set in app config."))?;

    key.set_string("DisplayName", display_name)
        .map_err(|err| format!("Failed to set Application User Model ID DisplayName: {err}"))?;

    key.set_string("IconBackgroundColor", "0")
        .map_err(|err| format!("Failed to set AppUserModelID IconBackgroundColor: {err}"))?;

    if let Ok(dir) = std::env::current_dir() {
        let candidates = [
            dir.join("icons/32x32.png"),
            dir.join("src-tauri/icons/32x32.png"),
        ];
        if let Some(path) = candidates.iter().find(|path| path.exists()) {
            if let Err(err) = key.set_hstring("IconUri", &path.as_path().into()) {
                log::warn!("Failed to set AppUserModelID IconUri: {err}");
            }
        }
    }

    Ok(())
}
