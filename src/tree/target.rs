use super::{Atom, Bridge};

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Atom(Atom),
    Bridge(Bridge),
}
