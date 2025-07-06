use super::Doc;
use crate::Error;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use rustdoc_types::{Crate, Id, Item};
use std::{cmp::Reverse, io::Write};

pub struct Indexed {
    pub ast: Crate,
    search_index: Option<Vec<(Id, String)>>,
}

impl Doc<Indexed> {
    pub(super) fn new(ast: Crate) -> Self {
        Self(Indexed {
            ast,
            search_index: None,
        })
    }

    pub fn save_index(&self) -> Result<(), Error> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("index")?;

        for item in self.0.search_index.as_ref().unwrap() {
            writeln!(file, "{}", item.1)?;
        }

        Ok(())
    }

    fn generate_searchkeys(&self, id: &Id, item: &Item) -> Option<Vec<(Id, String)>> {
        let krate = &self.0.ast;
        let mut search_keys = Vec::new();

        let base_path = krate.paths.get(id).map(|p| p.path.join("::"))?;
        search_keys.push((*id, base_path.clone()));

        use rustdoc_types::ItemEnum;

        if let ItemEnum::Struct(strukt) = &item.inner {
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

        println!("{search_keys:?}");

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

        println!("{index:?}");

        self.0.search_index = Some(index)
    }

    pub fn search(&self, query: &str, n: impl Into<Option<usize>>) -> Option<Vec<&Item>> {
        let index = match &self.0.search_index {
            Some(idx) => idx,
            None => return None,
        };

        let matcher = SkimMatcherV2::default();
        let mut results: Vec<(Id, i64)> = index
            .iter()
            .filter_map(|(id, searchable)| {
                matcher
                    .fuzzy_match(&searchable.to_lowercase(), &query.to_lowercase())
                    .map(|score| (*id, score))
            })
            .collect();

        results.sort_unstable_by_key(|&(_, score)| Reverse(score));

        let mut matches = results;
        if let Some(n) = n.into() {
            if n == 0 {
                return None;
            }
            matches.truncate(n);
        }

        Some(
            matches
                .iter()
                .filter_map(|(id, _)| self.0.ast.index.get(id))
                .collect(),
        )
    }
}
