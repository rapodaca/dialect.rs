use super::BondKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Bond {
    pub kind: BondKind,
    pub tid: usize
}

impl Bond {
    pub fn new(kind: BondKind, tid: usize) -> Self {
        Self {
            kind,
            tid
        }
    }

    pub fn is_directional(&self) -> bool {
        self.kind == BondKind::Up || self.kind == BondKind::Down
    }
}