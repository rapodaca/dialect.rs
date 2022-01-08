use super::{AtomKind, Element, Bond};

#[derive(Debug, PartialEq, Clone)]
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
