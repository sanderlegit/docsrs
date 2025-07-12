use super::{Doc, Parsed};
use crate::doc::indexed::SearchKey;
use std::collections::HashMap;

impl Doc<Parsed> {
    pub(super) fn build_items(&self, index: &[SearchKey]) -> HashMap<u32, Item> {
        index
            .iter()
            .map(|key| {
                let path = key.key.split("::").map(|s| s.to_owned()).collect();

                let item = self.0.ast.index.get(&rustdoc_types::Id(key.id)).unwrap();

                let links = item
                    .links
                    .clone()
                    .into_iter()
                    .map(|(k, id)| (k, (id.0)))
                    .collect();

                (
                    key.id,
                    Item {
                        id: key.id,
                        crate_id: item.crate_id,
                        path,
                        visibility: item.visibility.clone(),
                        span: item.span.clone(),
                        name: item.name.clone().unwrap_or_default(),
                        docs: item.docs.clone(),
                        links,
                        attributes: item.attrs.clone(),
                        deprecation: item.deprecation.clone(),
                        inner: item.inner.clone(),
                    },
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: u32,
    pub crate_id: u32,
    pub path: Vec<String>,
    pub visibility: rustdoc_types::Visibility,
    pub span: Option<rustdoc_types::Span>,
    pub name: String,
    pub docs: Option<String>,
    pub links: HashMap<String, u32>,
    pub attributes: Vec<String>,
    pub deprecation: Option<rustdoc_types::Deprecation>,
    pub inner: rustdoc_types::ItemEnum,
}
