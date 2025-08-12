use super::{Doc, Parsed};
use crate::doc::indexed::SearchKey;
use rustdoc_types::{Crate, Enum, Id, ItemEnum};
use std::collections::HashMap;

impl Doc<Parsed> {
    pub(super) fn search_keys_enums<'a>(
        &'a self,
        krate: &'a Crate,
        enm: &'a Enum,
        base_path: &str,
        parent_map: &HashMap<&'a Id, &'a Id>,
        path_cache: &mut HashMap<&'a Id, Vec<String>>,
    ) -> Vec<SearchKey> {
        let variant_keys = enm.variants.iter().filter_map(move |variant_id| {
            let variant_item = krate.index.get(variant_id)?;
            let name = variant_item.name.as_deref()?;
            Some(SearchKey {
                id: variant_id.0.to_string(),
                key: format!("{base_path}::{name}"),
            })
        });

        let impl_keys = enm
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
            .flatten();

        variant_keys.chain(impl_keys).collect()
    }
}
