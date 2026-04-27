pub mod a;

use bitpacker::packable;

#[packable(u128)]
#[derive(Debug, PartialEq, Eq)]
pub struct B;

#[packable(u128)]
#[derive(Debug, PartialEq, Eq)]
pub struct C(a::A, a::A, B);

#[packable(u128)]
#[derive(Debug, PartialEq, Eq)]
pub enum D {
    Unit,
    Empty(),
    Tuple(a::A, C),
    Named { x: B, y: C },
}

#[packable(u128)]
#[derive(Debug, PartialEq, Eq)]
pub enum E {
    Unit,
}

impl D {
    pub fn unit() -> Self {
        D::Unit
    }

    pub fn empty() -> Self {
        D::Empty()
    }

    pub fn tuple(a: u8, c0: u8, c1: u8) -> Self {
        D::Tuple(a::A(a), C(a::A(c0), a::A(c1), B))
    }

    pub fn named(c0: u8, c1: u8) -> Self {
        D::Named {
            x: B,
            y: C(a::A(c0), a::A(c1), B),
        }
    }
}
