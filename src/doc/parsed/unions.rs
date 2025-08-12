use super::{Doc, Parsed};
use crate::doc::indexed::SearchKey;
use rustdoc_types::{Crate, Id, ItemEnum, Union};
use std::collections::HashMap;

impl Doc<Parsed> {
    pub(super) fn search_keys_unions<'a>(
        &self,
        krate: &'a Crate,
        union: &'a Union,
        base_path: &'a str,
        parent_map: &HashMap<&'a Id, &'a Id>,
        path_cache: &mut HashMap<&'a Id, Vec<String>>,
    ) -> impl Iterator<Item = SearchKey> + 'a {
        union
            .impls
            .iter()
            .filter_map(move |impl_id| {
                let impl_item = krate.index.get(impl_id)?;

                let ItemEnum::Impl(impl_block) = &impl_item.inner else {
                    return None;
                };

                Some(
                    self.impl_method_keys(krate, impl_block, base_path, parent_map, path_cache)
                        .into_iter(),
                )
            })
            .flatten()
    }
}
