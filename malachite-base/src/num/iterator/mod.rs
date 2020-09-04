use std::marker::PhantomData;

use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;

#[derive(Clone, Debug)]
pub struct IteratorToBitChunks<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    xs: I,
    x: T,
    x_width: u64,
    y_width: u64,
    remaining_x_bits: u64,
    in_inner_loop: bool,
    phantom: PhantomData<U>,
}

impl<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    IteratorToBitChunks<I, T, U>
{
    pub fn next_with_wrapping<F: Fn(T) -> U>(&mut self, wrap: F) -> Option<U> {
        let mut y = U::ZERO;
        let mut remaining_y_bits = self.y_width;
        loop {
            if !self.in_inner_loop {
                if let Some(x) = self.xs.next() {
                    self.x = x;
                } else {
                    break;
                }
                self.remaining_x_bits = self.x_width;
                self.in_inner_loop = true;
            }
            while self.remaining_x_bits != 0 {
                let y_index = self.y_width - remaining_y_bits;
                if self.remaining_x_bits <= remaining_y_bits {
                    y |= wrap(self.x) << y_index;
                    remaining_y_bits -= self.remaining_x_bits;
                    self.remaining_x_bits = 0;
                } else {
                    y |= wrap(self.x).mod_power_of_two(remaining_y_bits) << y_index;
                    self.x >>= remaining_y_bits;
                    self.remaining_x_bits -= remaining_y_bits;
                    remaining_y_bits = 0;
                }
                if remaining_y_bits == 0 {
                    return Some(y);
                }
            }
            self.in_inner_loop = false;
        }
        if y == U::ZERO {
            None
        } else {
            Some(y)
        }
    }
}

impl<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<T>> Iterator
    for IteratorToBitChunks<I, T, U>
{
    type Item = U;

    #[inline]
    fn next(&mut self) -> Option<U> {
        self.next_with_wrapping(U::wrapping_from)
    }
}

pub fn iterator_to_bit_chunks<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    xs: I,
    in_chunk_size: u64,
    out_chunk_size: u64,
) -> IteratorToBitChunks<I, T, U> {
    assert!(in_chunk_size <= T::WIDTH);
    assert!(out_chunk_size <= U::WIDTH);
    IteratorToBitChunks {
        xs,
        x: T::ZERO,
        x_width: in_chunk_size,
        y_width: out_chunk_size,
        remaining_x_bits: 0,
        in_inner_loop: false,
        phantom: PhantomData,
    }
}
