use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Platform (os: {os}, arch: {arch}) is not supported.")]
    UnsupportedPlatform {
        os: Cow<'static, str>,
        arch: Cow<'static, str>,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
