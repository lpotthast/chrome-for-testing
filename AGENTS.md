# Repository Guidelines

## Project Structure & Module Organization

This Rust library provides typed access to the Chrome for Testing JSON API. Public exports are centralized in
`src/lib.rs`; implementation modules live in `src/api/`, with ChromeDriver utilities in `src/chromedriver.rs` and crate
errors in `src/error.rs`. Integration tests are in `tests/integration.rs`; unit tests are colocated with modules and use
`test-data/` fixtures. User docs are in `README.md`; release notes are in `CHANGELOG.md`.

## Build, Test, and Development Commands

- `cargo build`: build the crate.
- `cargo check --all`: type-check all targets quickly.
- `cargo fmt`: format Rust code.
- `cargo test --all`: run unit and integration tests.
- `cargo test <test_name>`: run a focused test by name.
- `just clippy`: run `cargo clippy --all -- -W clippy::pedantic`.
- `just tidy`: update deps, sort `Cargo.toml`, format, check, lint, test, and build docs.
- `just install-tools`: install tools used by `just` recipes.
- `just minimal-versions`: verify direct minimum dependency bounds.

## Architecture Notes

Endpoint structs expose `fetch(client)` for the live API and `fetch_with_base_url(client, url)` for tests. `HasVersion`
covers `VersionWithoutChannel` and `VersionInChannel`. `Version` is Chrome's four-part `major.minor.patch.build` format,
not semver. Watch the two `Downloads` types: known-good versions may omit `chromedriver`; last-known-good versions
include non-optional `chrome`, `chromedriver`, and `chrome_headless_shell` lists. `LastKnownGoodVersions` stores
channels in a `HashMap` with `stable()`, `beta()`, `dev()`, and `canary()` accessors.

## Coding Style & Naming Conventions

Use Rust 2024 with the MSRV in `Cargo.toml`. Follow `rustfmt` and keep clippy pedantic warnings clean. Prefer crate-root
imports such as `chrome_for_testing::KnownGoodVersions` in examples. Use `snake_case` for functions, modules, and tests;
`PascalCase` for types and traits. Keep serde mappings explicit for upstream API names. Timestamps use
`time::OffsetDateTime` with RFC 3339 serde support.

## Testing Guidelines

Use `tokio::test` for async tests and `assertr` for assertions. Prefer `mockito` plus `include_str!` fixtures from
`test-data/` for deserialization and endpoint behavior. Keep live API tests limited because they depend on network
availability and upstream data. Name tests by behavior, for example
`can_query_known_good_versions_api_endpoint_and_deserialize_response`.

## Commit & Pull Request Guidelines

Recent commits use short, imperative summaries such as `Fix clippy lints` or `Update CHANGELOG`; capitalize the first
word. Group related changes, and avoid mixing dependency updates with unrelated refactors. Pull requests should include
a description, linked issues, user-visible API changes, test results, and documentation or changelog updates when
behavior changes.

## Agent-Specific Instructions

Do not overwrite unrelated working-tree changes. Keep edits scoped, update fixtures only when response shapes change,
and prefer the existing `fetch`/`fetch_with_base_url` test pattern.
