//! Provides programmatic access to the chrome-for-testing JSON APIs through
//! [known_good_versions::request] and [last_known_good_versions::request].
//!
//! Also contains type definitions used for deserialization of the API responses.
//!
//! Chrome documentation can be found here: https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints

use api::{known_good_versions, last_known_good_versions};

pub mod api;
pub mod chromedriver;
