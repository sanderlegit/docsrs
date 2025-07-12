#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "fetch")]
    #[error("http error: {0}")]
    Http(#[from] ureq::Error),

    #[cfg(feature = "fetch")]
    #[error("url error: {0:?}")]
    Url(#[from] url::ParseError),

    #[error("io error: {0:?}")]
    Io(#[from] std::io::Error),

    #[error("serde error: {0:?}")]
    Serde(#[from] serde_json::Error),
}
