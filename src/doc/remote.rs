use super::{Doc, compressed::Compressed};
use crate::Error;

pub struct Remote {
    url: url::Url,
}

impl Doc<Remote> {
    pub fn from_docs(crate_name: &str, version: &str) -> Result<Self, Error> {
        Ok(Doc(Remote {
            url: url::Url::parse(&format!(
                "https://docs.rs/crate/{crate_name}/{version}/json.zst"
            ))?,
        }))
    }

    pub fn fetch(self) -> Result<Doc<Compressed>, Error> {
        let mut res = ureq::get(self.0.url.as_str()).call()?;

        let bytes = res.body_mut().with_config().limit(u64::MAX).read_to_vec()?;

        Ok(<Doc<Compressed>>::new(bytes))
    }
}
