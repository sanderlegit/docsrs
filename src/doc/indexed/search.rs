use super::{Doc, Indexed, SearchKey};
use crate::Item;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Reverse;

impl Doc<Indexed> {
    pub fn search(&self, query: &str, n: impl Into<Option<usize>>) -> Option<Vec<&Item>> {
        let index = &self.0.search_index;

        let matcher = &self.0.matcher;
        let mut results = index
            .iter()
            .filter_map(|SearchKey { id, key }| {
                matcher
                    .fuzzy_match(&key.to_lowercase(), &query.to_lowercase())
                    .map(|score| (*id, score))
            })
            .collect::<Vec<(u32, i64)>>();

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
                .filter_map(|(id, _)| self.0.items.get(id))
                .collect(),
        )
    }
}
