use lyn::{Action, Scanner};

use super::{digit, element, hex, missing_character, non_zero, Error};
use crate::graph::{
    Bracket, Charge, Extension, Isotope, SelectedElement, Stereodescriptor,
    Symbol, VirtualHydrogen,
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
        extension: extension(scanner),
    }));

    if scanner.take(&']') {
        result
    } else {
        Err(missing_character(scanner))
    }
}

fn isotope(scanner: &mut Scanner) -> Option<Isotope> {
    let mass_number = match non_zero(scanner) {
        Some(first) => match digit(scanner) {
            Some(second) => match digit(scanner) {
                Some(third) => {
                    100 * first as u16 + 10 * second as u16 + third as u16
                }
                None => second as u16 + 10 * first as u16,
            },
            None => first as u16,
        },
        None => return None,
    };

    Some(Isotope::try_from(mass_number).expect("isotope"))
}

fn symbol(scanner: &mut Scanner) -> Result<Option<Symbol>, Error> {
    if let Some(element) = element(scanner)? {
        Ok(Some(Symbol::Element(element)))
    } else if let Some(selected_element) = selected_element(scanner)? {
        Ok(Some(Symbol::SelectedElement(selected_element)))
    } else if star(scanner) {
        Ok(Some(Symbol::Star))
    } else {
        Ok(None)
    }
}

fn selected_element(
    scanner: &mut Scanner,
) -> Result<Option<SelectedElement>, Error> {
    Ok(scanner.scan(|symbol| match symbol {
        "c" => Some(Action::Return(SelectedElement::C)),
        _ => None,
    })?)
}

fn star(scanner: &mut Scanner) -> bool {
    scanner.take(&'*')
}

fn stereodescriptor(scanner: &mut Scanner) -> Option<Stereodescriptor> {
    if scanner.take(&'@') {
        if scanner.take(&'@') {
            Some(Stereodescriptor::Th2)
        } else {
            Some(Stereodescriptor::Th1)
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

fn extension(scanner: &mut Scanner) -> Option<Extension> {
    if !scanner.take(&':') {
        return None;
    }

    match hex(scanner) {
        Some(first) => match hex(scanner) {
            Some(second) => match hex(scanner) {
                Some(third) => match hex(scanner) {
                    Some(fourth) => Some(Extension::from(
                        first as u16 * 4096
                            + second as u16 * 256
                            + third as u16 * 16
                            + fourth as u16,
                    )),
                    None => Some(Extension::from(
                        first as u16 * 256 + second as u16 * 16 + third as u16,
                    )),
                },
                None => {
                    Some(Extension::from(first as u16 * 16 + second as u16))
                }
            },
            None => Some(Extension::from(first as u16)),
        },
        None => None,
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
    fn extension_overflow() {
        let mut scanner = Scanner::new("[C:ffff0]");

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
                symbol: Symbol::SelectedElement(SelectedElement::C),
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
                stereodescriptor: Some(Stereodescriptor::Th1),
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
    fn element_with_extension() {
        let mut scanner = Scanner::new("[C:ffff]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                extension: Some(0xffff.into()),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn kitchen_sink() {
        let mut scanner = Scanner::new("[12C@H1+2:abcd]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                isotope: Some(Isotope::try_from(12).unwrap()),
                stereodescriptor: Some(Stereodescriptor::Th1),
                virtual_hydrogen: Some(VirtualHydrogen::H1),
                charge: Some(Charge::Plus2),
                extension: Some(0xabcd.into())
            }))
        )
    }
}
