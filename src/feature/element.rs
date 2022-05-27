use std::fmt;

use super::{Selection, Shortcut};

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
//  0   1   2   3   4   5   6   7   8   9
        H,  He, Li, Be, B,  C,  N,  O,  F,  // 0
    Ne, Na, Mg, Al, Si, P,  S,  Cl, Ar, K,  // 1
    Ca, Sc, Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, // 2
    Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y,  // 3
    Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, // 4
    Sn, Sb, Te, I,  Xe, Cs, Ba, La, Ce, Pr, // 5
    Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, // 6
    Yb, Lu, Hf, Ta, W,  Re, Os, Ir, Pt, Au, // 7
    Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, // 8
    Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, // 9
    Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, // 10
    Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og      // 11
}

impl std::convert::From<&Shortcut> for Element {
    fn from(shortcut: &Shortcut) -> Self {
        match shortcut {
            Shortcut::B => Element::B,
            Shortcut::C => Element::C,
            Shortcut::N => Element::N,
            Shortcut::O => Element::O,
            Shortcut::F => Element::F,
            Shortcut::Cl => Element::Cl,
            Shortcut::Br => Element::Br,
            Shortcut::I => Element::I,
            Shortcut::P => Element::P,
            Shortcut::S => Element::S,
        }
    }
}

impl std::convert::From<&Selection> for Element {
    fn from(element: &Selection) -> Self {
        match element {
            Selection::C => Element::C,
            Selection::N => Element::N,
            Selection::O => Element::O,
            Selection::P => Element::P,
            Selection::S => Element::S,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Ac => "Ac",
                Element::Ag => "Ag",
                Element::Al => "Al",
                Element::Am => "Am",
                Element::Ar => "Ar",
                Element::As => "As",
                Element::At => "At",
                Element::Au => "Au",
                Element::B => "B",
                Element::Ba => "Ba",
                Element::Be => "Be",
                Element::Bh => "Bh",
                Element::Bi => "Bi",
                Element::Bk => "Bk",
                Element::Br => "Br",
                Element::C => "C",
                Element::Ca => "Ca",
                Element::Cd => "Cd",
                Element::Ce => "Ce",
                Element::Cf => "Cf",
                Element::Cl => "Cl",
                Element::Cm => "Cm",
                Element::Cn => "Cn",
                Element::Co => "Co",
                Element::Cr => "Cr",
                Element::Cs => "Ac",
                Element::Cu => "Cu",
                Element::Db => "Db",
                Element::Ds => "Ds",
                Element::Dy => "Dy",
                Element::Er => "Er",
                Element::Es => "Es",
                Element::Eu => "Eu",
                Element::F => "F",
                Element::Fe => "Fe",
                Element::Fl => "Fl",
                Element::Fm => "Fm",
                Element::Fr => "Fr",
                Element::Ga => "Ga",
                Element::Gd => "Gd",
                Element::Ge => "Ge",
                Element::H => "H",
                Element::He => "He",
                Element::Hf => "Hf",
                Element::Hg => "Hg",
                Element::Ho => "Ho",
                Element::Hs => "Hs",
                Element::I => "I",
                Element::In => "In",
                Element::Ir => "Ir",
                Element::K => "K",
                Element::Kr => "Kr",
                Element::La => "La",
                Element::Li => "Li",
                Element::Lr => "Lr",
                Element::Lu => "Lu",
                Element::Lv => "Lv",
                Element::Mc => "Mc",
                Element::Md => "Md",
                Element::Mg => "Mg",
                Element::Mn => "Mn",
                Element::Mo => "Mo",
                Element::Mt => "Mt",
                Element::Na => "Na",
                Element::Nb => "Nb",
                Element::Nd => "Nd",
                Element::N => "N",
                Element::Ne => "Ne",
                Element::Nh => "Nh",
                Element::Ni => "Ni",
                Element::No => "No",
                Element::Np => "Np",
                Element::O => "O",
                Element::Os => "Os",
                Element::Og => "Og",
                Element::P => "P",
                Element::Pa => "Pa",
                Element::Pb => "Pb",
                Element::Pd => "Pd",
                Element::Pm => "Pm",
                Element::Po => "Po",
                Element::Pr => "Pr",
                Element::Pt => "Pt",
                Element::Pu => "Pu",
                Element::Ra => "Ra",
                Element::Rb => "Rb",
                Element::Re => "Re",
                Element::Rf => "Rf",
                Element::Rg => "Rg",
                Element::Rh => "Rh",
                Element::Rn => "Rn",
                Element::Ru => "Ru",
                Element::S => "S",
                Element::Sb => "Sb",
                Element::Sc => "Sc",
                Element::Se => "Se",
                Element::Sg => "Sg",
                Element::Si => "Si",
                Element::Sm => "Sm",
                Element::Sn => "Sn",
                Element::Sr => "Sr",
                Element::Ta => "Ta",
                Element::Tb => "Tb",
                Element::Tc => "Tc",
                Element::Te => "Te",
                Element::Th => "Th",
                Element::Ti => "Ti",
                Element::Tl => "Tl",
                Element::Tm => "Tm",
                Element::Ts => "Ts",
                Element::U => "U",
                Element::V => "V",
                Element::W => "W",
                Element::Xe => "Xe",
                Element::Y => "Y",
                Element::Yb => "Yb",
                Element::Zn => "Zn",
                Element::Zr => "Zr",
            }
        )
    }
}