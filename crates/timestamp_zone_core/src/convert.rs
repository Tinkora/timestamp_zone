use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::Serialize;

use crate::error::CoreError;

/// Returns the current Unix timestamp in seconds.
///
/// On WASM targets, uses `js_sys::Date::now()` because `std::time::SystemTime`
/// panics in the WASM runtime.
pub fn now_unix() -> i64 {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Date::now() / 1000.0) as i64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}

/// Returns the current Unix timestamp in milliseconds.
pub fn now_unix_ms() -> i64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as i64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64
    }
}

/// Converts a Unix timestamp in seconds to an ISO 8601 string (UTC).
///
/// Example output: `"2026-08-06T16:30:00Z"`
pub fn unix_to_iso8601(unix_seconds: i64) -> Result<String, CoreError> {
    let dt = utc_from_unix(unix_seconds)?;
    Ok(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

/// Converts a Unix timestamp in milliseconds to an ISO 8601 string (UTC).
pub fn unix_ms_to_iso8601(unix_ms: i64) -> Result<String, CoreError> {
    unix_to_iso8601(unix_ms / 1000)
}

/// Converts an ISO 8601 string to a Unix timestamp in seconds.
///
/// Supports formats:
/// - `"2026-08-06T16:30:00Z"` (RFC 3339 with Z)
/// - `"2026-08-06T16:30:00+08:00"` (RFC 3339 with offset)
/// - `"2026-08-06T16:30:00"` (naive, treated as UTC)
/// - `"2026-08-06"` (date only, midnight UTC)
pub fn iso8601_to_unix(iso_str: &str) -> Result<i64, CoreError> {
    let trimmed = iso_str.trim();

    // Try RFC 3339 (with timezone offset or Z)
    if let Ok(dt) = DateTime::parse_from_rfc3339(trimmed) {
        return Ok(dt.timestamp());
    }

    // Try naive datetime with 'T' separator
    if trimmed.contains('T') {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S") {
            return Ok(ndt.and_utc().timestamp());
        }
        if let Ok(ndt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S%.f") {
            return Ok(ndt.and_utc().timestamp());
        }
    }

    // Try date only
    if let Ok(nd) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        return Ok(nd
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| CoreError::InvalidIso8601(iso_str.to_string()))?
            .and_utc()
            .timestamp());
    }

    // Try with space separator instead of T
    if trimmed.contains(' ') {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S") {
            return Ok(ndt.and_utc().timestamp());
        }
    }

    Err(CoreError::InvalidIso8601(iso_str.to_string()))
}

/// Formats a Unix timestamp as an RFC 2822 string.
///
/// Example output: `"Wed, 06 Aug 2026 16:30:00 +0000"`
pub fn format_rfc2822(unix_seconds: i64) -> Result<String, CoreError> {
    let dt = utc_from_unix(unix_seconds)?;
    Ok(dt.format("%a, %d %b %Y %H:%M:%S +0000").to_string())
}

/// Formats a Unix timestamp as an RFC 3339 string.
///
/// Example output: `"2026-08-06T16:30:00+00:00"`
pub fn format_rfc3339(unix_seconds: i64) -> Result<String, CoreError> {
    let dt = utc_from_unix(unix_seconds)?;
    Ok(dt.to_rfc3339())
}

/// Formats a Unix timestamp using a custom strftime-style format string.
///
/// Common specifiers: `%Y` (year), `%m` (month), `%d` (day),
/// `%H` (hour 00-23), `%M` (minute), `%S` (second),
/// `%A` (weekday name), `%B` (month name).
pub fn format_custom(unix_seconds: i64, format_str: &str) -> Result<String, CoreError> {
    if format_str.is_empty() {
        return Err(CoreError::InvalidFormatString(
            "format string must not be empty".into(),
        ));
    }
    let dt = utc_from_unix(unix_seconds)?;
    Ok(dt.format(format_str).to_string())
}

