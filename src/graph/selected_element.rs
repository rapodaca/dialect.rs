use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SelectedElement {
    B,
    C,
    N,
    O,
    P,
    S,
    Se,
    As,
}

impl fmt::Display for SelectedElement {
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
                Self::Se => "se",
                Self::As => "as",
            }
        )
    }
}
