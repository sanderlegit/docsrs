use log::debug;

#[cfg(feature = "decompress")]
use std::path::Path;
use tokio::fs;

pub struct Doc<State>(State);

#[cfg(feature = "fetch")]
pub struct Unfetched {
    crate_name: String,
    version: String,
    reqwest_client: reqwest::Client,
}

#[cfg(feature = "fetch")]
impl Doc<Unfetched> {
    pub fn from_docs(crate_name: &str, version: &str) -> Self {
        Doc(Unfetched {
            crate_name: crate_name.to_owned(),
            version: version.to_owned(),
            reqwest_client: reqwest::Client::new(),
        })
    }

    pub async fn fetch(self) -> Result<Doc<Compressed>, ()> {
        let url = url::Url::parse(&format!(
            "https://docs.rs/crate/{}/{}/json.zst",
            self.0.crate_name, self.0.version
        ))
        .unwrap();

        let res = self.0.reqwest_client.get(url).send().await.unwrap();

        let bytes = res.bytes().await.unwrap();
        let bytes = bytes.to_vec();

        Ok(Doc(Compressed(bytes)))
    }
}

#[cfg(feature = "decompress")]
pub struct Compressed(Vec<u8>);

#[cfg(feature = "decompress")]
impl Doc<Compressed> {
    pub async fn from_zst<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let compressed_data = fs::read(path).await.unwrap();
        Ok(Self(Compressed(compressed_data)))
    }

    fn is_compressed(data: &[u8]) -> bool {
        use zstd::zstd_safe::zstd_sys::ZSTD_MAGICNUMBER;
        debug!("{:?}", &data[..4]);

        data.len() >= 4
            && u32::from_le_bytes(data[..4].try_into().unwrap_or([0; 4])) == ZSTD_MAGICNUMBER
    }

    pub async fn decompress(self) -> Result<Doc<Json>, ()> {
        let decompressed_data = tokio::task::spawn_blocking(move || {
            use std::io::Read;

            let mut data = self.0.0;

            while Self::is_compressed(&data) {
                let mut decoder = zstd::Decoder::new(&data[..]).unwrap();
                let mut buffer = Vec::new();
                decoder.read_to_end(&mut buffer).unwrap();
                data = buffer;
                println!("decompression done");
            }

            data
        })
        .await
        .unwrap();

        Ok(Doc(Json(decompressed_data)))
    }
}

#[cfg(feature = "parse")]
pub struct Json(Vec<u8>);

#[cfg(feature = "parse")]
impl Doc<Json> {
    pub async fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let json = fs::read(path).await.unwrap();
        Ok(Doc(Json(json)))
    }

    pub async fn parse(self) -> Result<Doc<Parsed>, ()> {
        let ast = tokio::task::spawn_blocking(move || serde_json::from_slice(&self.0.0).unwrap())
            .await
            .unwrap();

        Ok(Doc(Parsed { ast }))
    }
}

pub struct Parsed {
    ast: rustdoc_types::Crate,
}

impl Doc<Parsed> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::init_logger;

    #[tokio::test]
    async fn fetch() {
        init_logger();

        let krate = Doc::from_docs("playground-api", "latest");
        let krate = krate.fetch().await.unwrap();
        let krate = krate.decompress().await.unwrap();
        let krate = krate.parse().await.unwrap();

        for (id, item) in &krate.0.ast.index {
            if let Some(name) = &item.name {
                if name == "Arg" {
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
