use std::{fmt::Display, ops::Add};

use super::molecule::Molecule;
pub mod elements;

pub type Symbol = &'static str;
pub type AtomicNumber = u8;

pub struct Element {
    symbol: Symbol,
    atomic_number: AtomicNumber,
}
impl Element {
    pub const fn init(symbol: Symbol, atomic_number: AtomicNumber) -> Self {
        Self {
            symbol,
            atomic_number,
        }
    }
}
impl Add for Element {
    type Output = Molecule;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
