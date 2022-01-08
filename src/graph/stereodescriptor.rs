use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stereodescriptor {
    Th1,
    Th2,
}

impl fmt::Display for Stereodescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Th1 => "@".fmt(f),
            Self::Th2 => "@@".fmt(f),
        }
    }
}
