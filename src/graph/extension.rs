use std::convert;

#[derive(Debug, PartialEq, Clone)]
pub struct Extension(u16);

impl convert::From<u16> for Extension {
    fn from(value: u16) -> Self {
        Extension(value)
    }
}
