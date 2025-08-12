use super::{Doc, Parsed};
use crate::doc::indexed::SearchKey;
use rustdoc_types::{Crate, ItemEnum, Union};

impl Doc<Parsed> {
    pub(super) fn search_keys_unions<'a>(
        krate: &'a Crate,
        union: &'a Union,
        base_path: &'a str,
    ) -> impl Iterator<Item = SearchKey> + 'a {
        union
            .impls
            .iter()
            .filter_map(move |impl_id| {
                let impl_item = krate.index.get(impl_id)?;

                let ItemEnum::Impl(impl_block) = &impl_item.inner else {
                    return None;
                };

                Some(Self::impl_method_keys(krate, impl_block, base_path))
            })
            .flatten()
    }
}
