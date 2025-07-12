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

pub struct Doc<State>(pub State);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{doc::indexed::SearchKey, logging::init_logger};
    use log::debug;
    use rustdoc_types::{Crate, Id};

    #[test]
    fn fetch() {
        init_logger();

        let krate = Doc::from_docs("playground-api", "latest").unwrap();
        let krate = krate.fetch().unwrap();
        let krate = krate.decompress().unwrap();
        let krate = krate.parse().unwrap();
        let ast = krate.0.ast.clone();
        let krate = krate.build_search_index();
        krate.save_index("index").unwrap();

        let hit = krate.search("playgrnd:clnt:clnt:exe", 1);
        println!("{hit:#?}");

        is_non_option(ast, krate.0.search_index.clone());
    }

    fn is_non_option(krate: Crate, index: Vec<SearchKey>) {
        let mut count = 0;
        index.iter().for_each(|key| {
            let id = Id(key.id);
            let item = krate.index.get(&id).unwrap();
            let name = &item.span;
            if name.is_none() {
                count += 1;
            }
        });
        debug!("how many don't have a name {count}");
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
