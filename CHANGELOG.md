# Changelog

## [0.1.0] - 2026-08-06

### Added
- `timestamp_zone_core`: Core conversion library with Unix ↔ ISO 8601, RFC 2822, RFC 3339 formatting
- `timestamp_zone_core`: Timezone data module with 16 common timezones and IANA listing
- `timestamp_zone_core`: Duration calculation between two timestamps
- `timestamp_zone_core`: Custom strftime-style format support
- `timestamp_zone_web`: WASM bridge with 10 JS exports
- `timestamp_zone_web`: Modern web UI with Chinese labels
- Agent Skill definition (`skills/`)
- CI workflow (native test, clippy, WASM check, wasm-pack build)
- Documentation: product spec, AGENTS.md

### Features
- Unix timestamp (seconds/ms) → ISO 8601, RFC 2822, RFC 3339, custom format
- ISO 8601 string → Unix timestamp (seconds + ms)
- Multi-timezone conversion with 16 common presets
- Duration calculator with human-readable output
- Auto-detect seconds vs milliseconds in Unix timestamp input
- One-click copy on all outputs
- All processing in WASM, zero server calls
