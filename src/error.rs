/// Errors that can occur during documentation processing
///
/// This enum covers all possible error conditions that may arise when fetching,
/// decompressing, parsing, or processing documentation data.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request failed when fetching documentation from docs.rs
    #[cfg(feature = "fetch")]
    #[error("http error: {0}")]
    Http(#[from] ureq::Error),

    /// Invalid URL format when constructin docs.rs endpoint
    #[error("url error: {0:?}")]
    Url(#[from] url::ParseError),

    /// File system operation failed (reading/writing files and decompression)
    #[error("io error: {0:?}")]
    Io(#[from] std::io::Error),

    /// JSON parsing failed when deserializing documentation data
    #[error("serde error: {0:?}")]
    Serde(#[from] serde_json::Error),
}
