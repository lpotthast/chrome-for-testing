# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.4.0] - 2026-04-13

### Added

- `KnownGoodDownloads::chrome_headless_shell`, modeling `chrome-headless-shell` downloads from the
  `known-good-versions-with-downloads` endpoint. This is optional because older known-good versions may omit it.
- `Channel::Other(String)` for allowing access to newly-added upstream channels without requiring library updates.
- `Channel::is_known()` and `Channel::as_str()`.
- Platform lookup helpers on known-good and last-known-good download groups:
  `chrome_for_platform()`, `chromedriver_for_platform()`, and `chrome_headless_shell_for_platform()`.
- A crate-level `Result<T>` alias for report-returning crate APIs.
- `Download::parsed_url()` for parsing upstream download URL strings into typed `url::Url` values.
- `Platform::chrome_executable_path()` for the relative path to the Chrome executable inside an unpacked archive.

### Changed

- **Breaking:** Fetch APIs and `Platform::detect` now return typed `rootcause::Report` values.
- **Breaking:** `Channel` is no longer `Copy` because it can now preserve unknown upstream channel names.
- **Breaking:** `Channel::from_str()` now accepts unknown non-empty channel names as `Channel::Other`.
- **Breaking:** `FromStr` impls for `Channel`, `Platform`, and `Version` now return typed `rootcause::Report` values
  while
  preserving the dedicated parse error contexts.
- **Breaking:** `KnownGoodDownloads` now has the additional public `chrome_headless_shell` field.
- **Breaking:** `LastKnownGoodVersions::channels` is private; use `channels()` to inspect the full channel map.
- **Breaking:** `Platform::chrome_binary_name()` now returns `Google Chrome for Testing` for both macOS platforms,
  matching the executable name instead of the `.app` bundle or Linux binary name.
- Endpoint fetch errors now attach endpoint-specific context and preserve non-success HTTP status errors.
- `fetch_with_base_url()` now joins endpoint paths through a shared fetch helper.
- `LastKnownGoodVersions::channel()` now accepts any `impl Borrow<Channel>`.
- `LastKnownGoodVersions::channels()` exposes a map so newly-added upstream channels are preserved, while known-channel
  convenience accessors continue to return `Option<&VersionInChannel>`.
- README and crate-level docs now describe the modeled endpoint scope more precisely.
- Bump MSRV to 1.89.0.
- Bump the direct `serde` dependency lower bound to 1.0.220.
- Bump the direct `time` dependency lower bound to a version that provides `time::serde::rfc3339`.
- Exclude repository/editor tooling files from published crate packages.
- Update README examples for the crate-level `Result` alias and version 0.4.

### Fixed

- Fetch APIs now reject unsuccessful HTTP status codes before deserializing response bodies.
- `LastKnownGoodVersions` deserialization now rejects responses where a channel map key disagrees with the nested
  `channel` field.

## [0.3.0] - 2026-03-23

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

[Unreleased]: https://github.com/lpotthast/chrome-for-testing/compare/v0.4.0...HEAD

[0.4.0]: https://github.com/lpotthast/chrome-for-testing/compare/v0.3.0...v0.4.0

[0.3.0]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.3...v0.3.0

[0.2.3]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.2...v0.2.3

[0.2.2]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.1...v0.2.2

[0.2.1]: https://github.com/lpotthast/chrome-for-testing/compare/v0.2.0...v0.2.1

[0.2.0]: https://github.com/lpotthast/chrome-for-testing/compare/v0.1.0...v0.2.0

[0.1.0]: https://github.com/lpotthast/chrome-for-testing/releases/tag/v0.1.0
