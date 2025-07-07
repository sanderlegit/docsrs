use super::{Doc, fetched::Fetched};
use crate::Error;

pub struct Remote {
    url: url::Url,
    reqwest_client: reqwest::blocking::Client,
}

impl Doc<Remote> {
    pub fn from_docs(crate_name: &str, version: &str) -> Result<Self, Error> {
        Ok(Doc(Remote {
            url: url::Url::parse(&format!(
                "https://docs.rs/crate/{crate_name}/{version}/json.zst"
            ))?,
            reqwest_client: reqwest::blocking::Client::new(),
        }))
    }

    pub fn fetch(self) -> Result<Doc<Fetched>, Error> {
        let res = self.0.reqwest_client.get(self.0.url).send()?;

        let bytes = res.bytes()?;
        let bytes = bytes.to_vec();

        Ok(<Doc<Fetched>>::new(bytes))
    }
}
