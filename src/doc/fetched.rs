use super::{Doc, rawjson::RawJson};
use crate::Error;
use log::debug;
use std::path::Path;
use tokio::fs;

pub struct Fetched(Vec<u8>);

impl Doc<Fetched> {
    pub(super) fn new(data: Vec<u8>) -> Self {
        Self(Fetched(data))
    }

    pub async fn from_zst<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let compressed_data = fs::read(path).await?;
        Ok(Self(Fetched(compressed_data)))
    }

    fn is_compressed(data: &[u8]) -> bool {
        use zstd::zstd_safe::zstd_sys::ZSTD_MAGICNUMBER;
        debug!("{:?}", &data[..4]);

        data.len() >= 4
            && u32::from_le_bytes(data[..4].try_into().unwrap_or([0; 4])) == ZSTD_MAGICNUMBER
    }

    pub async fn decompress(self) -> Result<Doc<RawJson>, Error> {
        let decompressed_data = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, Error> {
            use std::io::Read;

            let mut data = self.0.0;

            while Self::is_compressed(&data) {
                let mut decoder = zstd::Decoder::new(&data[..])?;
                let mut buffer = Vec::new();
                decoder.read_to_end(&mut buffer)?;
                data = buffer;
                println!("decompression done");
            }

            Ok(data)
        })
        .await??;

        Ok(<Doc<RawJson>>::new(decompressed_data))
    }
}
