use super::{Doc, Parsed};
use crate::{Indexed, doc::indexed::SearchKey};
use rustdoc_types::{Id, Item};
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
                search_keys.extend(
                    strukt
                        .impls
                        .iter()
                        .filter_map(|impl_id| {
                            let impl_item = krate.index.get(impl_id)?;

                            if let ItemEnum::Impl(impl_block) = &impl_item.inner {
                                Some(impl_block.items.iter().filter_map(|method_id| {
                                    let method_item = krate.index.get(method_id)?;
                                    let name = method_item.name.as_deref()?;
                                    let path = format!("{base_path}::{name}");
                                    Some(SearchKey {
                                        id: method_id.0,
                                        key: path,
                                    })
                                }))
                            } else {
                                None
                            }
                        })
                        .flatten(),
                );
            }

            ItemEnum::Enum(_) => {
                search_keys.extend(
                    krate
                        .index
                        .values()
                        .filter_map(|impl_item| {
                            if let ItemEnum::Impl(impl_block) = &impl_item.inner {
                                if let rustdoc_types::Type::ResolvedPath(path) = &impl_block.for_ {
                                    if &path.id == id {
                                        return Some(impl_block.items.iter().filter_map(
                                            |method_id| {
                                                let method_item = krate.index.get(method_id)?;
                                                let name = method_item.name.as_deref()?;
                                                let path = format!("{base_path}::{name}");
                                                Some(SearchKey {
                                                    id: method_id.0,
                                                    key: path,
                                                })
                                            },
                                        ));
                                    }
                                }
                            }
                            None
                        })
                        .flatten(),
                );
            }

            _ => {}
        }

        Some(search_keys)
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
