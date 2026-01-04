use crate::websocket;
use cookie::Cookie;
use log::{debug, info, warn};
use std::sync::Mutex;
use std::time::Duration;
use tauri::AppHandle;
use tokio::task::JoinHandle;

#[derive(Default)]
pub struct WebsocketState {
    task: Mutex<Option<JoinHandle<()>>>,
}

impl WebsocketState {
    pub fn start(&self, app: &AppHandle, auth_token: String, user_agent: Option<String>) {
        self.stop();

        let app = app.clone();
        let handle = tokio::spawn(async move {
            let user_agent = user_agent.unwrap_or_else(|| app.config().identifier.clone());

            let base_delay = Duration::from_secs(5);
            let max_delay = Duration::from_secs(60);

            let mut delay = base_delay;

            loop {
                let app = app.clone();

                if delay != base_delay {
                    info!("Websocket reconnecting...");
                }

                let result = websocket::listen(&app, &auth_token, &user_agent).await;

                match result {
                    Ok(()) => {
                        debug!("Websocket listen ended without error. Resettings backoff.");
                        delay = base_delay;
                    }
                    Err(err) => {
                        warn!(
                            "Websocket disconnected. (Will try again in {} seconds): {err}",
                            delay.as_secs()
                        );
                        tokio::time::sleep(delay).await;
                        delay = std::cmp::min(delay * 2, max_delay);
                    }
                }
            }
        });

        let mut guard = self.task.lock().expect("Websocket task lock poisoned.");
        *guard = Some(handle);
    }

    pub fn start_with_cookie_header(
        &self,
        app: &AppHandle,
        cookie_header: Option<String>,
        user_agent: Option<String>,
    ) {
        let Some(header) = cookie_header else {
            return;
        };
        if header.trim().is_empty() {
            warn!("Websocket start skipped. cookie header is empty.");
        }

        let auth_token = Cookie::split_parse(&header)
            .filter_map(|r| r.ok())
            .find_map(|c| match c.name() {
                "auth" | "authToken" | "auth_token" => Some(c.value().to_string()),
                _ => None,
            });

        let Some(auth_token) = auth_token else {
            warn!("Websocket start skipped. Auth token not found in cookie header.");
            return;
        };

        self.start(app, auth_token, user_agent);
    }

    pub fn stop(&self) {
        let mut guard = self.task.lock().expect("Websocket task lock poisoned.");
        if let Some(handle) = guard.take() {
            handle.abort();
        }
    }
}
