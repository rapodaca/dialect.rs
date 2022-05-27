use core::fmt;
use std::convert;

use crate::tree::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Charge {
    Minus9,
    Minus8,
    Minus7,
    Minus6,
    Minus5,
    Minus4,
    Minus3,
    Minus2,
    Minus1,
    Minus,
    Plus,
    Plus1,
    Plus2,
    Plus3,
    Plus4,
    Plus5,
    Plus6,
    Plus7,
    Plus8,
    Plus9,
}

impl convert::TryFrom<i8> for Charge {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -9 => Ok(Charge::Minus9),
            -8 => Ok(Charge::Minus8),
            -7 => Ok(Charge::Minus7),
            -6 => Ok(Charge::Minus6),
            -5 => Ok(Charge::Minus5),
            -4 => Ok(Charge::Minus4),
            -3 => Ok(Charge::Minus3),
            -2 => Ok(Charge::Minus2),
            -1 => Ok(Charge::Minus1),
            1 => Ok(Charge::Plus1),
            2 => Ok(Charge::Plus2),
            3 => Ok(Charge::Plus3),
            4 => Ok(Charge::Plus4),
            5 => Ok(Charge::Plus5),
            6 => Ok(Charge::Plus6),
            7 => Ok(Charge::Plus7),
            8 => Ok(Charge::Plus8),
            9 => Ok(Charge::Plus9),
            _ => Err(Error::Range),
        }
    }
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Charge::Minus9 => "-9".fmt(f),
            Charge::Minus8 => "-8".fmt(f),
            Charge::Minus7 => "-7".fmt(f),
            Charge::Minus6 => "-6".fmt(f),
            Charge::Minus5 => "-5".fmt(f),
            Charge::Minus4 => "-4".fmt(f),
            Charge::Minus3 => "-3".fmt(f),
            Charge::Minus2 => "-2".fmt(f),
            Charge::Minus1 => "-1".fmt(f),
            Charge::Minus => "-".fmt(f),
            Charge::Plus => "+".fmt(f),
            Charge::Plus1 => "+1".fmt(f),
            Charge::Plus2 => "+2".fmt(f),
            Charge::Plus3 => "+3".fmt(f),
            Charge::Plus4 => "+4".fmt(f),
            Charge::Plus5 => "+5".fmt(f),
            Charge::Plus6 => "+6".fmt(f),
            Charge::Plus7 => "+7".fmt(f),
            Charge::Plus8 => "+8".fmt(f),
            Charge::Plus9 => "+9".fmt(f),
        }
    }
}
