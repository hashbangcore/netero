use std::env;

/// Returns the value of an environment variable, ignoring blank values.
pub fn env_var(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
        _ => None,
    }
}

/// Returns local date and time in YYYY-MM-DD HH:MM:SS format.
pub fn current_datetime() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
