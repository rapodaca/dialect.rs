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
            "[{}{}{}]",
            match &self.isotope {
                Some(isotope) => isotope.to_string(),
                None => "".to_string(),
            },
            self.symbol.to_string(),
            match &self.virtual_hydrogen {
                Some(virtual_hydrogen) => virtual_hydrogen.to_string(),
                None => "".to_string(),
            }
        )
    }
}
