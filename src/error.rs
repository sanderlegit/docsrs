#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "fetch")]
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(feature = "fetch")]
    #[error("url error: {0:?}")]
    Url(#[from] url::ParseError),

    #[error("io error: {0:?}")]
    IO(#[from] std::io::Error),

    #[error("serde error: {0:?}")]
    Serde(#[from] serde_json::Error),
}
