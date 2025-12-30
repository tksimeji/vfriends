use std::sync::{Arc, Mutex};

use keyring::Entry;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::HeaderValue;
use reqwest::Url;
use vrchatapi::apis::configuration::Configuration;

const USER_AGENT: &str = "vfriends/authv2";
const COOKIE_FALLBACK_URL: &str = "https://api.vrchat.cloud";
const KEYCHAIN_SERVICE: &str = "vfriends";
const KEYCHAIN_ACCOUNT: &str = "vrchat_auth_cookies";

pub struct AuthSession {
    pub config: Configuration,
    pub cookie_jar: Arc<Jar>,
    pub cookie_base_url: Url,
    pub is_pending_two_factor: bool,
}

impl AuthSession {
    pub fn new() -> Self {
        Self::from_cookie_header(load_cookie_header())
    }

    pub fn new_without_cookies() -> Self {
        Self::from_cookie_header(None)
    }

    pub fn reset_for_login(&mut self) {
        *self = Self::new_without_cookies();
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn cookie_header(&self) -> Option<String> {
        self.cookie_jar
            .cookies(&self.cookie_base_url)
            .and_then(|value| {
                std::str::from_utf8(value.as_bytes())
                    .ok()
                    .map(|value| value.to_string())
            })
    }

    pub fn persist_cookies(&self) -> Result<(), keyring::Error> {
        if let Some(header) = self.cookie_header() {
            store_cookie_header(&header)?;
        }
        Ok(())
    }

    fn from_cookie_header(cookie_header: Option<String>) -> Self {
        let (mut config, cookie_jar, cookie_base_url) =
            build_config_with_cookie_header(cookie_header);
        config.user_agent = Some(String::from(USER_AGENT));
        Self {
            config,
            cookie_jar,
            cookie_base_url,
            is_pending_two_factor: false,
        }
    }
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
}

fn cookie_base_url_of(config: &Configuration) -> Url {
    match Url::parse(&config.base_path) {
        Ok(mut url) => {
            url.set_path("/");
            url.set_query(None);
            url.set_fragment(None);
            url
        }
        Err(err) => {
            eprintln!("Failed to parse base_path for cookie URL: {err}");
            Url::parse(COOKIE_FALLBACK_URL).expect("Invalid fallback cookie URL")
        }
    }
}

fn build_config_with_cookie_header(
    cookie_header: Option<String>,
) -> (Configuration, Arc<Jar>, Url) {
    let cookie_jar = Arc::new(Jar::default());

    let mut config = Configuration::default();
    config.client = reqwest::Client::builder()
        .cookie_provider(cookie_jar.clone())
        .build()
        .expect("Failed to build reqwest client");

    let cookie_base_url = cookie_base_url_of(&config);

    if let Some(header_value) = cookie_header {
        match HeaderValue::from_str(&header_value) {
            Ok(header_value) => {
                let mut headers = std::iter::once(&header_value);
                cookie_jar.set_cookies(&mut headers, &cookie_base_url);
            }
            Err(err) => {
                eprintln!("Failed to parse cookie header from keychain: {err}");
            }
        }
    }

    (config, cookie_jar, cookie_base_url)
}

fn load_cookie_header() -> Option<String> {
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

fn store_cookie_header(cookie_header: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT)?;
    entry.set_password(cookie_header)
}

pub fn clear_persisted_cookies() -> Result<(), keyring::Error> {
    let entry = Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT)?;
    entry.delete_credential()
}
