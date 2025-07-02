#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("url error: {0:?}")]
    Url(#[from] url::ParseError),

    #[error("io error: {0:?}")]
    IO(#[from] std::io::Error),

    #[error("tokio joinerror: {0:?}")]
    Join(#[from] tokio::task::JoinError),

    #[error("serde error: {0:?}")]
    Serde(#[from] serde_json::Error),
}
