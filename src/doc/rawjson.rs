use super::{Doc, indexed::Indexed};
use crate::Error;
use std::{fs, path::Path};

pub struct RawJson(Vec<u8>);

impl Doc<RawJson> {
    pub(super) fn new(data: Vec<u8>) -> Self {
        Self(RawJson(data))
    }

    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let json = fs::read(path)?;
        Ok(Doc(RawJson(json)))
    }

    pub fn parse(self) -> Result<Doc<Indexed>, Error> {
        let ast = serde_json::from_slice(&self.0.0)?;

        Ok(<Doc<Indexed>>::new(ast))
    }
}
