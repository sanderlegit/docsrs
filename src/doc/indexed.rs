use super::Doc;
use rustdoc_types::Crate;

pub struct Indexed {
    pub ast: Crate,
}

impl Doc<Indexed> {
    pub(super) fn new(ast: Crate) -> Self {
        Self(Indexed { ast })
    }
}
