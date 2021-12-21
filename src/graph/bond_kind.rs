use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum BondKind {
    Elided,
    Single,
    Double,
    Triple,
    Quadruple,
    Up,
    Down,
}

impl fmt::Display for BondKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Elided => write!(f, ""),
            Self::Single => write!(f, "-"),
            Self::Double => write!(f, "="),
            Self::Triple => write!(f, "#"),
            Self::Quadruple => write!(f, "$"),
            Self::Up => write!(f, "/"),
            Self::Down => write!(f, "\\"),
        }
    }
}
