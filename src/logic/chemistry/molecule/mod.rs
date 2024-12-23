use std::{fmt::Display, ops::Add};

use super::element::Element;

pub mod molecules;
pub struct Molecule {}
impl Add for Molecule {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Add<Element> for Molecule {
    type Output = Self;
    fn add(self, rhs: Element) -> Self::Output {
        todo!()
    }
}
impl Display for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
