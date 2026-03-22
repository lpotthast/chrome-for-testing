# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust library (`chrome-for-testing`) providing a typed async client for the Chrome for Testing JSON API. Fetches version info about Chrome and ChromeDriver for testing automation. Uses `reqwest` for HTTP, `serde` for deserialization, and `thiserror` for errors.

## Build & Dev Commands

```bash
cargo build                          # Build
cargo test --all                     # Run all tests (unit + integration)
cargo test <test_name>               # Run a single test by name
cargo clippy --all -- -W clippy::pedantic  # Lint (pedantic)
just tidy                            # Full pipeline: update deps, sort, fmt, check, clippy, test
just install-tools                   # One-time: install nightly + cargo-hack, cargo-minimal-versions, cargo-msrv
just minimal-versions                # Verify minimum dependency version bounds
```

Integration tests (`tests/integration.rs`) hit the real Chrome for Testing API. Unit tests in API modules use `mockito` with fixtures from `test-data/`.

## Architecture

```
src/
├── lib.rs              # Crate root, re-exports api, chromedriver, error modules
├── error.rs            # Error enum (UrlParsing, Request, UnsupportedPlatform) via thiserror
├── chromedriver.rs     # ChromeDriver utilities (LogLevel enum)
└── api/
    ├── mod.rs          # Shared types: Download, HasVersion trait, API_BASE_URL (LazyLock)
    ├── channel.rs      # Channel enum (Stable, Beta, Dev, Canary)
    ├── platform.rs     # Platform enum with detect(), binary name methods, OS checks
    ├── version.rs      # Version struct with custom serde Visitor (major.minor.patch.build)
    ├── known_good_versions.rs      # KnownGoodVersions::fetch() endpoint
    └── last_known_good_versions.rs # LastKnownGoodVersions::fetch() endpoint
```

Key patterns:
- API endpoint structs expose `fetch(client)` and `fetch_with_base_url(client, url)` methods (the latter enables mockito testing).
- `HasVersion` trait provides a common interface for version types (`VersionWithoutChannel`, `VersionInChannel`).
- `Version` has a custom `Deserialize` impl (visitor-based) parsing `"major.minor.patch.build"` strings, with full `PartialOrd` support.
- Platform serde renames match the Chrome for Testing API field names (e.g., `linux64`, `mac-arm64`).

## Conventions

- MSRV: 1.80.0
- License: MIT OR Apache-2.0
- Clippy pedantic warnings are enforced
- Test assertions use the `assertr` crate
