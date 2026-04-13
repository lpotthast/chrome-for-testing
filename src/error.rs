use rootcause::{Report, ReportConversion, markers};
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

impl<T> ReportConversion<url::ParseError, markers::Mutable, T> for Error
where
    Error: markers::ObjectMarkerFor<T>,
{
    fn convert_report(
        report: Report<url::ParseError, markers::Mutable, T>,
    ) -> Report<Self, markers::Mutable, T> {
        report.context_transform(Error::UrlParsing)
    }
}

impl<T> ReportConversion<reqwest::Error, markers::Mutable, T> for Error
where
    Error: markers::ObjectMarkerFor<T>,
{
    fn convert_report(
        report: Report<reqwest::Error, markers::Mutable, T>,
    ) -> Report<Self, markers::Mutable, T> {
        report.context_transform(Error::Request)
    }
}
