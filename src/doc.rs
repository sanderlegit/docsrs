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

    #[tokio::test]
    async fn fetch() {
        init_logger();

        let krate = Doc::from_docs("playground-api", "latest").unwrap();
        let krate = krate.fetch().await.unwrap();
        let krate = krate.decompress().await.unwrap();
        let mut krate = krate.parse().await.unwrap();

        krate.build_search_index();
        let finds = krate.search("Clnt");
        println!("{:#?}", &finds[..1]);
    }

    #[tokio::test]
    async fn from_json() {
        init_logger();

        let std = Doc::from_json(
            "/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/json/std.json",
        )
        .await
        .unwrap();
        let mut std = std.parse().await.unwrap();

        std.build_search_index();
        let hits = std.search("std::fs::File");
        println!("{:#?}", &hits[0])
    }
}
