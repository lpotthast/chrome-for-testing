use crate::api::version::Version;
use crate::api::{API_BASE_URL, Download};
use crate::error::Result;
use serde::{Deserialize, Serialize};

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
/// ```
const KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH: &str =
    "/chrome-for-testing/known-good-versions-with-downloads.json";

/// Download links for Chrome and `ChromeDriver` binaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Downloads {
    /// Download links for Chrome binaries for various platforms.
    pub chrome: Vec<Download>,

    /// Download links for `ChromeDriver` binaries for various platforms.
    /// Note: Some older Chrome versions may not have `ChromeDriver` downloads available!
    pub chromedriver: Option<Vec<Download>>,
}

/// An entry of the "known good versions" API response, representing one version.
///
/// No `Channel` information is provided.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request or deserialization fails.
    pub async fn fetch(client: &reqwest::Client) -> Result<Self> {
        Self::fetch_with_base_url(client, &API_BASE_URL).await
    }

    /// Fetches from a custom base URL (useful for testing).
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request or deserialization fails.
    pub async fn fetch_with_base_url(
        client: &reqwest::Client,
        base_url: &reqwest::Url,
    ) -> Result<Self> {
        let result = client
            .get(base_url.join(KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)?)
            .send()
            .await?
            .json::<Self>()
            .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Download;
    use crate::api::known_good_versions::KnownGoodVersions;
    use crate::api::platform::Platform;
    use crate::api::version::Version;
    use assertr::prelude::*;
    use time::macros::datetime;
    use url::Url;

    #[tokio::test]
    async fn can_request_from_real_world_endpoint() {
        let result = KnownGoodVersions::fetch(&reqwest::Client::new()).await;
        assert_that!(result).is_ok();
    }

    //noinspection DuplicatedCode
    #[tokio::test]
    async fn can_query_known_good_versions_api_endpoint_and_deserialize_response() {
        let mut server = mockito::Server::new_async().await;

        let _mock = server
            .mock("GET", KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!(
                "./../../test-data/known_good_versions_test_response.json"
            ))
            .create();

        let url: Url = server.url().parse().unwrap();

        let data = KnownGoodVersions::fetch_with_base_url(&reqwest::Client::new(), &url)
            .await
            .unwrap();

        assert_that!(data).is_equal_to(KnownGoodVersions {
            timestamp: datetime!(2025-01-17 10:09:31.689 UTC),
            versions: vec![
                VersionWithoutChannel {
                    version: Version { major: 113, minor: 0, patch: 5672, build: 0 },
                    revision: String::from("1121455"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/113.0.5672.0/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/113.0.5672.0/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/113.0.5672.0/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/113.0.5672.0/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/113.0.5672.0/win64/chrome-win64.zip") },
                        ],
                        chromedriver: None,
                    },
                },
                VersionWithoutChannel {
                    version: Version { major: 134, minor: 0, patch: 6962, build: 0 },
                    revision: String::from("1407692"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win64/chrome-win64.zip") },
                        ],
                        chromedriver: Some(vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win64/chromedriver-win64.zip") },
                        ]),
                    },
                },
            ],
        });
    }
}
