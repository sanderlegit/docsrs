mod search;
mod search_index;

use super::Doc;
use rustdoc_types::{Crate, Id, Item, ItemSummary};

pub struct Indexed {
    pub ast: Crate,
    search_index: Option<Vec<(Id, String)>>,
    matcher: fuzzy_matcher::skim::SkimMatcherV2,
}

impl Doc<Indexed> {
    pub(super) fn new(ast: Crate) -> Self {
        Self(Indexed {
            ast,
            search_index: None,
            matcher: fuzzy_matcher::skim::SkimMatcherV2::default(),
        })
    }

    pub fn get(&self, id: u32) -> Option<&Item> {
        self.0.ast.index.get(&Id(id))
    }

    pub fn get_path(&self, id: u32) -> Option<&ItemSummary> {
        self.0.ast.paths.get(&Id(id))
    }
}
