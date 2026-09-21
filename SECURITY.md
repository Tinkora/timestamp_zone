# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (current) | ✅ |

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.

Instead, email the project maintainer directly. You should receive a response
within 48 hours. We will work with you to understand the scope and coordinate
a fix and disclosure timeline.

### Scope

The following areas are within scope:

- WASM sandbox escapes
- Input parsing vulnerabilities (ISO 8601, format strings)
- Integer overflow in timestamp/duration calculations
- XSS vectors in the web UI

### Out of Scope

- Issues already documented as known limitations
- Theoretical attacks requiring physical access
- Issues in dependencies (please report upstream)

## Security Model

The timestamp_zone project follows these security principles:

1. **Browser-local by default**: All timestamp conversion, formatting, and timezone lookups happen in-browser via WASM. No data ever leaves the browser.

2. **No network requests**: The application makes zero HTTP requests. No analytics, no telemetry, no CDN calls for timezone data.

3. **Deterministic output**: All formatting functions produce deterministic output independent of browser locale or system timezone settings.

4. **Input validation**: All user-provided strings (ISO 8601 dates, format strings) are validated before processing. Invalid inputs return structured errors, never panics.

5. **No eval or dynamic code**: The UI uses textContent and Canvas API for rendering; no innerHTML from user input.
