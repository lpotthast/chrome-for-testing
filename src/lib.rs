//! Provides programmatic access to the chrome-for-testing JSON APIs through
//! [`api::known_good_versions::request`] and [`api::last_known_good_versions::request`].
//!
//! Also contains type definitions used for deserialization of the API responses.
//!
//! Chrome documentation can be found here: <https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints>

/// Chrome for Testing API types and functions for fetching version information.
pub mod api;

/// ChromeDriver specific utilities, such as log level configuration.
pub mod chromedriver;

/// Error types used throughout the crate.
pub mod error;
