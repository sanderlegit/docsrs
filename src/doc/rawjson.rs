use super::{Doc, indexed::Indexed};
use crate::Error;
use rustdoc_types::Crate;
use std::path::Path;
use tokio::fs;

pub struct RawJson(Vec<u8>);

impl Doc<RawJson> {
    pub(super) fn new(data: Vec<u8>) -> Self {
        Self(RawJson(data))
    }

    pub async fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let json = fs::read(path).await?;
        Ok(Doc(RawJson(json)))
    }

    pub async fn parse(self) -> Result<Doc<Indexed>, Error> {
        let ast = tokio::task::spawn_blocking(move || -> Result<Crate, Error> {
            Ok(serde_json::from_slice(&self.0.0)?)
        })
        .await??;

        Ok(<Doc<Indexed>>::new(ast))
    }
}
