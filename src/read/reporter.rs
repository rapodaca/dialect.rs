use crate::graph::{AtomKind, BondKind, Cut};

use super::Follower;

pub struct Reporter<'a, F: Follower> {
    follower: &'a mut F,
    chain: usize,
}

impl<'a, F: Follower> Reporter<'a, F> {
    pub fn new(follower: &'a mut F) -> Self {
        Self { follower, chain: 0 }
    }

    pub fn chain_length(&self) -> usize {
        self.chain
    }
}

impl<'a, F: Follower> Follower for Reporter<'a, F> {
    fn root(&mut self, root: &AtomKind) {
        self.chain += 1;

        self.follower.root(root)
    }

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind) {
        self.chain += 1;

        self.follower.extend(bond_kind, atom_kind)
    }

    fn join(&mut self, bond_kind: &BondKind, cut: &Cut) {
        self.chain += 1;

        self.follower.join(bond_kind, cut)
    }

    fn pop(&mut self, depth: usize) {
        self.chain -= depth;

        self.follower.pop(depth)
    }
}
