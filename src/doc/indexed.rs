mod search;

use super::Doc;
use crate::{Error, Item};
use std::{collections::HashMap, fs::OpenOptions, io::Write, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SearchKey {
    pub(crate) id: u32,
    pub(crate) key: String,
}

pub struct Indexed {
    pub(crate) search_index: Vec<SearchKey>,
    items: HashMap<u32, Item>,
    matcher: fuzzy_matcher::skim::SkimMatcherV2,
}

impl Doc<Indexed> {
    pub(super) fn new(search_index: Vec<SearchKey>, items: HashMap<u32, Item>) -> Self {
        Self(Indexed {
            search_index,
            items,
            matcher: fuzzy_matcher::skim::SkimMatcherV2::default(),
        })
    }

    pub fn save_index<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        self.0
            .search_index
            .iter()
            .try_for_each(|key| writeln!(file, "{key:?}"))?;

        Ok(())
    }
}
