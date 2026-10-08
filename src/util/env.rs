use std::env;

use chrono::{DateTime, Local};

/// Returns the value of an environment variable, ignoring blank values.
pub fn env_var(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
        _ => None,
    }
}

/// Returns local date and time in YYYY-MM-DD HH:MM:SS format.
pub fn current_datetime() -> String {
    format_datetime(&Local::now())
}

/// Formats a local instant in YYYY-MM-DD HH:MM:SS format.
fn format_datetime(at: &DateTime<Local>) -> String {
    at.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn formats_an_instant_with_a_fixed_pattern() {
        let at = Local
            .with_ymd_and_hms(2026, 6, 15, 9, 5, 3)
            .earliest()
            .expect("valid local time");
        assert_eq!(format_datetime(&at), "2026-06-15 09:05:03");
    }
}
