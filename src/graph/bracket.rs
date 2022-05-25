use std::fmt;

use super::{
    Charge, Extension, Isotope, Stereodescriptor, Symbol, VirtualHydrogen,
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Bracket {
    pub symbol: Symbol,
    pub isotope: Option<Isotope>,
    pub stereodescriptor: Option<Stereodescriptor>,
    pub virtual_hydrogen: Option<VirtualHydrogen>,
    pub charge: Option<Charge>,
    pub extension: Option<Extension>,
}

impl fmt::Display for Bracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}{}{}{}{}{}]",
            option_to_string(&self.isotope),
            self.symbol.to_string(),
            option_to_string(&self.stereodescriptor),
            option_to_string(&self.virtual_hydrogen),
            option_to_string(&self.charge),
            match &self.extension {
                Some(extension) => format!(":{}", extension),
                None => "".to_string(),
            }
        )
    }
}

fn option_to_string<T: fmt::Display>(option: &Option<T>) -> String {
    match option {
        Some(option) => option.to_string(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod to_string {
    use crate::graph::Element;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn kitchen_sink() {
        let bracket = Bracket {
            symbol: Symbol::Element(Element::C),
            isotope: Some(Isotope::try_from(13).unwrap()),
            stereodescriptor: Some(Stereodescriptor::Left),
            virtual_hydrogen: Some(VirtualHydrogen::H),
            charge: Some(Charge::Plus),
            extension: Some(Extension::try_from(10).unwrap()),
        };

        assert_eq!(bracket.to_string(), "[13C@H+:10]")
    }
}
