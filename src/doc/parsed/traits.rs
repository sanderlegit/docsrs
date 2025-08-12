use super::{Doc, Parsed};
use crate::doc::indexed::SearchKey;
use rustdoc_types::{Crate, Trait};

impl Doc<Parsed> {
    pub(super) fn search_keys_traits<'a>(
        krate: &'a Crate,
        trait_item: &'a Trait,
        base_path: &'a str,
    ) -> impl Iterator<Item = SearchKey> + 'a {
        trait_item.items.iter().filter_map(move |item_id| {
            let item = krate.index.get(item_id)?;
            let name = item.name.as_deref()?;
            Some(SearchKey {
                id: item_id.0,
                key: format!("{base_path}::{name}"),
            })
        })
    }
}
