//! WASM-friendly wrapper functions that mirror the core conversion API.
//!
//! These functions return plain Rust types (`Result<String, CoreError>`, etc.)
//! rather than `JsValue`. The actual `#[wasm_bindgen]` exports that convert
//! to/from `JsValue` live in `timestamp_zone_web`.
//!
//! All functions in this module are designed to be called directly from
//! wasm-bindgen-annotated functions with minimal glue.

use crate::convert;
use crate::error::CoreError;
use crate::timezone;

pub use crate::convert::Duration;
pub use crate::timezone::TimezoneInfo;

/// Auto-detect whether a numeric value is seconds or milliseconds.
/// Values > 10000000000000 are treated as milliseconds.
pub fn auto_detect_unix(value: i64) -> (i64, bool) {
    if value > 10_000_000_000_000 {
        (value / 1000, true)
    } else {
        (value, false)
    }
}

pub fn wasm_now_unix() -> i64 {
    convert::now_unix()
}

pub fn wasm_now_unix_ms() -> i64 {
    convert::now_unix_ms()
}

pub fn wasm_unix_to_iso8601(unix_seconds: i64) -> Result<String, CoreError> {
    convert::unix_to_iso8601(unix_seconds)
}

pub fn wasm_unix_ms_to_iso8601(unix_ms: i64) -> Result<String, CoreError> {
    convert::unix_ms_to_iso8601(unix_ms)
}

pub fn wasm_iso8601_to_unix(iso_str: &str) -> Result<i64, CoreError> {
    convert::iso8601_to_unix(iso_str)
}

pub fn wasm_format_rfc2822(unix_seconds: i64) -> Result<String, CoreError> {
    convert::format_rfc2822(unix_seconds)
}

pub fn wasm_format_rfc3339(unix_seconds: i64) -> Result<String, CoreError> {
    convert::format_rfc3339(unix_seconds)
}

pub fn wasm_format_custom(unix_seconds: i64, format_str: &str) -> Result<String, CoreError> {
    convert::format_custom(unix_seconds, format_str)
}

pub fn wasm_unix_to_timezone(
    unix_seconds: i64,
    tz_offset_hours: i32,
    tz_offset_minutes: i32,
) -> Result<String, CoreError> {
    convert::unix_to_timezone(unix_seconds, tz_offset_hours, tz_offset_minutes)
}

pub fn wasm_duration_between(from_unix: i64, to_unix: i64) -> Duration {
    convert::duration_between(from_unix, to_unix)
}

pub fn wasm_common_timezones() -> Vec<TimezoneInfo> {
    timezone::common_timezones()
}

pub fn wasm_all_iana_timezones() -> Vec<String> {
    timezone::all_iana_timezones()
}
