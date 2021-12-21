use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SelectedShortcut {
    B,
    C,
    N,
    O,
    P,
    S,
}

impl fmt::Display for SelectedShortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::B => "b",
                Self::C => "c",
                Self::N => "n",
                Self::O => "o",
                Self::P => "p",
                Self::S => "s",
            }
        )
    }
}
