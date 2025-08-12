#[cfg(feature = "fetch")]
mod remote;
#[cfg(feature = "fetch")]
pub use remote::Remote;

#[cfg(feature = "decompress")]
mod compressed;
#[cfg(feature = "decompress")]
pub use compressed::Compressed;

mod rawjson;
pub use rawjson::RawJson;

mod parsed;
pub use parsed::{Item, Parsed};

mod indexed;
pub use indexed::Indexed;

/// A generic wrapper for documentation data in different processing states.
///
/// This struct uses the type-state pattern to ensure compile-time safety when
/// transitioning between different stages of documentation processing. Each state
/// represents a different stage in the documentation pipeline.
///
/// # States
///
/// - [`Remote`] - Documentation URL ready to be fetched
/// - [`Compressed`] - Downloaded or opened compressed documentation data
/// - [`RawJson`] - Decompressed JSON data in bytes
/// - [`Parsed`] - Parsed documentation AST
/// - [`Indexed`] - Documentation with searchable index
///
/// # Example
///
/// ```rust
/// use docsrs::Doc;
///
/// let doc = Doc::from_docs("serde", "latest")?
///     .fetch()?
///     .decompress()?
///     .parse()?
///     .build_search_index();
///
/// let results = doc.search("Serialize", None);
/// ```
pub struct Doc<State>(pub State);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::init_logger;

    #[test]
    #[cfg(feature = "fetch")]
    fn fetch() {
        init_logger();

        let krate = Doc::from_docs("tokio", "latest").unwrap();
        let krate = krate.fetch().unwrap();
        let krate = krate.decompress().unwrap();
        let krate = krate.parse().unwrap();
        let krate = krate.build_search_index();
        krate.save_index("index").unwrap();

        let hit = krate.search("doc:from_docs", 1);
        println!("{hit:#?}");

        if let Some(item) = hit {
            let url = item[0]._url().unwrap().unwrap().to_string();
            println!("{url}")
        }
    }

    #[test]
    fn from_json() {
        init_logger();

        let std = Doc::from_json(
            "/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/json/std.json",
        )
        .unwrap();
        let std = std.parse().unwrap().build_search_index();

        let hit = std.search("std::fs::File", 1);
        println!("{hit:#?}")
    }
}
