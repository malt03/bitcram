use bitpacker::Packable;

#[derive(Debug, PartialEq, Eq)]
pub struct A(pub(crate) u8);

impl Packable<u128> for A {
    const SIZE: u32 = 3;
    fn pack(&self) -> u128 {
        self.0 as u128
    }
    fn unpack(buffer: u128) -> Self {
        Self(buffer as u8)
    }
}
