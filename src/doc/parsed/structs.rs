use super::{Doc, Parsed};
use crate::doc::indexed::SearchKey;
use rustdoc_types::{Crate, ItemEnum, Struct};

impl Doc<Parsed> {
    pub(super) fn search_keys_structs<'a>(
        krate: &'a Crate,
        strukt: &'a Struct,
        base_path: &'a str,
    ) -> impl Iterator<Item = SearchKey> + 'a {
        strukt
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
