use crate::api::channel::Channel;
use crate::api::version::Version;
use crate::api::{Download, API_BASE_URL};
use crate::error::Result;
use serde::Deserialize;
use std::collections::HashMap;

/// JSON Example:
/// ```json
/// {
///     "timestamp": "2025-01-05T22:09:08.729Z",
///     "channels": {
///         "Stable": {
///             "channel": "Stable",
///             "version": "131.0.6778.204",
///             "revision": "1368529",
///             "downloads": {
///                 "chrome": [
///                     {
///                         "platform": "linux64",
///                         "url": "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.204/linux64/chrome-linux64.zip"
///                     },
///                     {
///                         "platform": "mac-arm64",
///                         "url": "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.204/mac-arm64/chrome-mac-arm64.zip"
///                     },
///                     {
///                         "platform": "mac-x64",
///                         "url": "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.204/mac-x64/chrome-mac-x64.zip"
///                     },
///                     {
///                         "platform": "win32",
///                         "url": "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.204/win32/chrome-win32.zip"
///                     },
///                     {
///                         "platform": "win64",
///                         "url": "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.204/win64/chrome-win64.zip"
///                     }
///                 ],
///                 "chromedriver": [
///                     {
///                         "platform": "linux64",
///                         "url": "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.204/linux64/chromedriver-linux64.zip"
///                     },
///                     ...
///                 ],
///                 "chrome-headless-shell": [
///                     {
///                         "platform": "linux64",
///                         "url": "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.204/linux64/chrome-headless-shell-linux64.zip"
///                     },
///                     ...
///                 ]
///             }
///         },
///         "Beta": {
///             "channel": "Beta",
///             "version": "132.0.6834.57",
///             "revision": "1381561",
///             "downloads": {
///                 "chrome": [
///                    ...
///                 ],
///                 "chromedriver": [
///                     ...
///                 ],
///                 "chrome-headless-shell": [
///                     ...
///                 ]
///             }
///         },
///         "Dev": { ... },
///         "Canary": { ... }
///     }
/// }
/// ´´´
const LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH: &str =
    "/chrome-for-testing/last-known-good-versions-with-downloads.json";

/// Download links for Chrome, ChromeDriver, and Chrome Headless Shell binaries for various
/// platforms.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Downloads {
    /// Download links for Chrome binaries for various platforms.
    pub chrome: Vec<Download>,

    /// Download links for ChromeDriver binaries for various platforms.
    pub chromedriver: Vec<Download>,

    /// The "chrome-headless-shell" binary provides the "old" headless mode of Chrome, as described
    /// in [this blog post](https://developer.chrome.com/blog/chrome-headless-shell).
    /// For standard automated web-ui testing, you should pretty much always use the regular
    /// `chrome` binary instead.
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<Download>,
}

/// A Chrome version entry with channel information.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct VersionInChannel {
    /// The release channel this version belongs to.
    pub channel: Channel,

    /// The version identifier.
    pub version: Version,

    /// The Chromium revision number.
    pub revision: String,

    /// Available downloads for this version.
    pub downloads: Downloads,
}

/// Response structure for the "last known good versions" API endpoint.
///
/// Contains the most recent version for each Chrome release channel (Stable, Beta, Dev, Canary).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LastKnownGoodVersions {
    /// When this data was last updated.
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: time::OffsetDateTime,

    /// The latest known good version for each release channel.
    pub channels: HashMap<Channel, VersionInChannel>,
}

impl LastKnownGoodVersions {
    /// Fetches the last known good versions from the Chrome for Testing API.
    ///
    /// Returns the most recent version for each Chrome release channel (Stable, Beta, Dev, Canary).
    pub async fn fetch(client: reqwest::Client) -> Result<Self> {
        Self::fetch_with_base_url(client, API_BASE_URL.clone()).await
    }

