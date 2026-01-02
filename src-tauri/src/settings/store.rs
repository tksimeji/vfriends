use super::types::AppSettings;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

const SETTINGS_FILE: &str = "settings.json";

pub struct SettingsStore {
    path: PathBuf,
    state: Mutex<AppSettings>,
}

impl SettingsStore {
    pub fn load(app: &AppHandle) -> Self {
        let path = settings_path(app);
        let settings = read_settings(&path);
        Self {
            path,
            state: Mutex::new(settings),
        }
    }

    pub fn snapshot(&self) -> AppSettings {
        self.state
            .lock()
            .map(|settings| settings.clone())
            .unwrap_or_default()
    }

    pub fn consume(&self, consumer: impl FnOnce(&mut AppSettings)) {
        let snapshot = {
            let mut guard = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            consumer(&mut guard);
            guard.clone()
        };

        if let Err(err) = write_settings(&self.path, &snapshot) {
            eprintln!("Failed to save app settings: {err}");
        }
    }

    pub fn set(&self, settings: AppSettings) {
        {
            let mut guard = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            *guard = settings.clone();
        }

        if let Err(err) = write_settings(&self.path, &settings) {
            eprintln!("Failed to save app settings: {err}");
        }
    }
}

fn settings_path(app: &AppHandle) -> PathBuf {
    let base = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("vfriends"));
    base.join(SETTINGS_FILE)
}

fn read_settings(path: &Path) -> AppSettings {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return AppSettings::default();
    };

    serde_json::from_str::<AppSettings>(&contents).unwrap_or_default()
}

fn write_settings(path: &Path, settings: &AppSettings) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(settings).unwrap_or_else(|_| "{}".to_string());
    std::fs::write(path, json)
}
