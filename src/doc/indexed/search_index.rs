use super::Doc;
use super::Indexed;
use crate::Error;
use rustdoc_types::{Id, Item};
use std::{io::Write, path::Path};

impl Doc<Indexed> {
    fn generate_searchkeys(&self, id: &Id, item: &Item) -> Option<Vec<(Id, String)>> {
        let krate = &self.0.ast;
        let mut search_keys = Vec::new();

        let base_path = krate.paths.get(id).map(|p| p.path.join("::"))?;
        search_keys.push((*id, base_path.clone()));

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
                                    Some((*method_id, path))
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
                                                Some((*method_id, path))
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

    pub fn build_search_index(&mut self) {
        let krate = &self.0.ast;
        let index = krate
            .index
            .iter()
            .filter_map(|(id, item)| self.generate_searchkeys(id, item))
            .flat_map(|vec| vec.into_iter())
            .collect();

        self.0.search_index = Some(index)
    }

    pub fn save_index<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        for item in self.0.search_index.as_ref().unwrap() {
            writeln!(file, "{}", item.1)?;
        }

        Ok(())
    }
}
