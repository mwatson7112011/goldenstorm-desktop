use chrono::{DateTime, Local};
use serde_json::Value;

/// Safely extract a string from JSON without panicking.
pub fn json_str(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or_default()
        .to_string()
}

/// Safely extract a number from JSON without panicking.
pub fn json_f64(v: &Value, key: &str) -> f64 {
    v.get(key)
        .and_then(|x| x.as_f64())
        .unwrap_or(0.0)
}

/// Convert Celsius to Fahrenheit.
pub fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

/// Convert Fahrenheit to Celsius.
pub fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Format a timestamp into a readable local time string.
pub fn format_timestamp(ts: i64) -> String {
    let dt = DateTime::<Local>::from(std::time::UNIX_EPOCH + std::time::Duration::from_secs(ts as u64));
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Trim a string and collapse whitespace.
pub fn clean_string(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Convert Option<String> to a clean string.
pub fn opt_str(v: Option<String>) -> String {
    v.unwrap_or_default().trim().to_string()
}

/// Convert Option<&str> to a clean string.
pub fn opt_str_ref(v: Option<&str>) -> String {
    v.unwrap_or("").trim().to_string()
}
