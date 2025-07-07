#[cfg(feature = "fetch")]
mod remote;

#[cfg(feature = "decompress")]
mod fetched;

mod rawjson;

mod indexed;

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
        let mut krate = krate.parse().unwrap();

        krate.build_search_index();
        let hits = krate.search("playgrnd:clnt:clnt:exe", 1);
        println!("{hits:#?}");
        krate.save_index("index").unwrap();
    }

    #[test]
    fn from_json() {
        init_logger();

        let std = Doc::from_json(
            "/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/json/std.json",
        )
        .unwrap();
        let mut std = std.parse().unwrap();

        std.build_search_index();
        let hits = std.search("std::fs::File", 1).unwrap();
        println!("{:#?}", hits[0])
    }
}
