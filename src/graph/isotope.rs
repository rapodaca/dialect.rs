use std::{convert, fmt};

use super::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Isotope(u16);

impl convert::TryFrom<u16> for Isotope {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 1000 {
            Ok(Isotope(value))
        } else {
            Err(Error::Range)
        }
    }
}

impl fmt::Display for Isotope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
