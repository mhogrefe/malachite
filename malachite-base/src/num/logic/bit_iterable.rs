// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::logic::traits::BitIterable;
use core::cmp::min;
use core::cmp::Ordering::*;
use core::marker::PhantomData;
use core::ops::Index;

/// A double-ended iterator over the bits of an unsigned primitive integer.
///
/// This `struct` is created by [`BitIterable::bits`]; see its documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveUnsignedBitIterator<T: PrimitiveUnsigned> {
    pub(crate) value: T,
    pub(crate) remaining: usize,
    // If `n` is nonzero, this mask initially points to the least-significant bit, and is left-
    // shifted by next().
    pub(crate) i_mask: T,
    // If `n` is nonzero, this mask initially points to the most-significant nonzero bit, and is
    // right-shifted by next_back().
    pub(crate) j_mask: T,
}

impl<T: PrimitiveUnsigned> Iterator for PrimitiveUnsignedBitIterator<T> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.remaining != 0 {
            let bit = self.value & self.i_mask != T::ZERO;
            self.i_mask <<= 1;
            self.remaining -= 1;
            Some(bit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: PrimitiveUnsigned> DoubleEndedIterator for PrimitiveUnsignedBitIterator<T> {
    fn next_back(&mut self) -> Option<bool> {
        if self.remaining != 0 {
            let bit = self.value & self.j_mask != T::ZERO;
            self.j_mask >>= 1;
            self.remaining -= 1;
            Some(bit)
        } else {
            None
        }
    }
}

impl<T: PrimitiveUnsigned> ExactSizeIterator for PrimitiveUnsignedBitIterator<T> {}

impl<T: PrimitiveUnsigned> Index<u64> for PrimitiveUnsignedBitIterator<T> {
    type Output = bool;

    /// A function to retrieve bits by index.
    ///
    /// The index is the power of 2 of which the bit is a coefficient. Indexing at or above the
    /// significant bit count returns false bits.
    ///
    /// This is equivalent to [`get_bit`](super::traits::BitAccess::get_bit).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0u8.bits()[0], false);
    ///
    /// // 105 = 1101001b
    /// let bits = 105u32.bits();
    /// assert_eq!(bits[0], true);
    /// assert_eq!(bits[1], false);
    /// assert_eq!(bits[2], false);
    /// assert_eq!(bits[3], true);
    /// assert_eq!(bits[4], false);
    /// assert_eq!(bits[5], true);
    /// assert_eq!(bits[6], true);
    /// assert_eq!(bits[7], false);
    /// assert_eq!(bits[100], false);
    /// ```
    fn index(&self, index: u64) -> &bool {
        if self.value.get_bit(index) {
            &true
        } else {
            &false
        }
    }
}

fn bits_unsigned<T: PrimitiveUnsigned>(x: T) -> PrimitiveUnsignedBitIterator<T> {
    let significant_bits = x.significant_bits();
    PrimitiveUnsignedBitIterator {
        value: x,
        remaining: usize::exact_from(significant_bits),
        i_mask: T::ONE,
        j_mask: T::power_of_2(significant_bits.saturating_sub(1)),
    }
}

macro_rules! impl_bit_iterable_unsigned {
    ($t:ident) => {
        impl BitIterable for $t {
            type BitIterator = PrimitiveUnsignedBitIterator<$t>;

            /// Returns a double-ended iterator over the bits of an unsigned primitive integer.
            ///
            /// The forward order is ascending, so that less significant bits appear first. There
            /// are no trailing false bits going forward, or leading falses going backward.
            ///
            /// If it's necessary to get a [`Vec`] of all the bits, consider using
            /// [`to_bits_asc`](super::traits::BitConvertible::to_bits_asc) or
            /// [`to_bits_desc`](super::traits::BitConvertible::to_bits_desc) instead.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_iterable#bits).
            #[inline]
            fn bits(self) -> PrimitiveUnsignedBitIterator<$t> {
                bits_unsigned(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_iterable_unsigned);

/// A double-ended iterator over the bits of a signed primitive integer.
///
/// This `struct` is created by [`BitIterable::bits`]; see its documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveSignedBitIterator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom: PhantomData<*const S>,
    xs: PrimitiveUnsignedBitIterator<U>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned> Iterator for PrimitiveSignedBitIterator<U, S> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        self.xs.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.xs.size_hint()
    }
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned> DoubleEndedIterator
    for PrimitiveSignedBitIterator<U, S>
{
    #[inline]
    fn next_back(&mut self) -> Option<bool> {
        self.xs.next_back()
    }
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned> ExactSizeIterator
    for PrimitiveSignedBitIterator<U, S>
{
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned> Index<u64> for PrimitiveSignedBitIterator<U, S> {
    type Output = bool;

    /// A function to retrieve bits by index. The index is the power of 2 of which the bit is a
    /// coefficient.
    ///
    /// Indexing at or above the significant bit count returns false or true bits, depending on the
    /// value's sign.
    ///
    /// This is equivalent to [`get_bit`](super::traits::BitAccess::get_bit).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitIterable;
    ///
    /// assert_eq!(0i8.bits()[0], false);
    ///
    /// // -105 = 10010111 in two's complement
    /// let bits = (-105i32).bits();
    /// assert_eq!(bits[0], true);
    /// assert_eq!(bits[1], true);
    /// assert_eq!(bits[2], true);
    /// assert_eq!(bits[3], false);
    /// assert_eq!(bits[4], true);
    /// assert_eq!(bits[5], false);
    /// assert_eq!(bits[6], false);
    /// assert_eq!(bits[7], true);
    /// assert_eq!(bits[100], true);
    /// ```
    fn index(&self, index: u64) -> &bool {
        if self.xs[min(index, U::WIDTH - 1)] {
            &true
        } else {
            &false
        }
    }
}

fn bits_signed<U: PrimitiveUnsigned + WrappingFrom<S>, S: PrimitiveSigned>(
    x: S,
) -> PrimitiveSignedBitIterator<U, S> {
    let unsigned = U::wrapping_from(x);
    let significant_bits = match x.sign() {
        Equal => 0,
        Greater => unsigned.significant_bits() + 1,
        Less => (!unsigned).significant_bits() + 1,
    };
    PrimitiveSignedBitIterator {
        phantom: PhantomData,
        xs: PrimitiveUnsignedBitIterator {
            value: unsigned,
            remaining: usize::exact_from(significant_bits),
            i_mask: U::ONE,
            j_mask: U::power_of_2(significant_bits.saturating_sub(1)),
        },
    }
}

macro_rules! impl_bit_iterable_signed {
    ($u:ident, $s:ident) => {
        impl BitIterable for $s {
            type BitIterator = PrimitiveSignedBitIterator<$u, $s>;

            /// Returns a double-ended iterator over the bits of a signed primitive integer.
            ///
            /// The forward order is ascending, so that less significant bits appear first. There
            /// are no trailing sign bits going forward, or leading sign bits going backward.
            ///
            /// If it's necessary to get a [`Vec`] of all the bits, consider using
            /// [`to_bits_asc`](super::traits::BitConvertible::to_bits_asc) or
            /// [`to_bits_desc`](super::traits::BitConvertible::to_bits_desc) instead.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_iterable#bits).
            #[inline]
            fn bits(self) -> PrimitiveSignedBitIterator<$u, $s> {
                bits_signed::<$u, $s>(self)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_bit_iterable_signed);
