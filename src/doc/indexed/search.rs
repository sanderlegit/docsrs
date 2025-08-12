use super::{Doc, Indexed, SearchKey};
use crate::Item;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Reverse;

impl Doc<Indexed> {
    /// Performs fuzzy search on the indexed documentation
    ///
    /// Searches through all documentation items using fuzzy matching, returning
    /// results ranked by similarity score. The search is case-insensitive and matches
    /// against fully qualified item paths (e.g., "std::vec::Vec::push").
    ///
    /// # Arguments
    ///
    /// - `query` - The search term to match against
    /// - `n` - Maximum numbers of results to return (None for all matches)
    ///
    /// # Returns
    ///
    /// `Some(Vec<&Item>)` with matching items ranked by relevance, or `None` if matches found.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # fn main() -> Result<(), docsrs::Error> {
    /// use docsrs::Doc;
    /// let indexed_doc = Doc::from_json("path/to/docs.json")?.parse()?.build_search_index();
    /// // Search for up to 5 items matching "vec push"
    /// let results = indexed_doc.search("vec push", Some(5));
    /// let results = indexed_doc.search("vec push", 5); // this works too because `n` is `impl Into<Option<usize>>`
    ///
    /// // Get all matches
    /// let results = indexed_doc.search("HashMap", None);
    /// # Ok(())
    /// # }
    /// ```
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
