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

mod indexed;
pub use indexed::Indexed;

pub struct Doc<State>(pub State);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::init_logger;

    #[test]
    fn fetch() {
        init_logger();

        let krate = Doc::from_docs("playground-api", "latest").unwrap();
        let krate = krate.fetch().unwrap();
        let krate = krate.decompress().unwrap();
        let krate = krate.parse().unwrap();
        let krate = krate.build_search_index();
        krate.save_index("index").unwrap();

        let hits = krate.search("playgrnd:clnt:clnt:exe", 1);
        println!("{hits:#?}");
    }

    #[test]
    fn from_json() {
        init_logger();

        let std = Doc::from_json(
            "/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/json/std.json",
        )
        .unwrap();
        let std = std.parse().unwrap().build_search_index();

        let hits = std.search("std::fs::File", 1).unwrap();
        println!("{:#?}", hits[0])
    }
}
