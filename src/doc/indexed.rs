use super::Doc;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use rustdoc_types::{Crate, Item};

pub struct Indexed {
    pub ast: Crate,
    search_index: Option<Vec<(Item, String)>>,
}

impl Doc<Indexed> {
    pub(super) fn new(ast: Crate) -> Self {
        Self(Indexed {
            ast,
            search_index: None,
        })
    }

    pub fn build_search_index(&mut self) {
        let krate = &self.0.ast;
        let index = krate
            .index
            .iter()
            .filter_map(|(id, item)| {
                let name = item.name.as_deref()?;
                let path_summary = krate.paths.get(id)?;

                let full_path = path_summary.path.join("::");

                let search_key = format!("{name} {full_path}");

                Some((item.clone(), search_key))
            })
            .collect();

        self.0.search_index = Some(index)
    }

    pub fn search(&self, query: &str) -> Vec<(&Item, i64)> {
        let matcher = SkimMatcherV2::default();

        let index = match &self.0.search_index {
            Some(idx) => idx,
            None => return Vec::new(),
        };

        let mut results: Vec<_> = index
            .iter()
            .filter_map(|(item, searchable)| {
                matcher
                    .fuzzy_match(searchable, query)
                    .map(|score| (item, score))
            })
            .collect();

        results.sort_unstable_by_key(|&(_, score)| -score);

        results
    }
}
