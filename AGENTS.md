# Repository Guide for AI Agents

## Project Overview

timestamp_zone is a browser-native Unix timestamp ↔ ISO 8601 ↔ multi-timezone converter. All computation happens in WASM. Privacy-first — no data leaves the browser. Useful for log analysis, cross-timezone collaboration, and API debugging.

## Architecture

```
timestamp_zone/
├── crates/
│   ├── timestamp_zone_core/       # Conversion logic, timezone data, error types
│   └── timestamp_zone_web/        # WASM bridge + HTML UI
├── docs/                           # Product specification
├── skills/                         # Agent Skill definitions (MCP tools)
└── index.html                      # Landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/timestamp_zone_core/src/convert.rs` | Core conversion functions (Unix ↔ ISO 8601, formatting, duration) |
| `crates/timestamp_zone_core/src/timezone.rs` | Timezone list and offset data |
| `crates/timestamp_zone_core/src/error.rs` | CoreError enum with stable error codes |
| `crates/timestamp_zone_core/src/wasm.rs` | WASM-binding-friendly wrappers around core functions |
| `crates/timestamp_zone_web/src/lib.rs` | #[wasm_bindgen] exports for JS interop |
| `crates/timestamp_zone_web/static/index.html` | Full-featured converter UI |
| `skills/timestamp_zone.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |

## Build & Test Commands

```bash
# Run all tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p timestamp_zone_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/timestamp_zone_web
```

## Design Principles

1. **Browser-first**: All conversion and formatting happens in-browser via WASM
2. **Privacy-first**: No timestamps or timezone queries leave the browser; no server round-trips
3. **Zero-dependency timezone data**: Static timezone list compiled into WASM; no IANA database downloads
4. **Deterministic**: Same input always produces the same output; no locale-dependent formatting
5. **Auto-detect**: Unix timestamps are auto-detected as seconds or milliseconds based on magnitude

## Core API Design

### Conversion Functions (convert.rs)

- `now_unix() -> i64` — current Unix timestamp in seconds (uses `js_sys::Date::now()` in WASM)
- `now_unix_ms() -> i64` — current Unix timestamp in milliseconds
- `unix_to_iso8601(unix_seconds: i64) -> Result<String, CoreError>`
- `unix_ms_to_iso8601(unix_ms: i64) -> Result<String, CoreError>`
- `iso8601_to_unix(iso_str: &str) -> Result<i64, CoreError>`
- `format_rfc2822(unix_seconds: i64) -> Result<String, CoreError>`
- `format_rfc3339(unix_seconds: i64) -> Result<String, CoreError>`
- `format_custom(unix_seconds: i64, format_str: &str) -> Result<String, CoreError>`
- `unix_to_timezone(unix_seconds: i64, tz_offset_hours: i32, tz_offset_minutes: i32) -> Result<String, CoreError>`
- `duration_between(from_unix: i64, to_unix: i64) -> Duration`

### Duration struct

```rust
pub struct Duration {
    pub total_seconds: i64,
    pub days: i64,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub human_readable: String,
}
```

### TimezoneInfo struct

```rust
pub struct TimezoneInfo {
    pub name: String,
    pub offset_hours: i32,
    pub offset_minutes: i32,
    pub abbr: String,
}
```

## Error Codes (Stable Machine-Readable)

| Code | Meaning |
|------|---------|
| `INVALID_UNIX_TIMESTAMP` | Unix timestamp out of valid range |
| `INVALID_ISO8601` | ISO 8601 string could not be parsed |
| `INVALID_FORMAT_STRING` | Custom format string is invalid |
| `UNSUPPORTED_TIMEZONE` | Timezone offset not supported |

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
