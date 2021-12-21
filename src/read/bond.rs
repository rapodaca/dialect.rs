use lyn::Scanner;

use crate::graph::BondKind;

pub fn bond(scanner: &mut Scanner) -> Option<BondKind> {
    scanner.transform(|target| match target {
        '-' => Some(BondKind::Single),
        '=' => Some(BondKind::Double),
        '#' => Some(BondKind::Triple),
        '$' => Some(BondKind::Quadruple),
        '/' => Some(BondKind::Up),
        '\\' => Some(BondKind::Down),
        _ => None,
    })
}
