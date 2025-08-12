use super::{Doc, Parsed};
use crate::{Indexed, doc::indexed::SearchKey};
use rustdoc_types::{Crate, Id, Impl, ItemEnum, ItemKind, ItemSummary};

impl Doc<Parsed> {
    /// Builds a fuzzy searchable index from the parsed documentation
    ///
    /// This method processes the parsed documentation AST and creates search keys
    /// for all items (structs, enums, functions, methods, etc.) including their
    /// fully qualified paths. The resulting index enables fast fuzzy searching
    /// across all documentation items.
    ///
    /// # Returns
    ///
    /// A [`Doc<Indexed>`] that supports fuzzy search operations.
    pub fn build_search_index(&self) -> Doc<Indexed> {
        let krate = &self.0.ast;
        let index: Vec<SearchKey> = krate
            .paths
            .iter()
            .filter_map(|(id, item)| self.generate_searchkeys(id, item))
            .flat_map(|vec| vec.into_iter())
            .collect();

        let items = self.build_items(&index, krate.crate_version.clone());

        <Doc<Indexed>>::new(index, items)
    }

    fn generate_searchkeys(&self, id: &Id, item_summary: &ItemSummary) -> Option<Vec<SearchKey>> {
        if item_summary.crate_id != 0 {
            return None;
        };

        let krate = &self.0.ast;

        let base_path = item_summary.path.join("::");
        let kind = item_summary.kind;

        let mut search_keys = vec![SearchKey {
            id: id.0,
            key: base_path.clone(),
        }];

        match kind {
            ItemKind::Struct => {
                if let Some(item) = krate.index.get(id) {
                    if let ItemEnum::Struct(s) = &item.inner {
                        search_keys.extend(Self::search_keys_structs(krate, s, &base_path));
                    }
                }
            }
            ItemKind::Enum => {
                if let Some(item) = krate.index.get(id) {
                    if let ItemEnum::Enum(e) = &item.inner {
                        search_keys.extend(Self::search_keys_enums(krate, e, &base_path));
                    }
                }
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
}
