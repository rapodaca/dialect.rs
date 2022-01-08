use std::{convert, fmt};

#[derive(Debug, PartialEq, Clone)]
pub struct Extension(u16);

impl convert::From<u16> for Extension {
    fn from(value: u16) -> Self {
        Extension(value)
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
