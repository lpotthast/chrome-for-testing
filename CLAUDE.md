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
cargo doc --no-deps                  # Build docs
just tidy                            # Full pipeline: update deps, sort, fmt, check, clippy, test, doc
just install-tools                   # One-time: install nightly + cargo-hack, cargo-minimal-versions, cargo-msrv
just minimal-versions                # Verify minimum dependency version bounds
```

Integration tests (`tests/integration.rs`) hit the real Chrome for Testing API. Unit tests in API modules use `mockito` with fixtures from `test-data/` (loaded via `include_str!`).

## Architecture

All public types are re-exported from `lib.rs` — users import from the crate root (e.g., `chrome_for_testing::KnownGoodVersions`).

Key patterns:
- API endpoint structs (`KnownGoodVersions`, `LastKnownGoodVersions`) expose `fetch(client)` and `fetch_with_base_url(client, url)` methods. The latter enables mockito testing against a local server.
- `HasVersion` trait provides a common interface for the two version types (`VersionWithoutChannel` from known_good_versions, `VersionInChannel` from last_known_good_versions).
- `Version` uses a 4-part format (`major.minor.patch.build`) matching Chrome's versioning scheme — not semver. It has a custom serde `Visitor` for parsing, `FromStr` for `"131.0.6778.204".parse()`, and full `Ord` support.
- `Platform` serde renames match the Chrome for Testing API field names (e.g., `linux64`, `mac-arm64`).
- There are **two distinct `Downloads` structs** — a common gotcha:
  - `known_good_versions::Downloads`: has `chrome: Vec<Download>` and `chromedriver: Option<Vec<Download>>` (older versions lack ChromeDriver)
  - `last_known_good_versions::Downloads`: has `chrome`, `chromedriver`, and `chrome_headless_shell` (all non-optional `Vec<Download>`)
- `LastKnownGoodVersions` stores channels in a `HashMap<Channel, VersionInChannel>` with convenience accessors (`stable()`, `beta()`, `dev()`, `canary()`).
- Timestamps use `time::OffsetDateTime` with `#[serde(with = "time::serde::rfc3339")]`.

## Conventions

- Edition: 2024
- MSRV: 1.85.1
- License: MIT OR Apache-2.0
- Clippy pedantic warnings are enforced
- Test assertions use the `assertr` crate