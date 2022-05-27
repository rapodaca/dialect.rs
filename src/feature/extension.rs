use std::{convert, fmt};

use crate::tree::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Extension(u16);

impl convert::TryFrom<u16> for Extension {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 9999 {
            Err(Self::Error::Range)
        } else {
            Ok(Self(value))
        }
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn nine() {
        let extension = Extension::try_from(9).unwrap();

        assert_eq!(extension.to_string(), "9")
    }

    #[test]
    fn ten() {
        let extension = Extension::try_from(10).unwrap();

        assert_eq!(extension.to_string(), "10")
    }
}
