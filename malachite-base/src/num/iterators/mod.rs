use std::cmp::Ordering;
use std::marker::PhantomData;

use num::arithmetic::traits::DivisibleBy;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct SameWidthIteratorToBitChunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    xs: I,
    phantom: PhantomData<U>,
}

impl<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    SameWidthIteratorToBitChunks<I, T, U>
{
    fn next_with_wrapping<F: Fn(T) -> U>(&mut self, wrap: F) -> Option<U> {
        self.xs.next().map(wrap)
    }
}

fn same_width_iterator_to_bit_chunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    xs: I,
) -> SameWidthIteratorToBitChunks<I, T, U> {
    SameWidthIteratorToBitChunks {
        xs,
        phantom: PhantomData,
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct EvenFractionIteratorToBitChunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    xs: I,
    x: T,
    multiple: u64,
    y_width: u64,
    counter: u64,
    phantom: PhantomData<U>,
}

impl<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    EvenFractionIteratorToBitChunks<I, T, U>
{
    fn next_with_wrapping<F: Fn(T) -> U>(&mut self, wrap: F) -> Option<U> {
        if self.counter == 0 {
            if let Some(x) = self.xs.next() {
                self.x = x;
                self.counter = self.multiple;
            } else {
                return None;
            }
        } else {
            self.x >>= self.y_width;
        }
        let y = wrap(self.x.mod_power_of_two(self.y_width));
        self.counter -= 1;
        Some(y)
    }
}

fn even_fraction_iterator_to_bit_chunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    xs: I,
    multiple: u64,
    out_chunk_size: u64,
) -> EvenFractionIteratorToBitChunks<I, T, U> {
    EvenFractionIteratorToBitChunks {
        xs,
        x: T::ZERO,
        multiple,
        y_width: out_chunk_size,
        counter: 0,
        phantom: PhantomData,
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct EvenMultipleIteratorToBitChunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    xs: I,
    x: T,
    x_width: u64,
    y_width: u64,
    done: bool,
    phantom: PhantomData<U>,
}

impl<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    EvenMultipleIteratorToBitChunks<I, T, U>
{
    fn next_with_wrapping<F: Fn(T) -> U>(&mut self, wrap: F) -> Option<U> {
        if self.done {
            return None;
        }
        let mut y = U::ZERO;
        let mut shift = 0;
        while shift < self.y_width {
            if let Some(x) = self.xs.next() {
                y |= wrap(x) << shift;
                shift += self.x_width;
            } else {
                self.done = true;
                break;
            }
        }
        if shift == 0 {
            None
        } else {
            Some(y)
        }
    }
}

fn even_multiple_iterator_to_bit_chunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    xs: I,
    in_chunk_size: u64,
    out_chunk_size: u64,
) -> EvenMultipleIteratorToBitChunks<I, T, U> {
    EvenMultipleIteratorToBitChunks {
        xs,
        x: T::ZERO,
        x_width: in_chunk_size,
        y_width: out_chunk_size,
        done: false,
        phantom: PhantomData,
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct IrregularIteratorToBitChunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    xs: I,
    x: T,
    x_width: u64,
    y_width: u64,
    remaining_x_bits: u64,
    in_inner_loop: bool,
    phantom: PhantomData<U>,
}

impl<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    IrregularIteratorToBitChunks<I, T, U>
{
    fn next_with_wrapping<F: Fn(T) -> U>(&mut self, wrap: F) -> Option<U> {
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

fn irregular_iterator_to_bit_chunks<
    I: Iterator<Item = T>,
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    xs: I,
    in_chunk_size: u64,
    out_chunk_size: u64,
) -> IrregularIteratorToBitChunks<I, T, U> {
    IrregularIteratorToBitChunks {
        xs,
        x: T::ZERO,
        x_width: in_chunk_size,
        y_width: out_chunk_size,
        remaining_x_bits: 0,
        in_inner_loop: false,
        phantom: PhantomData,
    }
}

/// Regroups an iterator of bit chunks into another iterator of bit chunks, possibly with a
/// different chunk size.
///
/// This `enum` is created by the `iterator_to_bit_chunks` function. See its documentation for more.
#[derive(Clone, Debug)]
pub enum IteratorToBitChunks<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    SameWidth(SameWidthIteratorToBitChunks<I, T, U>),
    EvenFraction(EvenFractionIteratorToBitChunks<I, T, U>),
    EvenMultiple(EvenMultipleIteratorToBitChunks<I, T, U>),
    Irregular(IrregularIteratorToBitChunks<I, T, U>),
}

impl<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    IteratorToBitChunks<I, T, U>
{
    pub(crate) fn next_with_wrapping<F: Fn(T) -> U>(&mut self, wrap: F) -> Option<U> {
        match *self {
            IteratorToBitChunks::SameWidth(ref mut xs) => xs.next_with_wrapping(wrap),
            IteratorToBitChunks::EvenFraction(ref mut xs) => xs.next_with_wrapping(wrap),
            IteratorToBitChunks::EvenMultiple(ref mut xs) => xs.next_with_wrapping(wrap),
            IteratorToBitChunks::Irregular(ref mut xs) => xs.next_with_wrapping(wrap),
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

/// Regroups an iterator of bit chunks into another iterator of bit chunks, possibly with a
/// different chunk size.
///
/// In other words, let $A$ be the input chunk size and $B$ the output chunk size. Let's consider
/// the lowest $A$ bits of each unsigned value produced by the input iterator, and concatenate them
/// (least-significant bits first) into a single bit sequence. Then we chop the sequence up into
/// chunks of $B$ bits, and have the output iterator return each chunk.
///
/// Let $(x\_i)\_{i=0}^{n-1}$ be the input iterator, where $n$ may be $\infty$. If $n$ is finite, we
/// assume that $x\_{n-1} \neq 0$. Then we define the bit sequence $b\_{k=0}^\infty$ such that
/// $b \in \\{0, 1\\}$, $b\_k=0$ for $k \geq An$, and
/// $$
/// x_i = \sum_{k=0}^{A-1} b_{Ai+k}2^k.
/// $$
/// Then, let $(y\_j)\_{j=0}^{m-1}$ be a sequence such that
/// $$
/// y_j = \sum_{k=0}^{B-1} b_{Bi+k}2^k.
/// $$
/// Then we have $f((x\_i)\_{i=0}^{n-1}) = (y\_j)\_{j=0}^{m-1}$.
/// Note that the sequence $y$ is not uniquely specified, since it may contain arbitrarily many
/// trailing zeros. However, if $x$ is finite, $y$ is guaranteed to also be finite.
///
/// The output length is $An/B + O(1)$, where $n$ is $xs.count()$, $A$ is `in_chunk_size`, and $B$
/// is `out_chunk_size`.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::iterators::iterator_to_bit_chunks;
///
/// assert_eq!(
///     iterator_to_bit_chunks::<_, u16, u32>([123, 456].iter().cloned(), 10, 10)
///         .collect::<Vec<_>>().as_slice(),
///     &[123, 456]
/// );
/// assert_eq!(
///     iterator_to_bit_chunks::<_, u16, u16>([0b000111111, 0b110010010].iter().cloned(), 9, 3)
///         .collect::<Vec<_>>().as_slice(),
///     &[0b111, 0b111, 0b000, 0b010, 0b010, 0b110]
/// );
/// assert_eq!(
///     iterator_to_bit_chunks::<_, u16, u32>(
///         [0b111, 0b111, 0b000, 0b010, 0b010, 0b110].iter().cloned(),
///         3,
///         9
///     ).collect::<Vec<_>>().as_slice(),
///     &[0b000111111, 0b110010010]
/// );
/// assert_eq!(
///     iterator_to_bit_chunks::<_, u32, u16>(
///         [0b1010101, 0b1111101, 0b0100001, 0b110010].iter().cloned(),
///         7,
///         6
///     ).collect::<Vec<_>>().as_slice(),
///     &[0b010101, 0b111011, 0b000111, 0b010010, 0b110]
/// );
/// ```
pub fn iterator_to_bit_chunks<I: Iterator<Item = T>, T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    xs: I,
    in_chunk_size: u64,
    out_chunk_size: u64,
) -> IteratorToBitChunks<I, T, U> {
    assert_ne!(in_chunk_size, 0);
    assert_ne!(out_chunk_size, 0);
    assert!(in_chunk_size <= T::WIDTH);
    assert!(out_chunk_size <= U::WIDTH);
    match in_chunk_size.cmp(&out_chunk_size) {
        Ordering::Equal => {
            return IteratorToBitChunks::SameWidth(same_width_iterator_to_bit_chunks(xs))
        }
        Ordering::Less => {
            if out_chunk_size.divisible_by(in_chunk_size) {
                return IteratorToBitChunks::EvenMultiple(even_multiple_iterator_to_bit_chunks(
                    xs,
                    in_chunk_size,
                    out_chunk_size,
                ));
            }
        }
        Ordering::Greater => {
            let multiple = in_chunk_size / out_chunk_size;
            if multiple * out_chunk_size == in_chunk_size {
                return IteratorToBitChunks::EvenFraction(even_fraction_iterator_to_bit_chunks(
                    xs,
                    multiple,
                    out_chunk_size,
                ));
            }
        }
    }
    IteratorToBitChunks::Irregular(irregular_iterator_to_bit_chunks(
        xs,
        in_chunk_size,
        out_chunk_size,
    ))
}
