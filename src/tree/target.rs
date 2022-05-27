use crate::feature::Cut;

use super::Atom;

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Atom(Atom),
    Cut(Cut),
}
