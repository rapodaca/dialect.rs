use std::fmt;

use super::{Bracket, SelectedShortcut, Shortcut, Stereodescriptor};

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

impl AtomKind {
    /// Inverts configuration for Bracket variant given one or more virtual
    /// hydrogens.
    pub fn invert_configuration(&mut self) {
        if let AtomKind::Bracket(bracket) = self {
            if bracket.virtual_hydrogen.is_some() {
                bracket.stereodescriptor = match bracket.stereodescriptor {
                    Some(Stereodescriptor::Th1) => Some(Stereodescriptor::Th2),
                    Some(Stereodescriptor::Th2) => Some(Stereodescriptor::Th1),
                    None => None,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{Symbol, VirtualHydrogen};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn star() {
        let mut kind = AtomKind::Star;

        kind.invert_configuration();

        assert_eq!(kind, AtomKind::Star)
    }

    #[test]
    fn bracket_with_descriptor_without_hydrogen() {
        let mut kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Star,
            isotope: None,
            stereodescriptor: Some(Stereodescriptor::Th1),
            virtual_hydrogen: None,
            charge: None,
            extension: None,
        });

        kind.invert_configuration();

        assert_eq!(
            kind,
            AtomKind::Bracket(Bracket {
                symbol: Symbol::Star,
                isotope: None,
                stereodescriptor: Some(Stereodescriptor::Th1),
                virtual_hydrogen: None,
                charge: None,
                extension: None,
            })
        )
    }

    #[test]
    fn bracket_with_descriptor_and_hydrogen() {
        let mut kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Star,
            isotope: None,
            stereodescriptor: Some(Stereodescriptor::Th1),
            virtual_hydrogen: Some(VirtualHydrogen::H1),
            charge: None,
            extension: None,
        });

        kind.invert_configuration();

        assert_eq!(
            kind,
            AtomKind::Bracket(Bracket {
                symbol: Symbol::Star,
                isotope: None,
                stereodescriptor: Some(Stereodescriptor::Th2),
                virtual_hydrogen: Some(VirtualHydrogen::H1),
                charge: None,
                extension: None,
            })
        )
    }
}