    pub async fn fetch_with_base_url(
        client: reqwest::Client,
        base_url: reqwest::Url,
    ) -> Result<LastKnownGoodVersions> {
        let last_known_good_versions = client
            .get(base_url.join(LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)?)
            .send()
            .await?
            .json::<Self>()
            .await?;
        Ok(last_known_good_versions)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::channel::Channel;
    use crate::api::last_known_good_versions::{
        Downloads, LastKnownGoodVersions, VersionInChannel,
        LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH,
    };
    use crate::api::platform::Platform;
    use crate::api::version::Version;
    use crate::api::Download;
    use assertr::prelude::*;
    use std::collections::HashMap;
    use time::macros::datetime;
    use url::Url;

    #[tokio::test]
    async fn can_request_from_real_world_endpoint() {
        let result = LastKnownGoodVersions::fetch(reqwest::Client::new()).await;
        assert_that(result).is_ok();
    }

    //noinspection DuplicatedCode
    #[tokio::test]
    async fn can_query_last_known_good_versions_api_endpoint_and_deserialize_response() {
        let mut server = mockito::Server::new_async().await;

        let _mock = server
            .mock("GET", LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!(
                "./../../test-data/last_known_good_versions_test_response.json"
            ))
            .create();

        let url: Url = server.url().parse().unwrap();

        let data = LastKnownGoodVersions::fetch_with_base_url(reqwest::Client::new(), url)
            .await
            .unwrap();

        assert_that(data).is_equal_to(LastKnownGoodVersions {
            timestamp: datetime!(2025-01-17 10:09:31.683 UTC),
            channels: HashMap::from([
                (
                    Channel::Stable,
                    VersionInChannel {
                        channel: Channel::Stable,
                        version: Version { major: 132, minor: 0, patch: 6834, build: 83 },
                        revision: String::from("1381561"),
                        downloads: Downloads {
                            chrome: vec![
                                Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/linux64/chrome-linux64.zip") },
                                Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/mac-arm64/chrome-mac-arm64.zip") },
                                Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/mac-x64/chrome-mac-x64.zip") },
                                Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/win32/chrome-win32.zip") },
                                Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/win64/chrome-win64.zip") },
                            ],
                            chromedriver: vec![
                                Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/linux64/chromedriver-linux64.zip") },
                                Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/mac-arm64/chromedriver-mac-arm64.zip") },
                                Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/mac-x64/chromedriver-mac-x64.zip") },
                                Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/win32/chromedriver-win32.zip") },
                                Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/win64/chromedriver-win64.zip") },
                            ],
                            chrome_headless_shell: vec![
                                Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/linux64/chrome-headless-shell-linux64.zip") },
                                Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                                Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/mac-x64/chrome-headless-shell-mac-x64.zip") },
                                Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/win32/chrome-headless-shell-win32.zip") },
                                Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/132.0.6834.83/win64/chrome-headless-shell-win64.zip") },
                            ],
                        },
                    }
                ),
                (Channel::Beta, VersionInChannel {
                    channel: Channel::Beta,
                    version: Version { major: 133, minor: 0, patch: 6943, build: 16 },
                    revision: String::from("1402768"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/win64/chrome-win64.zip") },
                        ],
                        chromedriver: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/win64/chromedriver-win64.zip") },
                        ],
                        chrome_headless_shell: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/linux64/chrome-headless-shell-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/mac-x64/chrome-headless-shell-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/win32/chrome-headless-shell-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/133.0.6943.16/win64/chrome-headless-shell-win64.zip") },
                        ],
                    },
                }),
                (Channel::Dev, VersionInChannel {
                    channel: Channel::Dev,
                    version: Version { major: 134, minor: 0, patch: 6958, build: 2 },
                    revision: String::from("1406477"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/win64/chrome-win64.zip") },
                        ],
                        chromedriver: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/win64/chromedriver-win64.zip") },
                        ],
                        chrome_headless_shell: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/linux64/chrome-headless-shell-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/mac-x64/chrome-headless-shell-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/win32/chrome-headless-shell-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6958.2/win64/chrome-headless-shell-win64.zip") },
                        ],
                    },
                }),
                (Channel::Canary, VersionInChannel {
                    channel: Channel::Canary,
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
                        chromedriver: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win64/chromedriver-win64.zip") },
                        ],
                        chrome_headless_shell: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/linux64/chrome-headless-shell-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/mac-x64/chrome-headless-shell-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win32/chrome-headless-shell-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/134.0.6962.0/win64/chrome-headless-shell-win64.zip") },
                        ],
                    },
                })
            ]),
        });
    }
}
