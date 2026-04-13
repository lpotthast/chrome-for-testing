use crate::api::channel::Channel;
use crate::api::platform::Platform;
use crate::api::version::Version;
use crate::api::{API_BASE_URL, Download, DownloadsByPlatform, fetch_endpoint};
use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
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
/// ```
const LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH: &str =
    "/chrome-for-testing/last-known-good-versions-with-downloads.json";

/// Download links for Chrome, `ChromeDriver`, and Chrome Headless Shell binaries for various
/// platforms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Downloads {
    /// Download links for Chrome binaries for various platforms.
    pub chrome: Vec<Download>,

    /// Download links for `ChromeDriver` binaries for various platforms.
    pub chromedriver: Vec<Download>,

    /// The "chrome-headless-shell" binary provides the "old" headless mode of Chrome, as described
    /// in [this blog post](https://developer.chrome.com/blog/chrome-headless-shell).
    /// For standard automated web-ui testing, you should pretty much always use the regular
    /// `chrome` binary instead.
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<Download>,
}

impl Downloads {
    /// Returns the Chrome download entry for the given platform, if available.
    #[must_use]
    pub fn chrome_for_platform(&self, platform: Platform) -> Option<&Download> {
        self.chrome.for_platform(platform)
    }

    /// Returns the `ChromeDriver` download entry for the given platform, if available.
    #[must_use]
    pub fn chromedriver_for_platform(&self, platform: Platform) -> Option<&Download> {
        self.chromedriver.for_platform(platform)
    }

    /// Returns the Chrome Headless Shell download entry for the given platform, if available.
    #[must_use]
    pub fn chrome_headless_shell_for_platform(&self, platform: Platform) -> Option<&Download> {
        self.chrome_headless_shell.for_platform(platform)
    }
}

/// A Chrome version entry with channel information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

fn deserialize_channels<'de, D>(
    deserializer: D,
) -> Result<HashMap<Channel, VersionInChannel>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let channels = HashMap::<Channel, VersionInChannel>::deserialize(deserializer)?;

    for (key, value) in &channels {
        if key != &value.channel {
            return Err(D::Error::custom(format!(
                "expected channels.{key}.channel to be {key}, got {}",
                value.channel
            )));
        }
    }

    Ok(channels)
}

/// Response structure for the "last known good versions" API endpoint.
///
/// Contains the most recent version for each Chrome release channel (Stable, Beta, Dev, Canary).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastKnownGoodVersions {
    /// When this data was last updated.
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: time::OffsetDateTime,

    /// The latest known good version for each release channel.
    ///
    /// The Chrome for Testing docs currently define Stable, Beta, Dev, and Canary for this
    /// endpoint, but this remains a map so the crate can preserve newly-added upstream channels
    /// instead of discarding them.
    #[serde(deserialize_with = "deserialize_channels")]
    channels: HashMap<Channel, VersionInChannel>,
}

impl LastKnownGoodVersions {
    /// Fetches the last known good versions from the Chrome for Testing API.
    ///
    /// Returns the most recent version for each Chrome release channel (Stable, Beta, Dev, Canary).
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, the response has an unsuccessful status, or
    /// deserialization fails.
    pub async fn fetch(client: &reqwest::Client) -> crate::Result<Self> {
        Self::fetch_with_base_url(client, &API_BASE_URL).await
    }

    /// Fetches from a custom base URL (useful for testing).
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, the response has an unsuccessful status, or
    /// deserialization fails.
    pub async fn fetch_with_base_url(
        client: &reqwest::Client,
        base_url: &reqwest::Url,
    ) -> crate::Result<LastKnownGoodVersions> {
        fetch_endpoint::<Self>(
            client,
            base_url,
            LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH,
            "LastKnownGoodVersions",
        )
        .await
    }

    /// Returns the version info for the given channel.
    #[must_use]
    pub fn channel(&self, channel: impl Borrow<Channel>) -> Option<&VersionInChannel> {
        self.channels.get(channel.borrow())
    }

    /// Returns the latest known good versions by release channel.
    #[must_use]
    pub fn channels(&self) -> &HashMap<Channel, VersionInChannel> {
        &self.channels
    }

    /// Returns the Stable channel version info, if present.
    #[must_use]
    pub fn stable(&self) -> Option<&VersionInChannel> {
        self.channel(Channel::Stable)
    }

    /// Returns the Beta channel version info, if present.
    #[must_use]
    pub fn beta(&self) -> Option<&VersionInChannel> {
        self.channel(Channel::Beta)
    }

    /// Returns the Dev channel version info, if present.
    #[must_use]
    pub fn dev(&self) -> Option<&VersionInChannel> {
        self.channel(Channel::Dev)
    }

    /// Returns the Canary channel version info, if present.
    #[must_use]
    pub fn canary(&self) -> Option<&VersionInChannel> {
        self.channel(Channel::Canary)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Download;
    use crate::api::channel::Channel;
    use crate::api::last_known_good_versions::{
        Downloads, LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH, LastKnownGoodVersions,
        VersionInChannel,
    };
    use crate::api::platform::Platform;
    use crate::api::version::Version;
    use crate::error::Error;
    use assertr::prelude::*;
    use std::collections::HashMap;
    use time::macros::datetime;
    use url::Url;

    // This test should not be `#[ignore]`, even though it hits the Chrome For Testing API.
    #[tokio::test]
    async fn can_request_from_real_world_endpoint() {
        let result = LastKnownGoodVersions::fetch(&reqwest::Client::new()).await;
        assert_that!(result).is_ok();
    }

    //noinspection DuplicatedCode
    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn can_query_last_known_good_versions_api_endpoint_and_deserialize_response() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!(
                "./../../test-data/last_known_good_versions_with_downloads_test_response.json"
            ))
            .create();

