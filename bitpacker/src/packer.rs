use crate::{Buffer, Packable};

pub struct Packer<B> {
    buffer: B,
}

impl<B: Buffer> Packer<B> {
    pub fn new() -> Self {
        Self { buffer: B::ZERO }
    }

    pub fn pack<P: Packable<B>>(&mut self, packable: &P) {
        self.raw_pack(packable.pack(), P::SIZE);
    }

    pub fn raw_pack(&mut self, packed: B, size: u32) {
        debug_assert!(size <= B::BITS);
        self.buffer = packed | (self.buffer << size);
    }

    pub fn into_inner(self) -> B {
        self.buffer
    }
}
