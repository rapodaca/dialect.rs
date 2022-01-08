#[derive(Debug, PartialEq)]
pub enum Error {
    DuplicateBond(usize, usize),
    HalfBond(usize, usize),
    IncompatibleBond(usize, usize),
    Loop(usize),
    UnknownTarget(usize, usize),
}
