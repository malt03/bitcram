use crate::{Buffer, Packable};

#[derive(Debug)]
pub struct Packer<B> {
    buffer: B,
}

impl<B: Buffer> Packer<B> {
    #[inline]
    pub fn new() -> Self {
        Self { buffer: B::ZERO }
    }

    #[inline]
    pub fn pack<P: Packable<B>>(&mut self, packable: &P) {
        self.raw_pack(packable.pack(), P::SIZE);
    }

    #[inline]
    pub fn raw_pack(&mut self, packed: B, size: u32) {
        debug_assert!(size <= B::BITS);
        if size == 0 {
            return;
        }
        self.buffer = packed | (self.buffer << size);
    }

    #[inline]
    pub fn into_inner(self) -> B {
        self.buffer
    }
}
