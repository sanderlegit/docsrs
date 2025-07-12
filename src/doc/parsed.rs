mod enums;
mod index;
mod structs;

use super::Doc;
use rustdoc_types::Crate;

pub struct Parsed {
    pub ast: Crate,
}

impl Doc<Parsed> {
    pub(super) fn new(ast: Crate) -> Self {
        Self(Parsed { ast })
    }
}
