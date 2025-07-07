use super::{Doc, Indexed};
use fuzzy_matcher::FuzzyMatcher;
use rustdoc_types::{Id, Item};
use std::cmp::Reverse;

impl Doc<Indexed> {
    pub fn search(&self, query: &str, n: impl Into<Option<usize>>) -> Option<Vec<&Item>> {
        let index = match &self.0.search_index {
            Some(idx) => idx,
            None => return None,
        };

        let matcher = &self.0.matcher;
        let mut results: Vec<(Id, i64)> = index
            .iter()
            .filter_map(|(id, searchable)| {
                matcher
                    .fuzzy_match(&searchable.to_lowercase(), &query.to_lowercase())
                    .map(|score| (*id, score))
            })
            .collect();

        results.sort_unstable_by_key(|&(_, score)| Reverse(score));

        let mut matches = results;
        let n = n.into();
        if n == Some(0) || matches.is_empty() {
            return None;
        }

        if let Some(n) = n {
            matches.truncate(n);
        }

        Some(
            matches
                .iter()
                .filter_map(|(id, _)| self.0.ast.index.get(id))
                .collect(),
        )
    }
}
