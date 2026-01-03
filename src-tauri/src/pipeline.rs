use crate::websockets::listen;
use std::sync::Mutex;
use std::time::Duration;
use tauri::async_runtime::{spawn, JoinHandle};
use tauri::{AppHandle, Manager};
use tokio::time::sleep;

const DEFAULT_USER_AGENT: &str = "vfriends";

#[derive(Default)]
pub struct PipelineState {
    task: Mutex<Option<JoinHandle<()>>>,
}

pub fn start_with_cookie_header(
    app: &AppHandle,
    cookie_header: Option<String>,
    user_agent: Option<String>,
) {
    let Some(header) = cookie_header else {
        return;
    };
    if header.trim().is_empty() {
        eprintln!("Pipeline start skipped: cookie header is empty");
        return;
    }
    let Some(auth_token) = auth_token_from_cookie_header(&header) else {
        eprintln!("Pipeline start skipped: auth token not found in cookie header");
        return;
    };
    eprintln!("Pipeline start: auth token found, starting websocket listener.");
    start(app, auth_token, user_agent);
}

pub fn start(app: &AppHandle, auth_token: String, user_agent: Option<String>) {
    let state = app.state::<PipelineState>();
    state.start(app, auth_token, user_agent);
}

pub fn stop(app: &AppHandle) {
    let state = app.state::<PipelineState>();
    state.stop();
}

impl PipelineState {
    pub fn start(&self, app: &AppHandle, auth_token: String, user_agent: Option<String>) {
        self.stop();
        let app = app.clone();
        let handle = spawn(async move {
            run_pipeline(app, auth_token, user_agent).await;
        });
        let mut guard = self.task.lock().expect("Pipeline task lock poisoned");
        *guard = Some(handle);
    }

    pub fn stop(&self) {
        let mut guard = self.task.lock().expect("Pipeline task lock poisoned");
        if let Some(handle) = guard.take() {
            handle.abort();
        }
    }
}

async fn run_pipeline(app: AppHandle, auth_token: String, user_agent: Option<String>) {
    let user_agent = user_agent.unwrap_or_else(|| DEFAULT_USER_AGENT.to_string());
    let mut delay = Duration::from_secs(5);

    loop {
        let app_handle = app.clone();
        let result = listen(&app_handle, &auth_token, &user_agent).await;

        match result {
            Ok(()) => delay = Duration::from_secs(5),
            Err(err) => {
                eprintln!("Pipeline disconnected: {err}");
                sleep(delay).await;
                delay = std::cmp::min(delay * 2, Duration::from_secs(60));
            }
        }
    }
}

fn auth_token_from_cookie_header(cookie_header: &str) -> Option<String> {
    cookie_header.split(';').find_map(|entry| {
        let mut parts = entry.trim().splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        match name {
            "auth" | "authToken" | "auth_token" => Some(value.trim_matches('"').to_string()),
            _ => None,
        }
    })
}
