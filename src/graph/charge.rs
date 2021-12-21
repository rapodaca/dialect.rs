use std::convert;

use super::Error;

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
