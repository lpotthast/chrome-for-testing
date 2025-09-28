use crate::api::version::Version;
use platform::Platform;
use reqwest::Url;
use serde::Deserialize;
use std::sync::LazyLock;

/// Chrome release channel definitions.
pub mod channel;

/// Platform identification for different operating systems and architectures.
pub mod platform;

/// Version parsing and representation.
pub mod version;

/// API request for a list of working releases. None are assigned to any channel.
pub mod known_good_versions;

/// The last working releases for each channel.
pub mod last_known_good_versions;

/// The standard chrome-for-testing API endpoint protocol and hostname.
///
/// Consult <https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints>
/// for verification.
pub static API_BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://googlechromelabs.github.io").expect("Valid URL"));

/// Represents a download link for a specific platform.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Download {
    /// The target platform for this download.
    pub platform: Platform,

    /// The download URL.
    pub url: String,
}

/// Trait for types that contain a version identifier.
pub trait HasVersion {
    /// Returns the version identifier.
    fn version(&self) -> Version;
}

impl HasVersion for known_good_versions::VersionWithoutChannel {
    fn version(&self) -> Version {
        self.version
    }
}

impl HasVersion for last_known_good_versions::VersionInChannel {
    fn version(&self) -> Version {
        self.version
    }
}
