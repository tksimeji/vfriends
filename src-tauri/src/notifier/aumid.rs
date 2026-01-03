use std::sync::OnceLock;
use tauri::AppHandle;
use windows_registry::CURRENT_USER;

static APP_USER_ID_READY: OnceLock<bool> = OnceLock::new();

pub fn ensure_app_user_model_id(app: &AppHandle) -> bool {
    if let Some(ready) = APP_USER_ID_READY.get() {
        return *ready;
    }

    let ready = if tauri::is_dev() {
        register_app_user_model_id(app)
    } else {
        true
    };

    APP_USER_ID_READY.set(ready).ok();
    ready
}

fn register_app_user_model_id(app: &AppHandle) -> bool {
    let app_id = app.config().identifier.clone();

    let key = match CURRENT_USER.create(format!(r"SOFTWARE\Classes\AppUserModelId\{app_id}")) {
        Ok(key) => key,
        Err(err) => {
            eprintln!("Failed to create AppUserModelID registry key: {err}");
            return false;
        }
    };

    let display_name = app
        .config()
        .product_name
        .clone()
        .unwrap();

    if let Err(err) = key.set_string("DisplayName", display_name) {
        eprintln!("Failed to set AppUserModelID DisplayName: {err}");
        return false;
    }

    if let Err(err) = key.set_string("IconBackgroundColor", "0") {
        eprintln!("Failed to set AppUserModelID IconBackgroundColor: {err}");
    }

    if let Ok(dir) = std::env::current_dir() {
        let candidates = [
            dir.join("icons/32x32.png"),
            dir.join("src-tauri/icons/32x32.png"),
        ];
        if let Some(path) = candidates.iter().find(|path| path.exists()) {
            if let Err(err) = key.set_hstring("IconUri", &path.as_path().into()) {
                eprintln!("Failed to set AppUserModelID IconUri: {err}");
            }
        }
    }

    true
}
