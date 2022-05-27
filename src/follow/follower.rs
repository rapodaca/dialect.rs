use crate::feature::{AtomKind, BondKind, Cut};

pub trait Follower {
    fn root(&mut self, root: &AtomKind);

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind);

    fn join(&mut self, bond_kind: &BondKind, cut: &Cut);

    fn push(&mut self);

    fn pop(&mut self);
}
