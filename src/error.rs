use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("URL parse error: {0}")]
    UrlParsing(#[from] url::ParseError),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
