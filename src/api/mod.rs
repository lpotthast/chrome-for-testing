use crate::api::version::Version;
use crate::error::Error;
use platform::Platform;
use reqwest::Url;
use rootcause::prelude::ResultExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Download {
    /// The target platform for this download.
    pub platform: Platform,

    /// The download URL.
    pub url: String,
}

impl Download {
    /// Parses the download URL into a typed [`Url`].
    ///
    /// # Errors
    ///
    /// Returns an error if the upstream URL string is not a valid URL.
    pub fn parsed_url(&self) -> crate::Result<Url> {
        Url::parse(&self.url).context_to::<Error>().attach_with(|| {
            format!(
                "while parsing Chrome for Testing download URL: '{}'",
                self.url
            )
        })
    }
}

/// Extension trait for download slices, providing platform-based lookup.
pub trait DownloadsByPlatform {
    /// Returns the download entry for the given platform, if available.
    fn for_platform(&self, platform: Platform) -> Option<&Download>;
}

impl DownloadsByPlatform for [Download] {
    fn for_platform(&self, platform: Platform) -> Option<&Download> {
        self.iter().find(|d| d.platform == platform)
    }
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

pub(crate) async fn fetch_endpoint<T>(
    client: &reqwest::Client,
    base_url: &Url,
    path: &str,
    endpoint_name: &str,
) -> crate::Result<T>
where
    T: DeserializeOwned,
{
    let url = base_url.join(path).context_to::<Error>().attach_with(|| {
        format!("while joining Chrome for Testing {endpoint_name} endpoint path: {path}")
    })?;

    let result = client
        .get(url)
        .send()
        .await
        .context_to::<Error>()
        .attach_with(|| format!("while sending Chrome for Testing {endpoint_name} request"))?
        .error_for_status()
        .context_to::<Error>()?
        .json::<T>()
        .await
        .context_to::<Error>()
        .attach_with(|| {
            format!("while deserializing Chrome for Testing {endpoint_name} response")
        })?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn parsed_url_returns_typed_url() {
        let download = Download {
            platform: Platform::Linux64,
            url: String::from("https://example.com/chrome.zip"),
        };

        assert_that!(download.parsed_url().map(|url| url.to_string()))
            .is_ok()
            .is_equal_to("https://example.com/chrome.zip");
    }

    #[test]
    fn parsed_url_reports_invalid_urls() {
        let download = Download {
            platform: Platform::Linux64,
            url: String::from("not a url"),
        };

        let err = download.parsed_url().unwrap_err();

        let Error::UrlParsing(url_error) = err.current_context() else {
            panic!("expected URL parse error, got: {:?}", err.current_context());
        };

        assert_that!(url_error.to_string()).contains("relative URL without a base");
    }

    #[tokio::test]
    async fn fetch_endpoint_path_is_root_relative_when_base_url_has_path_prefix() {
        let mut server = mockito::Server::new_async().await;
        let endpoint_path = "/chrome-for-testing/test-endpoint.json";

        let _mock = server
            .mock("GET", endpoint_path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ok":true}"#)
            .create();

        let url: Url = format!("{}/prefix/", server.url()).parse().unwrap();

        let data = fetch_endpoint::<serde_json::Value>(
            &reqwest::Client::new(),
            &url,
            endpoint_path,
            "TestEndpoint",
        )
        .await
        .unwrap();

        assert_that!(data["ok"].as_bool()).is_equal_to(Some(true));
    }
}