/// Converts a Unix timestamp to a local time string in the specified timezone offset.
///
/// `tz_offset_hours` and `tz_offset_minutes` define the UTC offset.
/// Returns an ISO 8601-style string without the timezone suffix.
///
/// Example: `unix_to_timezone(0, 8, 0)` → `"1970-01-01T08:00:00"`
pub fn unix_to_timezone(
    unix_seconds: i64,
    tz_offset_hours: i32,
    tz_offset_minutes: i32,
) -> Result<String, CoreError> {
    let offset_seconds = tz_offset_hours * 3600 + tz_offset_minutes * 60;
    let offset = FixedOffset::east_opt(offset_seconds).ok_or_else(|| {
        CoreError::UnsupportedTimezone(format!(
            "offset {}h{}m is out of valid range",
            tz_offset_hours, tz_offset_minutes
        ))
    })?;
    let dt = utc_from_unix(unix_seconds)?;
    let local = dt.with_timezone(&offset);
    Ok(local.format("%Y-%m-%dT%H:%M:%S").to_string())
}

/// A duration between two timestamps, with a human-readable representation.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Duration {
    /// Total seconds between the two timestamps (can be negative).
    pub total_seconds: i64,
    /// Number of whole days.
    pub days: i64,
    /// Remaining hours (0-23).
    pub hours: i32,
    /// Remaining minutes (0-59).
    pub minutes: i32,
    /// Remaining seconds (0-59).
    pub seconds: i32,
    /// Human-readable string like `"3d 5h 30m 15s"`.
    pub human_readable: String,
}

/// Calculates the duration between two Unix timestamps.
pub fn duration_between(from_unix: i64, to_unix: i64) -> Duration {
    let total_seconds = to_unix - from_unix;
    let abs_seconds = total_seconds.unsigned_abs();
    let days = (abs_seconds / 86400) as i64;
    let remaining = abs_seconds % 86400;
    let hours = (remaining / 3600) as i32;
    let remaining = remaining % 3600;
    let minutes = (remaining / 60) as i32;
    let seconds = (remaining % 60) as i32;

    let sign = if total_seconds < 0 { "-" } else { "" };
    let human_readable = if days > 0 {
        format!("{}{}d {}h {}m {}s", sign, days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}{}h {}m {}s", sign, hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}{}m {}s", sign, minutes, seconds)
    } else {
        format!("{}{}s", sign, seconds)
    };

    Duration {
        total_seconds,
        days: if total_seconds < 0 { -days } else { days },
        hours,
        minutes,
        seconds,
        human_readable,
    }
}

/// Helper: convert a Unix timestamp to a UTC DateTime.
fn utc_from_unix(unix_seconds: i64) -> Result<DateTime<Utc>, CoreError> {
    DateTime::from_timestamp(unix_seconds, 0)
        .ok_or(CoreError::InvalidUnixTimestamp(unix_seconds))
}

#[cfg(test)]
mod tests {
    use super::*;

    // 2026-08-06T16:30:00Z
    const TEST_TIMESTAMP: i64 = 1786033800;

    #[test]
    fn test_unix_to_iso8601() {
        let result = unix_to_iso8601(TEST_TIMESTAMP).unwrap();
        assert_eq!(result, "2026-08-06T16:30:00Z");
    }

    #[test]
    fn test_unix_ms_to_iso8601() {
        let result = unix_ms_to_iso8601(TEST_TIMESTAMP * 1000).unwrap();
        assert_eq!(result, "2026-08-06T16:30:00Z");
    }

    #[test]
    fn test_iso8601_to_unix_rfc3339_z() {
        let result = iso8601_to_unix("2026-08-06T16:30:00Z").unwrap();
        assert_eq!(result, TEST_TIMESTAMP);
    }

    #[test]
    fn test_iso8601_to_unix_rfc3339_offset() {
        // 2026-08-07T00:30:00+08:00 = same instant as 2026-08-06T16:30:00Z
        let result = iso8601_to_unix("2026-08-07T00:30:00+08:00").unwrap();
        assert_eq!(result, TEST_TIMESTAMP);
    }

    #[test]
    fn test_iso8601_to_unix_naive() {
        let result = iso8601_to_unix("2026-08-06T16:30:00").unwrap();
        assert_eq!(result, TEST_TIMESTAMP);
    }

