use std::fmt::Debug;

use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use num::random::random_unsigned_bit_chunks;
use num::random::thrifty_random::RandomPrimitiveIntegers;
use random::seed::Seed;

#[derive(Clone, Debug)]
pub struct RandomUnsignedBitChunks<T: PrimitiveUnsigned> {
    pub(crate) xs: RandomPrimitiveIntegers<T>,
    pub(crate) x: T,
    pub(crate) bits_left: u64,
    pub(crate) chunk_size: u64,
    pub(crate) mask: T,
    pub(crate) high_bits: Option<T>,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedBitChunks<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.chunk_size == 0 {
            return Some(T::ZERO);
        }
        let width_minus_chunk_size = T::WIDTH - self.chunk_size;
        Some(if self.bits_left == 0 {
            self.x = self.xs.next().unwrap();
            self.bits_left = width_minus_chunk_size;
            self.x & self.mask
        } else if self.bits_left >= self.chunk_size {
            self.x >>= self.chunk_size;
            if let Some(bits) = self.high_bits {
                self.x |= bits << width_minus_chunk_size;
                self.high_bits = None;
            }
            self.bits_left -= self.chunk_size;
            self.x & self.mask
        } else {
            let mut old_x = self.x >> self.chunk_size;
            if let Some(bits) = self.high_bits {
                old_x |= bits << width_minus_chunk_size;
            }
            self.x = self.xs.next().unwrap();
            self.high_bits = Some(self.x >> (T::WIDTH - self.bits_left));
            self.x <<= self.bits_left;
            self.bits_left += width_minus_chunk_size;
            (self.x | old_x) & self.mask
        })
    }
}

pub trait RandomSignedChunkable: Sized {
    type AbsoluteChunks: Clone + Debug;

    fn new_absolute_chunks(seed: Seed, chunk_size: u64) -> Self::AbsoluteChunks;

    fn next_chunk(xs: &mut Self::AbsoluteChunks) -> Option<Self>;
}

macro_rules! impl_random_signed_chunkable {
    ($u: ident, $s: ident) => {
        impl RandomSignedChunkable for $s {
            type AbsoluteChunks = RandomUnsignedBitChunks<$u>;

            fn new_absolute_chunks(seed: Seed, chunk_size: u64) -> RandomUnsignedBitChunks<$u> {
                random_unsigned_bit_chunks(seed, chunk_size)
            }

            fn next_chunk(xs: &mut Self::AbsoluteChunks) -> Option<$s> {
                xs.next().map(WrappingFrom::wrapping_from)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_random_signed_chunkable);

#[derive(Clone, Debug)]
pub struct RandomSignedBitChunks<T: RandomSignedChunkable> {
    pub(crate) xs: T::AbsoluteChunks,
}

impl<T: RandomSignedChunkable> Iterator for RandomSignedBitChunks<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        T::next_chunk(&mut self.xs)
    }
}
