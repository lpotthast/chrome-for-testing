use crate::api::channel::Channel;
use crate::api::version::Version;
use crate::api::{Download, API_BASE_URL};
use crate::error::Error;
use serde::Deserialize;
use std::collections::HashMap;

const LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH: &str =
    "/chrome-for-testing/last-known-good-versions-with-downloads.json";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Downloads {
    pub chrome: Vec<Download>,

    pub chromedriver: Vec<Download>,

    /// The "chrome-headless-shell" binary provides the "old" headless mode of Chrome, as described
    /// in [this blog post](https://developer.chrome.com/blog/chrome-headless-shell).
    /// For standard automated web-ui testing, you should pretty much always use the regular
    /// `chrome` binary instead.
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<Download>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct VersionInChannel {
    pub channel: Channel,
    pub version: Version,
    pub revision: String,
    pub downloads: Downloads,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LastKnownGoodVersions {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: time::OffsetDateTime,
    pub channels: HashMap<Channel, VersionInChannel>,
}

pub async fn request(client: reqwest::Client) -> Result<LastKnownGoodVersions, Error> {
    request_with_base_url(client, API_BASE_URL.clone()).await
}

pub async fn request_with_base_url(
    client: reqwest::Client,
    base_url: reqwest::Url,
) -> Result<LastKnownGoodVersions, Error> {
    let last_known_good_versions = client
        .get(base_url.join(LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)?)
        .send()
        .await?
        .json::<LastKnownGoodVersions>()
        .await?;
    Ok(last_known_good_versions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::platform::Platform;
    use assertr::prelude::*;
    use time::macros::datetime;
    use url::Url;

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

        let data = request_with_base_url(reqwest::Client::new(), url)
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
