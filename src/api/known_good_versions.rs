use crate::api::version::Version;
use crate::api::{Download, API_BASE_URL};
use crate::error::Error;
use serde::Deserialize;

const KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH: &str =
    "/chrome-for-testing/known-good-versions-with-downloads.json";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Downloads {
    pub chrome: Vec<Download>,
    pub chromedriver: Option<Vec<Download>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct VersionWithoutChannel {
    pub version: Version,
    pub revision: String,
    pub downloads: Downloads,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct KnownGoodVersions {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: time::OffsetDateTime,
    pub versions: Vec<VersionWithoutChannel>,
}

pub async fn request(client: reqwest::Client) -> Result<KnownGoodVersions, Error> {
    request_with_base_url(client, API_BASE_URL.clone()).await
}

pub async fn request_with_base_url(
    client: reqwest::Client,
    base_url: reqwest::Url,
) -> Result<KnownGoodVersions, Error> {
    let result = client
        .get(base_url.join(KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH)?)
        .send()
        .await?
        .json::<KnownGoodVersions>()
        .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::api::known_good_versions::{
        Downloads, KnownGoodVersions, VersionWithoutChannel,
        KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_PATH,
    };
    use crate::api::platform::Platform;
    use crate::api::version::Version;
    use crate::api::Download;
    use assertr::prelude::*;
    use time::macros::datetime;
    use url::Url;

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

        let data = super::request_with_base_url(reqwest::Client::new(), url)
            .await
            .unwrap();

        assert_that(data).is_equal_to(KnownGoodVersions {
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
