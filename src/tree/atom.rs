use crate::feature::AtomKind;

use super::Child;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Atom {
    pub kind: AtomKind,
    pub children: Vec<Child>,
}

impl Atom {
    pub fn star(children: Vec<Child>) -> Self {
        Self {
            kind: AtomKind::Star,
            children,
        }
    }

    pub fn degree(&self) -> usize {
        self.children.len()
    }
}
