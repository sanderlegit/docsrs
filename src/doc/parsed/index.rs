use super::{Doc, Parsed};
use crate::{Indexed, doc::indexed::SearchKey};
use rustdoc_types::{Crate, Id, Impl, Item};
use std::collections::HashMap;

impl Doc<Parsed> {
    fn generate_searchkeys(&self, id: &Id, item: &Item) -> Option<Vec<SearchKey>> {
        let krate = &self.0.ast;
        let mut search_keys = Vec::new();

        let base_path = krate.paths.get(id).map(|p| p.path.join("::"))?;
        search_keys.push(SearchKey {
            id: id.0,
            key: base_path.clone(),
        });

        use rustdoc_types::ItemEnum;

        match &item.inner {
            ItemEnum::Struct(strukt) => {
                search_keys.extend(Self::search_keys_structs(krate, strukt, &base_path));
            }

            ItemEnum::Enum(_) => {
                search_keys.extend(Self::search_keys_enums(krate, id, &base_path));
            }

            _ => {}
        }

        Some(search_keys)
    }

    pub(super) fn impl_method_keys<'a>(
        krate: &'a Crate,
        impl_block: &'a Impl,
        base_path: &'a str,
    ) -> impl Iterator<Item = SearchKey> + 'a {
        impl_block.items.iter().filter_map(move |method_id| {
            let method_item = krate.index.get(method_id)?;
            let name = method_item.name.as_deref()?;
            Some(SearchKey {
                id: method_id.0,
                key: format!("{base_path}::{name}"),
            })
        })
    }

    pub fn build_search_index(&self) -> Doc<Indexed> {
        let krate = &self.0.ast;
        let index = krate
            .index
            .iter()
            .filter_map(|(id, item)| self.generate_searchkeys(id, item))
            .flat_map(|vec| vec.into_iter())
            .collect();

        let items = HashMap::new();

        Doc::<Indexed>::new(index, items)
    }
}
