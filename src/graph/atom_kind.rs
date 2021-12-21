use std::fmt;

use super::{Bracket, SelectedShortcut, Shortcut};

#[derive(Debug, PartialEq, Clone)]
pub enum AtomKind {
    Star,
    Shortcut(Shortcut),
    SelectedShortcut(SelectedShortcut),
    Bracket(Bracket),
}

impl fmt::Display for AtomKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Star => write!(f, "*"),
            Self::Shortcut(shortcut) => shortcut.fmt(f),
            Self::SelectedShortcut(selected) => selected.fmt(f),
            Self::Bracket(bracket) => bracket.fmt(f),
        }
    }
}
