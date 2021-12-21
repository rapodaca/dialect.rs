use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Shortcut {
    B,
    C,
    N,
    O,
    F,
    Cl,
    Br,
    I,
    At,
    Ts,
    P,
    S,
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::B => "B",
                Self::C => "C",
                Self::N => "N",
                Self::O => "O",
                Self::F => "F",
                Self::Cl => "Cl",
                Self::Br => "Br",
                Self::I => "I",
                Self::At => "At",
                Self::Ts => "Ts",
                Self::P => "P",
                Self::S => "S",
            }
        )
    }
}
