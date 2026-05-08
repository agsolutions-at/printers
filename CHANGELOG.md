# Changelog

All notable changes are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0]

### Added

- `pauseJob`, `resumeJob`, `cancelJob`, `restartJob` — control individual print
  jobs by ID. Each throws on failure.
- Optional Ghostscript pre-converter for `print` / `printFile` via a 5th
  `ghostscript?: GhostscriptConfig` parameter. Useful for PDF→PostScript
  conversion before sending to a printer (esp. on Windows). Requires `gs` /
  `gswin64c.exe` on `PATH`. New types: `GhostscriptDevice` enum (`PS2WRITE`,
  `PNG16M`, `TIFFG4`, `PNGMONO`), `GhostscriptConfig` interface.

### Changed

- Upgrade to napi-rs v3. Public TypeScript API is preserved; the regenerated
  loader and bindings ship the same function signatures, field names, and
  enum values. Existing consumers can upgrade with no code changes.
- Switch the underlying Rust crate from a git fork to the upstream
  [`printers`](https://crates.io/crates/printers) v2.3.0 release on crates.io.
