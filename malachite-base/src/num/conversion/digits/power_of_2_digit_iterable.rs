// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivRound, SaturatingSubAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{
    ExactFrom, PowerOf2DigitIterable, PowerOf2DigitIterator, WrappingFrom,
};
use crate::num::logic::traits::BitBlockAccess;
use crate::rounding_modes::RoundingMode::*;
use core::marker::PhantomData;

/// A double-ended iterator over the base-$2^k$ digits of an unsigned primitive integer.
///
/// This `struct` is created by the [`PowerOf2DigitIterable::power_of_2_digits`] function. See its
/// documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimitivePowerOf2DigitIterator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    pub(crate) value: T,
    pub(crate) log_base: u64,
    pub(crate) remaining: usize,
    // If `n` is nonzero, this index initially points to the least-significant bit of the least-
    // significant digit, and is left-shifted by `next`.
    pub(crate) i: u64,
    // If `n` is nonzero, this mask initially points to the least-significant bit of the most-
    // significant nonzero digit, and is right-shifted by `next_back`.
    pub(crate) j: u64,
    phantom: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    Iterator for PrimitivePowerOf2DigitIterator<T, U>
{
    type Item = U;

    fn next(&mut self) -> Option<U> {
        if self.remaining != 0 {
            let digit = U::wrapping_from(self.value.get_bits(self.i, self.i + self.log_base));
            self.i += self.log_base;
            self.remaining -= 1;
            Some(digit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    DoubleEndedIterator for PrimitivePowerOf2DigitIterator<T, U>
{
    fn next_back(&mut self) -> Option<U> {
        if self.remaining != 0 {
            let digit = U::wrapping_from(self.value.get_bits(self.j, self.j + self.log_base));
            self.j.saturating_sub_assign(self.log_base);
            self.remaining -= 1;
            Some(digit)
        } else {
            None
        }
    }
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    ExactSizeIterator for PrimitivePowerOf2DigitIterator<T, U>
{
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    PowerOf2DigitIterator<U> for PrimitivePowerOf2DigitIterator<T, U>
{
    /// Retrieves base-$2^k$ digits by index.
    ///
    /// Indexing at or above the significant digit count returns zero.
    ///
    /// This function doesn't affect, and isn't affected by, the iterator's position.
    ///
    /// $f(x, k, i) = d_i$, where $0 \leq d_i < 2^k$ for all $i$ and
    /// $$
    /// \sum_{i=0}^\infty2^{ki}d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::{
    ///     PowerOf2DigitIterable, PowerOf2DigitIterator,
    /// };
    ///
    /// let digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(0u8, 2);
    /// assert_eq!(digits.get(0), 0);
    ///
    /// // 107 = 1101011b
    /// let digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(107u32, 2);
    /// assert_eq!(digits.get(0), 3);
    /// assert_eq!(digits.get(1), 2);
    /// assert_eq!(digits.get(2), 2);
    /// assert_eq!(digits.get(100), 0);
    /// ```
    fn get(&self, index: u64) -> U {
        let i = index * self.log_base;
        U::wrapping_from(self.value.get_bits(i, i + self.log_base))
    }
}

fn power_of_2_digits<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    x: T,
    log_base: u64,
) -> PrimitivePowerOf2DigitIterator<T, U> {
    assert_ne!(log_base, 0);
    assert!(
        log_base <= U::WIDTH,
        "type {:?} is too small for a digit of width {}",
        U::NAME,
        log_base
    );
    let significant_digits = x.significant_bits().div_round(log_base, Ceiling).0;
    PrimitivePowerOf2DigitIterator {
        value: x,
        log_base,
        remaining: usize::exact_from(significant_digits),
        i: 0,
        j: significant_digits.saturating_sub(1) * log_base,
        phantom: PhantomData,
    }
}

macro_rules! impl_power_of_2_digit_iterable {
    ($t:ident) => {
        macro_rules! impl_power_of_2_digit_iterable_inner {
            ($u:ident) => {
                impl PowerOf2DigitIterable<$u> for $t {
                    type PowerOf2DigitIterator = PrimitivePowerOf2DigitIterator<$t, $u>;

                    /// Returns a double-ended iterator over the base-$2^k$ digits of a primitive
                    /// unsigned integer.
                    ///
                    /// The forward order is ascending, so that less-significant digits appear
                    /// first. There are no trailing zeros going forward, or leading zeros going
                    /// backward.
                    ///
                    /// If it's necessary to get a [`Vec`] of all the digits, consider using
                    /// [`to_power_of_2_digits_asc`](super::super::traits::PowerOf2Digits::to_power_of_2_digits_asc)
                    /// or
                    /// [`to_power_of_2_digits_desc`](super::super::traits::PowerOf2Digits::to_power_of_2_digits_desc)
                    /// instead.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is larger than the width of output type width.
                    ///
                    /// # Examples
                    /// See [here](super::power_of_2_digit_iterable#power_of_2_digits).
                    #[inline]
                    fn power_of_2_digits(
                        self,
                        log_base: u64,
                    ) -> PrimitivePowerOf2DigitIterator<$t, $u> {
                        power_of_2_digits(self, log_base)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_power_of_2_digit_iterable_inner);
    };
}
apply_to_unsigneds!(impl_power_of_2_digit_iterable);
