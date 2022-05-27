use std::{convert, default, fmt};

use crate::tree::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum VirtualHydrogen {
    H,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
}

impl convert::TryFrom<u8> for VirtualHydrogen {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::H1),
            2 => Ok(Self::H2),
            3 => Ok(Self::H3),
            4 => Ok(Self::H4),
            5 => Ok(Self::H5),
            6 => Ok(Self::H6),
            7 => Ok(Self::H7),
            8 => Ok(Self::H8),
            9 => Ok(Self::H9),
            _ => Err(Error::Range),
        }
    }
}

impl default::Default for VirtualHydrogen {
    fn default() -> Self {
        VirtualHydrogen::H1
    }
}

impl fmt::Display for VirtualHydrogen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::H => "H",
                Self::H1 => "H1",
                Self::H2 => "H2",
                Self::H3 => "H3",
                Self::H4 => "H4",
                Self::H5 => "H5",
                Self::H6 => "H6",
                Self::H7 => "H7",
                Self::H8 => "H8",
                Self::H9 => "H9",
            }
        )
    }
}
