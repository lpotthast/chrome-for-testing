use crate::api::channel::Channel;
use crate::api::version::Version;
use crate::api::Download;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct VersionInChannel {
    pub channel: Channel,
    pub version: Version,
    pub revision: String,
    pub downloads: Downloads,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LastKnownGoodVersions {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: time::OffsetDateTime,
    pub channels: HashMap<Channel, VersionInChannel>,
}

pub async fn request(client: reqwest::Client) -> anyhow::Result<LastKnownGoodVersions> {
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
    const LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_URL: &str =
        "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";

    let result = client
        .get(LAST_KNOWN_GOOD_VERSIONS_WITH_DOWNLOADS_JSON_URL)
        .send()
        .await?
        .json::<LastKnownGoodVersions>()
        .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_query_last_known_good_versions_api_endpoint_and_deserialize_response(
    ) -> anyhow::Result<()> {
        let data = request(reqwest::Client::new()).await?;
        dbg!(&data);
        Ok(())
    }
}
