use crate::api::version::Version;
use crate::api::Download;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Downloads {
    pub chrome: Vec<Download>,
    pub chromedriver: Option<Vec<Download>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionWithoutChannel {
    pub version: Version,
    pub revision: String,
    pub downloads: Downloads,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KnownGoodVersions {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: time::OffsetDateTime,
    pub versions: Vec<VersionWithoutChannel>,
}

pub async fn request(client: reqwest::Client) -> anyhow::Result<KnownGoodVersions> {
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
        .json::<KnownGoodVersions>()
        .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn can_query_known_good_versions_api_endpoint_and_deserialize_response() -> anyhow::Result<()> {
        let data = super::request(reqwest::Client::new()).await?;
        dbg!(&data);
        Ok(())
    }
}
