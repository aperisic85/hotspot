use log::info;
use chrono::Utc;

pub fn log_login_attempt(username: &str, password: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    info!("Login attempt: username={}, password={}, timestamp={}", username, password, timestamp);
}