use lyn::Scanner;

use crate::feature::Cut;

use super::{digit, missing_character, Error};

pub fn cut(scanner: &mut Scanner) -> Result<Option<Cut>, Error> {
    if scanner.take(&'%') {
        if let Some(first) = digit(scanner) {
            if let Some(second) = digit(scanner) {
                let index = first * 10 + second;

                Ok(Some(Cut::new(index).expect("cut index")))
            } else {
                Err(missing_character(scanner))
            }
        } else {
            Err(missing_character(scanner))
        }
    } else if let Some(digit) = digit(scanner) {
        Ok(Some(Cut::new(digit).expect("cut index")))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_invalid() {
        let mut scanner = Scanner::new("%x");

        assert_eq!(cut(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn percent_digit_invalid() {
        let mut scanner = Scanner::new("%1x");

        assert_eq!(cut(&mut scanner), Err(Error::Character(2)))
    }

    #[test]
    fn none() {
        let mut scanner = Scanner::new("x");

        assert_eq!(cut(&mut scanner), Ok(None))
    }

    #[test]
    fn percent_digit_digit() {
        let mut scanner = Scanner::new("%42");

        assert_eq!(cut(&mut scanner), Ok(Some(Cut::C42)))
    }

    #[test]
    fn digit() {
        let mut scanner = Scanner::new("7");

        assert_eq!(cut(&mut scanner), Ok(Some(Cut::C7)))
    }
}
