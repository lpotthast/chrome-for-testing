# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- `Serialize` impl for all API types (`Download`, `Version`, `Channel`, `Platform`, `LogLevel`, `KnownGoodVersions`,
  `LastKnownGoodVersions`, and their inner types) , allowing API responses to be forwarded or cached using their
  serialized form.
- `FromStr` impl for `Channel`, `Platform`, and `Version` with dedicated parse error types (`ParseChannelError`,
  `ParsePlatformError`, `ParseVersionError`).
- `Ord` impl for `Version` (previously only `PartialOrd`).
- `DownloadsByPlatform` trait with `for_platform()` lookup method on download slices.
- Convenience accessors on `LastKnownGoodVersions`: `channel()`, `stable()`, `beta()`, `dev()`, `canary()`.
- `Hash` derive on `Download`, `Platform`, `Version` and `LogLevel`.
- `Clone` and `Copy` derives on `LogLevel`.
- Re-exports of all public types from the crate root (no longer need to reach into `api::` submodules).
- Justfile for common development tasks.
- CLAUDE.md, LLM instructions for Claude Code.
- CHANGELOG.md

### Changed

- **Breaking:** `fetch()` and `fetch_with_base_url()` now take `&reqwest::Client` and `&reqwest::Url` instead of owned
  values.
- **Breaking:** `api` and `error` modules are now `pub(crate)`; all public types are re-exported from the crate root.
- Bump MSRV to 1.85.1.
- Minimum versions for dependencies lowered as much as possible.
- Cargo.toml keywords for better crate discoverability.

### Fixed

- Mark crate-level doc example as `no_run` to prevent doc tests from hitting the network.

### Removed

- `serde_json` from direct dependencies.

## [0.2.3] - 2026-03-22

### Fixed

- Fix clippy lints.

### Changed

- Upgrade dev-dependencies.

## [0.2.2] - 2026-03-22

### Changed

- Switch to Rust edition 2024.

## [0.2.1] - 2026-03-22

### Changed

- Update reqwest dependency.

## [0.2.0] - 2025-09-28

### Changed

- Replace `anyhow` with custom error type using `thiserror`.
- Custom `Version` deserialization visitor (zero-allocation parsing of `"major.minor.patch.build"` strings).
- Move freestanding request functions to `fetch()` methods on API structs.
- Only use required `tokio` features.

### Added

- Integration tests hitting the real Chrome for Testing API.
- Unit tests using `mockito` with fixture data.
- Comprehensive rustdoc documentation.
- `Display` impl for `Channel`.

## [0.1.0] - 2025-01-17

### Added

- Initial release.
- Typed async client for Chrome for Testing JSON API.
- `KnownGoodVersions` and `LastKnownGoodVersions` API endpoints.
- `Platform` enum with OS detection.
- `Channel` enum (Stable, Beta, Dev, Canary).
- `Version` struct with custom deserialization.
- `HasVersion` trait providing a common interface for version types.
- ChromeDriver log level configuration.

[Unreleased]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.3...HEAD

[0.2.3]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.2...v0.2.3

[0.2.2]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.1...v0.2.2

[0.2.1]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.0...v0.2.1

[0.2.0]: https://github.com/lpotthast/chrome-for-testing/compare/v0.1.0...v0.2.0

[0.1.0]: https://github.com/lpotthast/chrome-for-testing/releases/tag/v0.1.0
