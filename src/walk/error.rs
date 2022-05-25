#[derive(Debug, PartialEq)]
pub enum Error {
    DuplicateBond(u32, u32),
    HalfBond(u32, u32),
    IncompatibleBond(u32, u32),
    Loop(u32),
    UnknownTarget(u32, u32),
    TooManyAtoms,
}
