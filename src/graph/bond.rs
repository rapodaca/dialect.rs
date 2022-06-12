use crate::feature::BondKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Bond {
    kind: BondKind,
    tid: usize,
}
