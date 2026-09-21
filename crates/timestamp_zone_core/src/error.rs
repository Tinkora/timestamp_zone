use thiserror::Error;

/// Stable error type for timestamp conversion operations.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("Invalid Unix timestamp: {0}")]
    InvalidUnixTimestamp(i64),

    #[error("Invalid ISO 8601 string: {0}")]
    InvalidIso8601(String),

    #[error("Invalid format string: {0}")]
    InvalidFormatString(String),

    #[error("Unsupported timezone: {0}")]
    UnsupportedTimezone(String),
}

impl CoreError {
    /// Returns a stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidUnixTimestamp(_) => "INVALID_UNIX_TIMESTAMP",
            Self::InvalidIso8601(_) => "INVALID_ISO8601",
            Self::InvalidFormatString(_) => "INVALID_FORMAT_STRING",
            Self::UnsupportedTimezone(_) => "UNSUPPORTED_TIMEZONE",
        }
    }
}
