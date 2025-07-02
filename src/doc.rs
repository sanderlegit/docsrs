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
        let krate = krate.parse().await.unwrap();

        for (id, item) in &krate.0.ast.index {
            if let Some(name) = &item.name {
                if name == "Client" {
                    println!(
                        "{id:?}{}:{item:#?}",
                        item.docs.clone().unwrap_or_default().len()
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn from_json() {
        init_logger();

        let std = Doc::from_json(
            "/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/json/core.json",
        )
        .await
        .unwrap();
        let _std = std.parse().await.unwrap();
    }
}
