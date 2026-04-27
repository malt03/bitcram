use crate::{Buffer, Packable};

#[derive(Debug)]
pub struct Unpacker<B> {
    buffer: B,
}

impl<B: Buffer> Unpacker<B> {
    #[inline]
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }

    #[inline]
    pub fn unpack<P: Packable<B>>(&mut self) -> P {
        P::unpack(self.raw_unpack(P::SIZE))
    }

    #[inline]
    pub fn raw_unpack(&mut self, size: u32) -> B {
        debug_assert!(size <= B::BITS);
        if size == 0 {
            return B::ZERO;
        }
        let mask = B::MAX >> (B::BITS - size);
        let packed = self.buffer & mask;
        self.buffer = self.buffer >> size;
        packed
    }

    #[inline]
    pub fn into_inner(self) -> B {
        self.buffer
    }
}
