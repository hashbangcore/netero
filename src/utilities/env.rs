use std::env;

/// Returns the preferred user language from common locale variables.
pub fn get_user_lang() -> String {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Some(value) = env_var(key) {
            return value;
        }
    }
    "unknown".to_string()
}

/// Returns the value of an environment variable, ignoring blank values.
pub fn env_var(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
        _ => None,
    }
}