    #[test]
    fn test_iso8601_to_unix_date_only() {
        let result = iso8601_to_unix("2026-08-06").unwrap();
        // 2026-08-06T00:00:00Z
        assert_eq!(result, 1785974400);
    }

    #[test]
    fn test_iso8601_to_unix_invalid() {
        let result = iso8601_to_unix("not-a-date");
        assert!(result.is_err());
        match result {
            Err(CoreError::InvalidIso8601(_)) => {}
            _ => panic!("expected InvalidIso8601"),
        }
    }

    #[test]
    fn test_format_rfc2822() {
        let result = format_rfc2822(TEST_TIMESTAMP).unwrap();
        assert_eq!(result, "Thu, 06 Aug 2026 16:30:00 +0000");
    }

    #[test]
    fn test_format_rfc3339() {
        let result = format_rfc3339(TEST_TIMESTAMP).unwrap();
        assert_eq!(result, "2026-08-06T16:30:00+00:00");
    }

    #[test]
    fn test_format_custom() {
        let result = format_custom(TEST_TIMESTAMP, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(result, "2026-08-06 16:30:00");
    }

    #[test]
    fn test_format_custom_empty() {
        let result = format_custom(TEST_TIMESTAMP, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_unix_to_timezone_utc() {
        let result = unix_to_timezone(TEST_TIMESTAMP, 0, 0).unwrap();
        assert_eq!(result, "2026-08-06T16:30:00");
    }

    #[test]
    fn test_unix_to_timezone_shanghai() {
        let result = unix_to_timezone(TEST_TIMESTAMP, 8, 0).unwrap();
        assert_eq!(result, "2026-08-07T00:30:00");
    }

    #[test]
    fn test_unix_to_timezone_kolkata() {
        // IST is UTC+5:30
        let result = unix_to_timezone(TEST_TIMESTAMP, 5, 30).unwrap();
        assert_eq!(result, "2026-08-06T22:00:00");
    }

    #[test]
    fn test_unix_to_timezone_negative() {
        // US Eastern is UTC-5
        let result = unix_to_timezone(TEST_TIMESTAMP, -5, 0).unwrap();
        assert_eq!(result, "2026-08-06T11:30:00");
    }

    #[test]
    fn test_duration_between_positive() {
        let d = duration_between(1000, 2000);
        assert_eq!(d.total_seconds, 1000);
        assert_eq!(d.days, 0);
        assert_eq!(d.hours, 0);
        assert_eq!(d.minutes, 16);
        assert_eq!(d.seconds, 40);
        assert_eq!(d.human_readable, "16m 40s");
    }

    #[test]
    fn test_duration_between_negative() {
        let d = duration_between(2000, 1000);
        assert_eq!(d.total_seconds, -1000);
        assert_eq!(d.days, 0);
        assert_eq!(d.hours, 0);
        assert_eq!(d.minutes, 16);
        assert_eq!(d.seconds, 40);
        assert_eq!(d.human_readable, "-16m 40s");
    }

    #[test]
    fn test_duration_between_days() {
        // 3 days + 5 hours + 30 minutes + 15 seconds
        let seconds = 3 * 86400 + 5 * 3600 + 30 * 60 + 15;
        let d = duration_between(0, seconds);
        assert_eq!(d.days, 3);
        assert_eq!(d.hours, 5);
        assert_eq!(d.minutes, 30);
        assert_eq!(d.seconds, 15);
        assert_eq!(d.human_readable, "3d 5h 30m 15s");
    }

    #[test]
    fn test_duration_between_zero() {
        let d = duration_between(100, 100);
        assert_eq!(d.total_seconds, 0);
        assert_eq!(d.human_readable, "0s");
    }

    #[test]
    fn test_invalid_unix_timestamp() {
        // chrono's valid range for DateTime<Utc> is roughly +- 262000 years
        // Testing with a clearly out-of-range value
        let result = unix_to_iso8601(i64::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn test_epoch() {
        let result = unix_to_iso8601(0).unwrap();
        assert_eq!(result, "1970-01-01T00:00:00Z");
    }
}
