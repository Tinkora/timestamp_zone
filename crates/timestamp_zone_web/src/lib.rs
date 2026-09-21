use timestamp_zone_core::error::CoreError;
use timestamp_zone_core::wasm;
use wasm_bindgen::prelude::*;

/// Convert a CoreError into a JsValue with stable `code` and `message` fields.
fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

/// Serialize any serde value to JsValue.
fn serde_wasm<T: serde::Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value).map_err(|e| JsValue::from_str(&format!("序列化失败: {e}")))
}

/// Get the current Unix timestamp in seconds.
#[wasm_bindgen]
pub fn now_unix() -> i64 {
    wasm::wasm_now_unix()
}

/// Get the current Unix timestamp in milliseconds.
#[wasm_bindgen]
pub fn now_unix_ms() -> i64 {
    wasm::wasm_now_unix_ms()
}

/// Auto-detect and convert a Unix timestamp value (seconds or ms) to ISO 8601.
/// Returns JSON: `{ "iso8601": "...", "unix_seconds": N, "unix_ms": N, "was_ms": bool }`
#[wasm_bindgen]
pub fn unix_to_formats(value: i64) -> Result<JsValue, JsValue> {
    let (unix_seconds, was_ms) = wasm::auto_detect_unix(value);
    let unix_ms = if was_ms { value } else { value * 1000 };

    let iso8601 = wasm::wasm_unix_to_iso8601(unix_seconds).map_err(core_err)?;
    let rfc2822 = wasm::wasm_format_rfc2822(unix_seconds).map_err(core_err)?;
    let rfc3339 = wasm::wasm_format_rfc3339(unix_seconds).map_err(core_err)?;

    let result = serde_json::json!({
        "iso8601": iso8601,
        "rfc2822": rfc2822,
        "rfc3339": rfc3339,
        "unix_seconds": unix_seconds,
        "unix_ms": unix_ms,
        "was_ms": was_ms,
    });
    serde_wasm(&result)
}

/// Convert a Unix timestamp (seconds) to ISO 8601 string.
#[wasm_bindgen]
pub fn unix_to_iso8601(unix_seconds: i64) -> Result<String, JsValue> {
    wasm::wasm_unix_to_iso8601(unix_seconds).map_err(core_err)
}

/// Convert a Unix timestamp (ms) to ISO 8601 string.
#[wasm_bindgen]
pub fn unix_ms_to_iso8601(unix_ms: i64) -> Result<String, JsValue> {
    wasm::wasm_unix_ms_to_iso8601(unix_ms).map_err(core_err)
}

/// Convert an ISO 8601 string to Unix timestamp values.
/// Returns JSON: `{ "unix_seconds": N, "unix_ms": N }`
#[wasm_bindgen]
pub fn iso8601_to_unix(iso_str: &str) -> Result<JsValue, JsValue> {
    let unix_seconds = wasm::wasm_iso8601_to_unix(iso_str).map_err(core_err)?;
    let result = serde_json::json!({
        "unix_seconds": unix_seconds,
        "unix_ms": unix_seconds * 1000_i64,
    });
    serde_wasm(&result)
}

/// Format a Unix timestamp as RFC 2822.
#[wasm_bindgen]
pub fn format_rfc2822(unix_seconds: i64) -> Result<String, JsValue> {
    wasm::wasm_format_rfc2822(unix_seconds).map_err(core_err)
}

/// Format a Unix timestamp as RFC 3339.
#[wasm_bindgen]
pub fn format_rfc3339(unix_seconds: i64) -> Result<String, JsValue> {
    wasm::wasm_format_rfc3339(unix_seconds).map_err(core_err)
}

/// Format a Unix timestamp with a custom strftime format string.
#[wasm_bindgen]
pub fn format_custom(unix_seconds: i64, format_str: &str) -> Result<String, JsValue> {
    wasm::wasm_format_custom(unix_seconds, format_str).map_err(core_err)
}

/// Convert a Unix timestamp to local time in the specified timezone offset.
#[wasm_bindgen]
pub fn unix_to_timezone(
    unix_seconds: i64,
    tz_offset_hours: i32,
    tz_offset_minutes: i32,
) -> Result<String, JsValue> {
    wasm::wasm_unix_to_timezone(unix_seconds, tz_offset_hours, tz_offset_minutes)
        .map_err(core_err)
}

/// Calculate the duration between two Unix timestamps.
/// Returns the Duration struct as JSON.
#[wasm_bindgen]
pub fn duration_between(from_unix: i64, to_unix: i64) -> Result<JsValue, JsValue> {
    let d = wasm::wasm_duration_between(from_unix, to_unix);
    serde_wasm(&d)
}

/// Get the list of common timezones as a JSON array.
#[wasm_bindgen]
pub fn common_timezones() -> Result<JsValue, JsValue> {
    let tzs = wasm::wasm_common_timezones();
    serde_wasm(&tzs)
}

/// Get all IANA timezone names as a JSON array of strings.
#[wasm_bindgen]
pub fn all_iana_timezones() -> Result<JsValue, JsValue> {
    let tzs = wasm::wasm_all_iana_timezones();
    serde_wasm(&tzs)
}
