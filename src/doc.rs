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

    pub async fn decompress(self) -> Result<Doc<Json>, ()> {
        let decompressed_data = tokio::task::spawn_blocking(move || {
            use std::io::Read;

            let mut decoder = zstd::Decoder::new(&self.0.0[..]).unwrap();
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).unwrap();
            decompressed
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

    #[tokio::test]
    async fn success() {
        let krate = Doc::from_docs("tokio", "latest");
        let krate = krate.fetch().await.unwrap();
        let krate = krate.decompress().await.unwrap();
        let krate = krate.parse().await.unwrap();
        println!("{:?}", krate.0.ast);
    }
}
