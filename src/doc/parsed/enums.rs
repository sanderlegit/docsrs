use crate::doc::indexed::SearchKey;
use rustdoc_types::{Crate, Id, ItemEnum};

use super::{Doc, Parsed};

impl Doc<Parsed> {
    pub(super) fn search_keys_enums<'a>(
        krate: &'a Crate,
        id: &'a Id,
        base_path: &'a str,
    ) -> impl Iterator<Item = SearchKey> + 'a {
        krate
            .index
            .values()
            .filter_map(move |impl_item| match &impl_item.inner {
                ItemEnum::Impl(impl_block) => {
                    let rustdoc_types::Type::ResolvedPath(path) = &impl_block.for_ else {
                        return None;
                    };

                    if &path.id != id {
                        return None;
                    }

                    Some(Self::impl_method_keys(krate, impl_block, base_path))
                }
                _ => None,
            })
            .flatten()
    }
}
