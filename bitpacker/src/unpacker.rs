use crate::{Buffer, Packable};

pub struct Unpacker<B> {
    buffer: B,
}

impl<B: Buffer> Unpacker<B> {
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    pub fn unpack<P: Packable<B>>(&mut self) -> P {
        P::unpack(self.raw_unpack(P::SIZE))
    }

    pub fn raw_unpack(&mut self, size: u32) -> B {
        debug_assert!(size <= B::BITS);
        let mask = B::MAX >> (B::BITS - size);
        let packed = self.buffer & mask;
        self.buffer = self.buffer >> size;
        packed
    }

    pub fn into_inner(self) -> B {
        self.buffer
    }
}
