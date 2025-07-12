mod enums;
mod index;
mod item;
mod structs;

use super::Doc;
pub use item::Item;
use rustdoc_types::Crate;

pub struct Parsed {
    pub ast: Crate,
}

impl Doc<Parsed> {
    pub(super) fn new(ast: Crate) -> Self {
        Self(Parsed { ast })
    }
}
