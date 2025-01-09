use crate::api::version::Version;
use platform::Platform;
use serde::Deserialize;

pub mod channel;
pub mod platform;
pub mod version;

/// A long list of working releases. None are assigned to any channel.
pub mod known_good_versions;

/// The last working releases for each channel.
pub mod last_known_good_versions;

#[derive(Debug, Clone, Deserialize)]
pub struct Download {
    pub platform: Platform,
    pub url: String,
}

pub trait HasVersion {
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