        let url: Url = server.url().parse().unwrap();

        let data = LastKnownGoodVersions::fetch_with_base_url(&reqwest::Client::new(), &url)
            .await
            .unwrap();

        assert_that!(data).is_equal_to(LastKnownGoodVersions {
            timestamp: datetime!(2026-04-13 08:53:52.841 UTC),
            channels: HashMap::from([
                (
                    Channel::Stable,
                    VersionInChannel {
                    channel: Channel::Stable,
                    version: Version { major: 147, minor: 0, patch: 7727, build: 56 },
                    revision: String::from("1596535"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/win64/chrome-win64.zip") },
                        ],
                        chromedriver: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/win64/chromedriver-win64.zip") },
                        ],
                        chrome_headless_shell: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/linux64/chrome-headless-shell-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/mac-x64/chrome-headless-shell-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/win32/chrome-headless-shell-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/147.0.7727.56/win64/chrome-headless-shell-win64.zip") },
                        ],
                    },
                    }
                ),
                (Channel::Beta, VersionInChannel {
                    channel: Channel::Beta,
                    version: Version { major: 148, minor: 0, patch: 7778, build: 5 },
                    revision: String::from("1610480"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/win64/chrome-win64.zip") },
                        ],
                        chromedriver: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/win64/chromedriver-win64.zip") },
                        ],
                        chrome_headless_shell: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/linux64/chrome-headless-shell-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/mac-x64/chrome-headless-shell-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/win32/chrome-headless-shell-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7778.5/win64/chrome-headless-shell-win64.zip") },
                        ],
                    },
                }),
                (Channel::Dev, VersionInChannel {
                    channel: Channel::Dev,
                    version: Version { major: 148, minor: 0, patch: 7766, build: 3 },
                    revision: String::from("1607787"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/win64/chrome-win64.zip") },
                        ],
                        chromedriver: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/win64/chromedriver-win64.zip") },
                        ],
                        chrome_headless_shell: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/linux64/chrome-headless-shell-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/mac-x64/chrome-headless-shell-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/win32/chrome-headless-shell-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/148.0.7766.3/win64/chrome-headless-shell-win64.zip") },
                        ],
                    },
                }),
                (Channel::Canary, VersionInChannel {
                    channel: Channel::Canary,
                    version: Version { major: 149, minor: 0, patch: 7789, build: 0 },
                    revision: String::from("1613465"),
                    downloads: Downloads {
                        chrome: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/linux64/chrome-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/mac-arm64/chrome-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/mac-x64/chrome-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/win32/chrome-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/win64/chrome-win64.zip") },
                        ],
                        chromedriver: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/linux64/chromedriver-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/mac-arm64/chromedriver-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/mac-x64/chromedriver-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/win32/chromedriver-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/win64/chromedriver-win64.zip") },
                        ],
                        chrome_headless_shell: vec![
                            Download { platform: Platform::Linux64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/linux64/chrome-headless-shell-linux64.zip") },
                            Download { platform: Platform::MacArm64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/mac-arm64/chrome-headless-shell-mac-arm64.zip") },
                            Download { platform: Platform::MacX64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/mac-x64/chrome-headless-shell-mac-x64.zip") },
                            Download { platform: Platform::Win32, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/win32/chrome-headless-shell-win32.zip") },
                            Download { platform: Platform::Win64, url: String::from("https://storage.googleapis.com/chrome-for-testing-public/149.0.7789.0/win64/chrome-headless-shell-win64.zip") },
                        ],
                    },
                }),
            ]),
        });
    }

    #[tokio::test]
    async fn unsuccessful_http_status_is_reported_as_request_error() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(include_str!(
                "./../../test-data/last_known_good_versions_with_downloads_test_response.json"
            ))
            .create();

        let url: Url = server.url().parse().unwrap();

        let err = LastKnownGoodVersions::fetch_with_base_url(&reqwest::Client::new(), &url)
            .await
            .unwrap_err();

        let Error::Request(request_error) = err.current_context() else {
            panic!("expected request error, got: {:?}", err.current_context());
        };

        assert_that!(request_error.status())
            .is_equal_to(Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[test]
    fn deserialization_rejects_channel_mismatch() {
        let json = include_str!(
            "./../../test-data/last_known_good_versions_with_downloads_test_response.json"
        )
        .replacen(r#""channel": "Stable""#, r#""channel": "Beta""#, 1);

        let result = serde_json::from_str::<LastKnownGoodVersions>(&json);

        assert_that!(result)
            .is_err()
            .derive(|it| it.to_string())
            .contains("expected channels.Stable.channel to be Stable, got Beta");
    }

    #[test]
    fn deserialization_preserves_unknown_channels() {
        let json = include_str!(
            "./../../test-data/last_known_good_versions_with_downloads_test_response.json"
        )
        .replacen(r#""Canary": {"#, r#""Extended": {"#, 1)
        .replacen(r#""channel": "Canary""#, r#""channel": "Extended""#, 1);

        let data = serde_json::from_str::<LastKnownGoodVersions>(&json).unwrap();
        let extended = Channel::Other(String::from("Extended"));

        assert_that!(data.canary()).is_none();
        assert_that!(data.channel(&extended))
            .is_some()
            .derive(|it| it.channel.clone())
            .is_equal_to(extended);
    }
}
