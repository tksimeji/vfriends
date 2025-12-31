use super::state::AuthSession;
use keyring::Entry;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::HeaderValue;
use reqwest::{Client, Url};
use std::iter::once;
use std::sync::Arc;
use vrchatapi::apis::configuration::Configuration;

const KEYCHAIN_SERVICE: &str = "vfriends";
const KEYCHAIN_ACCOUNT: &str = "vrchat_auth_cookies";

impl AuthSession {
    pub fn new() -> Self {
        Self::from_cookie_header(load_saved_cookie_header())
    }

    pub fn new_without_cookies() -> Self {
        Self::from_cookie_header(None)
    }

    pub fn cookie_header(&self) -> Option<String> {
        let cookie_base_url = cookie_base_url(&self.config);
        self.cookie_jar.cookies(&cookie_base_url).and_then(|value| {
            std::str::from_utf8(value.as_bytes())
                .ok()
                .map(|value| value.to_string())
        })
    }

    pub fn save_cookies(&self) -> Result<(), keyring::Error> {
        if let Some(header) = self.cookie_header() {
            let entry = Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT)?;
            entry.set_password(&header)?;
        }
        Ok(())
    }

    pub fn clear_saved_cookies() -> Result<(), keyring::Error> {
        let entry = Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT)?;
        entry.delete_credential()
    }

    pub fn reset(&mut self) {
        *self = Self::new_without_cookies();
    }

    fn from_cookie_header(cookie_header: Option<String>) -> Self {
        let cookie_jar = Arc::new(Jar::default());

        let mut config = Configuration::default();
        config.user_agent = Some(String::from("vfriends"));
        config.client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("Failed to build reqwest client");

        if let Some(header_value) = cookie_header {
            match HeaderValue::from_str(&header_value) {
                Ok(header_value) => {
                    let mut headers = once(&header_value);
                    let cookie_base_url = cookie_base_url(&config);
                    cookie_jar.set_cookies(&mut headers, &cookie_base_url);
                }
                Err(e) => {
                    eprintln!("Failed to parse cookie header: {}", e);
                }
            }
        }

        Self {
            config,
            cookie_jar,
            is_pending_2fa: false,
        }
    }
}

fn cookie_base_url(config: &Configuration) -> Url {
    match Url::parse(&config.base_path) {
        Ok(mut url) => {
            url.set_path("/");
            url.set_query(None);
            url.set_fragment(None);
            url
        }
        Err(err) => {
            eprintln!("Failed to parse base path for cookie URL: {err}");
            Url::parse("https://api.vrchat.cloud").expect("Invalid fallback cookie URL")
        }
    }
}

fn load_saved_cookie_header() -> Option<String> {
    let entry = match Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT) {
        Ok(entry) => entry,
        Err(err) => {
            eprintln!("Failed to access keychain entry: {err}");
            return None;
        }
    };

    match entry.get_password() {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        Err(keyring::Error::NoEntry) => None,
        Err(err) => {
            eprintln!("Failed to read auth cookies from keychain: {err}");
            None
        }
    }
}
