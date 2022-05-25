use lyn::{Action, Scanner};

use super::{element, missing_character, non_zero, uint16, Error};
use crate::graph::{
    Bracket, Charge, Extension, Isotope, Selection, Stereodescriptor, Symbol,
    VirtualHydrogen,
};

pub fn bracket(scanner: &mut Scanner) -> Result<Option<Bracket>, Error> {
    if !scanner.take(&'[') {
        return Ok(None);
    }

    let result = Ok(Some(Bracket {
        isotope: isotope(scanner),
        symbol: match symbol(scanner)? {
            Some(symbol) => symbol,
            None => return Err(missing_character(scanner)),
        },
        stereodescriptor: stereodescriptor(scanner),
        virtual_hydrogen: virtual_hydrogen(scanner),
        charge: charge(scanner),
        extension: extension(scanner)?,
    }));

    if scanner.take(&']') {
        result
    } else {
        Err(missing_character(scanner))
    }
}

fn isotope(scanner: &mut Scanner) -> Option<Isotope> {
    match uint16(scanner, 3) {
        Some(number) => Some(number.try_into().expect("isotope")),
        None => None,
    }
}

fn symbol(scanner: &mut Scanner) -> Result<Option<Symbol>, Error> {
    if let Some(element) = element(scanner)? {
        Ok(Some(Symbol::Element(element)))
    } else if let Some(selection) = selected_element(scanner)? {
        Ok(Some(Symbol::Selection(selection)))
    } else if star(scanner) {
        Ok(Some(Symbol::Star))
    } else {
        Ok(None)
    }
}

fn selected_element(scanner: &mut Scanner) -> Result<Option<Selection>, Error> {
    Ok(scanner.scan(|symbol| match symbol {
        "c" => Some(Action::Return(Selection::C)),
        _ => None,
    })?)
}

fn star(scanner: &mut Scanner) -> bool {
    scanner.take(&'*')
}

fn stereodescriptor(scanner: &mut Scanner) -> Option<Stereodescriptor> {
    if scanner.take(&'@') {
        if scanner.take(&'@') {
            Some(Stereodescriptor::Right)
        } else {
            Some(Stereodescriptor::Left)
        }
    } else {
        None
    }
}

fn virtual_hydrogen(scanner: &mut Scanner) -> Option<VirtualHydrogen> {
    if scanner.take(&'H') {
        match non_zero(scanner) {
            Some(digit) => {
                Some(VirtualHydrogen::try_from(digit).expect("digit"))
            }
            _ => Some(VirtualHydrogen::default()),
        }
    } else {
        None
    }
}

fn charge(scanner: &mut Scanner) -> Option<Charge> {
    if scanner.take(&'+') {
        match non_zero(scanner) {
            Some(digit) => Some(Charge::try_from(digit as i8).expect("charge")),
            None => Some(Charge::Plus),
        }
    } else if scanner.take(&'-') {
        match non_zero(scanner) {
            Some(digit) => {
                Some(Charge::try_from(digit as i8 * -1).expect("charge"))
            }
            None => Some(Charge::Minus),
        }
    } else {
        None
    }
}

fn extension(scanner: &mut Scanner) -> Result<Option<Extension>, Error> {
    if !scanner.take(&':') {
        return Ok(None);
    }

    match uint16(scanner, 4) {
        Some(number) => {
            Ok(Some(number.try_into().expect("extension overflow")))
        }
        None => Err(missing_character(scanner)),
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Element;

    use super::*;

    #[test]
    fn no_open() {
        let mut scanner = Scanner::new("X");

        assert_eq!(bracket(&mut scanner), Ok(None))
    }

    #[test]
    fn open_no_close() {
        let mut scanner = Scanner::new("[");

        assert_eq!(bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn open_invalid() {
        let mut scanner = Scanner::new("[?");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn open_element_no_close() {
        let mut scanner = Scanner::new("[C");

        assert_eq!(bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn open_isotope_no_close() {
        let mut scanner = Scanner::new("[1");

        assert_eq!(bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn open_symbol_extension_no_close() {
        let mut scanner = Scanner::new("[C:");

        assert_eq!(bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn open_symbol_extension_close() {
        let mut scanner = Scanner::new("[C:]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(3)))
    }

    #[test]
    fn isotope_overflow() {
        let mut scanner = Scanner::new("[1234C]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(4)))
    }

    #[test]
    fn charge_overflow() {
        let mut scanner = Scanner::new("[C+10]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(4)))
    }

    #[test]
    fn extension_bad_character() {
        let mut scanner = Scanner::new("[C:a00]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(3)))
    }

    #[test]
    fn extension_overflow() {
        let mut scanner = Scanner::new("[C:99990]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(7)))
    }

    #[test]
    fn element() {
        let mut scanner = Scanner::new("[C]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn selected_element() {
        let mut scanner = Scanner::new("[c]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Selection(Selection::C),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn open_star_close() {
        let mut scanner = Scanner::new("[*]");

        assert_eq!(bracket(&mut scanner), Ok(Some(Bracket::default())))
    }

    #[test]
    fn element_with_isotope() {
        let mut scanner = Scanner::new("[1H]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::H),
                isotope: Some(Isotope::try_from(1).unwrap()),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_with_stereodescriptor() {
        let mut scanner = Scanner::new("[C@]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                stereodescriptor: Some(Stereodescriptor::Left),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_with_virtual_hydrogen() {
        let mut scanner = Scanner::new("[CH1]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                virtual_hydrogen: Some(VirtualHydrogen::H1),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_with_charge() {
        let mut scanner = Scanner::new("[C+1]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                charge: Some(Charge::Plus1),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_extension() {
        let mut scanner = Scanner::new("[C:9999]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                extension: Some(9999.try_into().unwrap()),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_extension_leading_zero() {
        let mut scanner = Scanner::new("[C:01]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                extension: Some(1.try_into().unwrap()),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn kitchen_sink() {
        let mut scanner = Scanner::new("[12C@H1+2:1234]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                isotope: Some(Isotope::try_from(12).unwrap()),
                stereodescriptor: Some(Stereodescriptor::Left),
                virtual_hydrogen: Some(VirtualHydrogen::H1),
                charge: Some(Charge::Plus2),
                extension: Some(1234.try_into().unwrap())
            }))
        )
    }
}
