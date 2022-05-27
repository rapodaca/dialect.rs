use crate::feature::{BondKind, Cut};

use super::{Atom, Target};

#[derive(Debug, PartialEq, Clone)]
pub enum Child {
    Union(BondKind, Target),
    Split(Atom),
}

impl Child {
    pub fn elided_star(children: Vec<Child>) -> Self {
        Self::Union(BondKind::Elided, Target::Atom(Atom::star(children)))
    }

    pub fn split_star(children: Vec<Child>) -> Self {
        Self::Split(Atom::star(children))
    }

    pub fn elided_cut(cut: Cut) -> Self {
        Self::Union(BondKind::Elided, Target::Cut(cut))
    }
}
