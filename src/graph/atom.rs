use crate::feature::AtomKind;

use super::Bond;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Atom {
    pub kind: AtomKind,
    pub bonds: Vec<Bond>,
}
