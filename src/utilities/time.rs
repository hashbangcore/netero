use chrono::Local;

/// Returns local date and time in YYYY-MM-DD HH:MM:SS format.
pub fn current_datetime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
