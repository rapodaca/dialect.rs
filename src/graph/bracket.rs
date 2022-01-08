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
            option_to_string(&self.extension)
        )
    }
}

fn option_to_string<T: fmt::Display>(option: &Option<T>) -> String {
    match option {
        Some(option) => option.to_string(),
        None => "".to_string()
    }
}
