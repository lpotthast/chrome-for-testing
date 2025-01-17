//! # Chrome for Testing API Client
//!
//! This crate provides programmatic access to "chrome-for-testing" JSON APIs,
//! which are used to retrieve version details and other relevant information about
//! Chrome and ChromeDriver for testing purposes.
//!
//! ## Modules Overview
//!
//! - [`api`]: Contains the core functionality to interact with the API endpoints.
//! - [`chromedriver`]: Facilitates interaction with ChromeDriver-specific data and operations.
//!
//! ## API Endpoints
//!
//! The crate leverages the following JSON API endpoints:
//! - **Known Good Versions**: Provides a list of known good versions of Chrome.
//! - **Last Known Good Versions**: Retrieves the last known good version of Chrome.
//!
//! For detailed documentation on these APIs, see the
//! [official Chrome for Testing documentation](https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints).
//!
//! ## Features
//!
//! - **Ease of Use**: Simplifies interaction with Chrome's testing-related APIs.
//! - **Type-Safe Deserialization**: Automatically maps JSON responses to Rust structs for
//!   seamless API interaction.
//! - **Asynchronous Support**: Fully asynchronous using the `tokio` runtime.
//!
//! ## Example Usage
//!
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     let client = reqwest::Client::new();
//!     match chrome_for_testing::api::known_good_versions::request(client).await {
//!         Ok(data) => println!("Successfully fetched Chrome versions: {:?}", data),
//!         Err(e) => println!("Error occurred: {}", e),
//!     }
//! }
//! ```

pub mod api;
pub mod chromedriver;
pub mod error;
