use crate::api::version::Version;
use crate::api::Download;
use crate::error::Result;
use serde::Deserialize;

/// Download links for Chrome and ChromeDriver binaries.
#[derive(Debug, Clone, Deserialize)]
pub struct Downloads {
    /// Download links for Chrome binaries for various platforms.
    pub chrome: Vec<Download>,

    /// Download links for ChromeDriver binaries for various platforms.
    /// Note: Some older Chrome versions may not have ChromeDriver downloads available!
    pub chromedriver: Option<Vec<Download>>,
}

/// An entry of the "known good versions" API response, representing one version.
///
/// No `Channel` information is provided.
#[derive(Debug, Clone, Deserialize)]
pub struct VersionWithoutChannel {
    /// The version identifier.
    pub version: Version,

    /// The Chrome revision number.
    pub revision: String,

    /// Available downloads for this version.
    pub downloads: Downloads,
}

/// Response structure for the "known good versions" API endpoint.
///
/// Contains a comprehensive list of Chrome versions that have been tested and verified to work.
/// Unlike the "last known good versions" API, this includes all historical versions without
/// channel assignments.
#[derive(Debug, Clone, Deserialize)]
pub struct KnownGoodVersions {
    /// When this data was last updated.
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: time::OffsetDateTime,

    /// List of all known good Chrome versions.
    pub versions: Vec<VersionWithoutChannel>,
}

impl KnownGoodVersions {
    /// Fetches the list of all known good Chrome versions from the Chrome for Testing API.
    ///
    /// Returns a comprehensive list of Chrome versions that have been tested and verified to work.
    /// Unlike the "last known good versions" API, this includes all historical versions without
    /// channel assignments.
    pub async fn fetch(client: reqwest::Client) -> Result<Self> {
        /// JSON Example:
        /// ```json
        /// {
        ///     "version": "115.0.5763.0",
        ///     "revision": "1141961",
        ///     "downloads": {
        ///         "chrome": [
        ///             {
        ///                 "platform": "linux64",
        ///                 "url": "https://.../chrome-linux64.zip"
        ///             },
        ///             ...
        ///         ],
        ///         "chromedriver": [ /* <- Some versions don't have this field! */
        ///             {
        ///                 "platform": "linux64",
        ///                 "url": "https://.../chromedriver-linux64.zip"
        ///             },
        ///             ...
        ///         ]
        ///     }
        /// },
        /// ´´´
        const KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_URL: &str =
        "https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json";

        let result = client
            .get(KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_URL)
            .send()
            .await?
            .json::<Self>()
            .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::known_good_versions::KnownGoodVersions;
    use assertr::prelude::*;

    #[tokio::test]
    async fn can_query_known_good_versions_api_endpoint_and_deserialize_response() {
        let result = KnownGoodVersions::fetch(reqwest::Client::new()).await;
        let data = assert_that(result).is_ok().unwrap_inner();
        dbg!(&data);
    }
}
