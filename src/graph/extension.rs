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
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn nine() {
        let extension = Extension::from(9);

        assert_eq!(extension.to_string(), "9")
    }
}
