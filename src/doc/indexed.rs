mod search;

use std::collections::HashMap;

use super::Doc;
use rustdoc_types::Item;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct SearchKey {
    pub(super) id: u32,
    pub(super) key: String,
}

pub struct Indexed {
    search_index: Vec<SearchKey>,
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
}
