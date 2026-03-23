use std::borrow::Cow;
use thiserror::Error;

/// Errors that can occur when using this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A URL could not be parsed.
    #[error("URL parse error: {0}")]
    UrlParsing(#[from] url::ParseError),

    /// An HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The current platform is not supported by `chrome-for-testing`.
    #[error("Platform (os: {os}, arch: {arch}) is not supported.")]
    UnsupportedPlatform {
        /// The operating system name, e.g. "linux".
        os: Cow<'static, str>,

        /// The system architecture name, e.g. "`x86_64`".
        arch: Cow<'static, str>,
    },
}

/// A convenience type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;
