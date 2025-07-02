use super::{Doc, fetched::Fetched};
use crate::Error;

pub struct Remote {
    url: url::Url,
    reqwest_client: reqwest::Client,
}

impl Doc<Remote> {
    pub fn from_docs(crate_name: &str, version: &str) -> Result<Self, Error> {
        Ok(Doc(Remote {
            url: url::Url::parse(&format!(
                "https://docs.rs/crate/{crate_name}/{version}/json.zst"
            ))?,
            reqwest_client: reqwest::Client::new(),
        }))
    }

    pub async fn fetch(self) -> Result<Doc<Fetched>, Error> {
        let res = self.0.reqwest_client.get(self.0.url).send().await?;

        let bytes = res.bytes().await?;
        let bytes = bytes.to_vec();

        Ok(<Doc<Fetched>>::new(bytes))
    }
}
