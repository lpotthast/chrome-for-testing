//! # Chrome for Testing API Client
//!
//! This crate provides programmatic access to "chrome-for-testing" JSON APIs,
//! which are used to retrieve version details and download URLs for `Chrome`, `ChromeDriver`, and
//! Chrome Headless Shell for testing purposes.
//!
//! ## Modules Overview
//!
//! - [`chromedriver`]: `ChromeDriver` specific utilities, such as log level configuration.
//!
//! ## API Endpoints
//!
//! The crate leverages the following JSON API endpoints:
//!
//! - **Last Known Good Versions**:
//!   Recent good versions for each release channel (Stable/Beta/Dev/Canary), including `chrome`,
//!   `chromedriver`, and `chrome-headless-shell` downloads. Perfect if you just need the "latest
//!   stable" version for example.
//!
//! - **Known Good Versions**:
//!   All known good versions. Longer API response, not pre-grouped per release channel. Good fit
//!   if you have a hardcoded old version that you want to resolve a download URL for. Older
//!   entries may omit `chromedriver` and `chrome-headless-shell` downloads.
//!
//! For detailed documentation on these APIs, see the
//! [official Chrome for Testing documentation](https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints).
//!
//! ## Features
//!
//! - **Ease of Use**: Simplifies interaction with Chrome's testing-related APIs.
//! - **Type-Safe Deserialization**: Automatically maps JSON responses to Rust structs for
//!   seamless API interaction.
//! - **Asynchronous Support**: Fully asynchronous.
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() {
//!     use chrome_for_testing::KnownGoodVersions;
//!
//!     let client = reqwest::Client::new();
//!     match KnownGoodVersions::fetch(&client).await {
//!         Ok(data) => println!("Successfully fetched Chrome versions: {data:?}"),
//!         Err(e) => println!("Error occurred: {e:?}"),
//!     }
//! }
//! ```

/// `ChromeDriver` specific utilities, such as log level configuration.
pub mod chromedriver;

pub(crate) mod api;
pub(crate) mod error;

pub use api::Download;
pub use api::DownloadsByPlatform;
pub use api::HasVersion;
pub use api::channel::Channel;
pub use api::channel::ParseChannelError;
pub use api::known_good_versions::Downloads as KnownGoodDownloads;
pub use api::known_good_versions::KnownGoodVersions;
pub use api::known_good_versions::VersionWithoutChannel;
pub use api::last_known_good_versions::Downloads as LastKnownGoodDownloads;
pub use api::last_known_good_versions::LastKnownGoodVersions;
pub use api::last_known_good_versions::VersionInChannel;
pub use api::platform::ParsePlatformError;
pub use api::platform::Platform;
pub use api::version::ParseVersionError;
pub use api::version::Version;
pub use error::Error;

/// Result type returned by fallible crate APIs.
pub type Result<T, E = Error> = std::result::Result<T, rootcause::Report<E>>;
