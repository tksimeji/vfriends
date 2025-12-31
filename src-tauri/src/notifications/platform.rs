use super::image::fetch_image;
use super::FriendOnlinePayload;
use std::path::PathBuf;
use tauri::AppHandle;

#[cfg(target_os = "windows")]
use std::sync::OnceLock;
#[cfg(target_os = "windows")]
use tauri::Manager;
#[cfg(target_os = "windows")]
use tauri_winrt_notification::{IconCrop, Toast};
#[cfg(target_os = "windows")]
use windows_registry::CURRENT_USER;

#[cfg(not(target_os = "windows"))]
use tauri_plugin_notification::NotificationExt;

pub async fn notify(app: &AppHandle, payload: &FriendOnlinePayload, user_agent: &str) {
    #[cfg(target_os = "windows")]
    {
        notify_windows(app, payload, user_agent).await;
        return;
    }

    #[cfg(not(target_os = "windows"))]
    {
        notify_desktop(app, payload, user_agent).await;
    }
}

#[cfg(not(target_os = "windows"))]
async fn notify_desktop(app: &AppHandle, payload: &FriendOnlinePayload, user_agent: &str) {
    let mut builder = app
        .notification()
        .builder()
        .title(&payload.title)
        .body(&payload.body);

    if let Some(path) = fetch_image(payload.image_url.clone(), user_agent).await {
        if let Some(path_str) = path.to_str() {
            builder = builder.icon(path_str.to_string());
        }
    }

    let _ = builder.show();
}

#[cfg(target_os = "windows")]
static APP_ID_READY: OnceLock<bool> = OnceLock::new();

#[cfg(target_os = "windows")]
async fn notify_windows(app: &AppHandle, payload: &FriendOnlinePayload, user_agent: &str) {
    let app_id = app.config().identifier.clone();
    let app_name = app
        .config()
        .product_name
        .clone()
        .unwrap_or_else(|| "vfriends".to_string());

    if ensure_windows_app_id(app, &app_id, &app_name) {
        let icon_path = fetch_image(payload.image_url.clone(), user_agent).await;
        let mut toast = Toast::new(&app_id)
            .title(&payload.display_name)
            .text1(&payload.title)
            .text2(&payload.body);

        if let Some(path) = icon_path {
            toast = toast.icon(path.as_path(), IconCrop::Circular, &payload.display_name);
        }

        if let Err(err) = toast.show() {
            eprintln!("Failed to show toast notification: {err}");
        }
    }
}

#[cfg(target_os = "windows")]
fn ensure_windows_app_id(app: &AppHandle, app_id: &str, app_name: &str) -> bool {
    if let Some(ready) = APP_ID_READY.get() {
        return *ready;
    }

    let ready = if tauri::is_dev() {
        register_windows_app_id(app_id, app_name, resolve_app_icon(app))
    } else {
        true
    };

    let _ = APP_ID_READY.set(ready);
    ready
}

#[cfg(target_os = "windows")]
fn register_windows_app_id(app_id: &str, app_name: &str, icon_path: Option<PathBuf>) -> bool {
    let key = match CURRENT_USER.create(format!(r"SOFTWARE\Classes\AppUserModelId\{app_id}")) {
        Ok(key) => key,
        Err(err) => {
            eprintln!("Failed to create AppUserModelID registry key: {err}");
            return false;
        }
    };

    if let Err(err) = key.set_string("DisplayName", app_name) {
        eprintln!("Failed to set AppUserModelID DisplayName: {err}");
        return false;
    }

    if let Some(icon_path) = icon_path {
        if let Some(icon) = icon_path.to_str() {
            if let Err(err) = key.set_string("IconUri", icon) {
                eprintln!("Failed to set AppUserModelID IconUri: {err}");
            }
        }
    }

    let _ = key.set_string("IconBackgroundColor", "0");
    true
}

#[cfg(target_os = "windows")]
fn resolve_app_icon(app: &AppHandle) -> Option<PathBuf> {
    let candidates = [
        "icons/icon.ico",
        "icons/128x128@2x.png",
        "icons/128x128.png",
        "icons/32x32.png",
    ];

    if let Ok(resource_dir) = app.path().resource_dir() {
        for candidate in candidates {
            let path = resource_dir.join(candidate);
            if path.exists() {
                return Some(path);
            }
        }
    }

    if let Ok(exe) = tauri::utils::platform::current_exe() {
        for ancestor in exe.ancestors().take(6) {
            for candidate in candidates {
                let path = ancestor.join(candidate);
                if path.exists() {
                    return Some(path);
                }
            }
        }
    }

    None
}
