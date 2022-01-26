use super::{AtomKind, Bond, Element};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Atom {
    pub bonds: Vec<Bond>,
    pub kind: AtomKind,
}

impl Atom {
    pub fn element(&self) -> Option<Element> {
        match &self.kind {
            AtomKind::Star => None,
            AtomKind::Shortcut(shortcut) => Some(shortcut.into()),
            AtomKind::SelectedShortcut(selected) => Some(selected.into()),
            AtomKind::Bracket(bracket) => match &bracket.symbol {
                super::Symbol::Star => None,
                super::Symbol::Element(element) => Some(element.clone()),
                super::Symbol::SelectedElement(selected) => {
                    Some(selected.into())
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{
        Bracket, SelectedElement, SelectedShortcut, Shortcut, Symbol,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn star() {
        let atom = Atom {
            kind: AtomKind::Star,
            ..Default::default()
        };

        assert_eq!(atom.element(), None)
    }

    #[test]
    fn shortcut() {
        let atom = Atom {
            kind: AtomKind::Shortcut(Shortcut::C),
            ..Default::default()
        };

        assert_eq!(atom.element(), Some(Element::C))
    }

    #[test]
    fn selected_shortcut() {
        let atom = Atom {
            kind: AtomKind::SelectedShortcut(SelectedShortcut::C),
            ..Default::default()
        };

        assert_eq!(atom.element(), Some(Element::C))
    }

    #[test]
    fn bracket_star() {
        let atom = Atom {
            kind: AtomKind::Bracket(Bracket {
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(atom.element(), None)
    }

    #[test]
    fn bracket_element() {
        let atom = Atom {
            kind: AtomKind::Bracket(Bracket {
                symbol: Symbol::Element(Element::C),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(atom.element(), Some(Element::C))
    }

    #[test]
    fn bracket_selected_element() {
        let atom = Atom {
            kind: AtomKind::Bracket(Bracket {
                symbol: Symbol::SelectedElement(SelectedElement::C),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(atom.element(), Some(Element::C))
    }
}
