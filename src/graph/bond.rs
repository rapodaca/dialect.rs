use super::BondKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Bond {
    pub kind: BondKind,
    pub tid: u32,
}

impl Bond {
    pub fn new(kind: BondKind, tid: u32) -> Self {
        Self { kind, tid }
    }

    pub fn is_directional(&self) -> bool {
        self.kind == BondKind::Up || self.kind == BondKind::Down
    }
}
