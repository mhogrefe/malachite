// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{PowerOf2Digits, WrappingFrom};
use alloc::vec::Vec;

fn to_power_of_2_digits_asc<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<T>>(
    x: &T,
    log_base: u64,
) -> Vec<U> {
    assert_ne!(log_base, 0);
    assert!(
        log_base <= U::WIDTH,
        "type {:?} is too small for a digit of width {}",
        U::NAME,
        log_base
    );
    let mut digits = Vec::new();
    if *x == T::ZERO {
    } else if x.significant_bits() <= log_base {
        digits.push(U::wrapping_from(*x));
    } else {
        let mut x = *x;
        let mask = U::low_mask(log_base);
        while x != T::ZERO {
            digits.push(U::wrapping_from(x) & mask);
            x >>= log_base;
        }
    }
    digits
}

fn to_power_of_2_digits_desc<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<T>>(
    x: &T,
    log_base: u64,
) -> Vec<U> {
    let mut digits = to_power_of_2_digits_asc(x, log_base);
    digits.reverse();
    digits
}

fn from_power_of_2_digits_asc<
    T: TryFrom<U> + PrimitiveUnsigned + WrappingFrom<U>,
    U: PrimitiveUnsigned,
    I: Iterator<Item = U>,
>(
    log_base: u64,
    digits: I,
) -> Option<T> {
    assert_ne!(log_base, 0);
    assert!(
        log_base <= U::WIDTH,
        "type {:?} is too small for a digit of width {}",
        U::NAME,
        log_base
    );
    let mut n = T::ZERO;
    let mut shift = 0;
    for digit in digits {
        if digit.significant_bits() > log_base {
            return None;
        }
        n |= T::try_from(digit)
            .ok()
            .and_then(|d| d.arithmetic_checked_shl(shift))?;
        shift += log_base;
    }
    Some(n)
}

fn from_power_of_2_digits_desc<
    T: PrimitiveUnsigned + WrappingFrom<U>,
    U: PrimitiveUnsigned,
    I: Iterator<Item = U>,
>(
    log_base: u64,
    digits: I,
) -> Option<T> {
    assert_ne!(log_base, 0);
    assert!(
        log_base <= U::WIDTH,
        "type {:?} is too small for a digit of width {}",
        U::NAME,
        log_base
    );
    let mut n = T::ZERO;
    for digit in digits {
        if digit.significant_bits() > log_base {
            return None;
        }
        let shifted = n.arithmetic_checked_shl(log_base)?;
        n = shifted | T::wrapping_from(digit);
    }
    Some(n)
}

macro_rules! impl_power_of_2_digits {
    ($t:ident) => {
        macro_rules! impl_power_of_2_digits_inner {
            ($u:ident) => {
                impl PowerOf2Digits<$u> for $t {
                    /// Returns a [`Vec`] containing the base-$2^k$ digits of a number in ascending
                    /// order (least- to most-significant).
                    ///
                    /// The base-2 logarithm of the base is specified. `log_base` must be no larger
                    /// than the width of the digit type. If `self` is 0, the [`Vec`] is empty;
                    /// otherwise, it ends with a nonzero digit.
                    ///
                    /// $f(x, k) = (d_i)_ {i=0}^{n-1}$, where $0 \leq d_i < 2^k$ for all $i$, $n=0$
                    /// or $d_{n-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{n-1}2^{ki}d_i = x.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(n)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `self.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of the output type, or if
                    /// `log_base` is zero.
                    ///
                    /// # Examples
                    /// See [here](super::power_of_2_digits#to_power_of_2_digits_asc).
                    #[inline]
                    fn to_power_of_2_digits_asc(&self, log_base: u64) -> Vec<$u> {
                        to_power_of_2_digits_asc(self, log_base)
                    }

                    /// Returns a [`Vec`] containing the base-$2^k$ digits of a number in descending
                    /// order (most- to least-significant).
                    ///
                    /// The base-2 logarithm of the base is specified. `log_base` must be no larger
                    /// than the width of the digit type. If `self` is 0, the [`Vec`] is empty;
                    /// otherwise, it begins with a nonzero digit.
                    ///
                    /// $f(x, k) = (d_i)_ {i=0}^{n-1}$, where $0 \leq d_i < 2^k$ for all $i$, $n=0$
                    /// or $d_0 \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{n-1}2^{k (n-i-1)}d_i = x.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(n)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `self.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of the output type, or if
                    /// `log_base` is zero.
                    ///
                    /// # Examples
                    /// See [here](super::power_of_2_digits#to_power_of_2_digits_desc).
                    #[inline]
                    fn to_power_of_2_digits_desc(&self, log_base: u64) -> Vec<$u> {
                        to_power_of_2_digits_desc(self, log_base)
                    }

                    /// Converts an iterator of base-$2^k$ digits into a value.
                    ///
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// ascending order (least- to most-significant). `log_base` must be no larger
                    /// than the width of the digit type. The function returns `None` if the input
                    /// represents a number that can't fit in the output type.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{n-1}, k) = \sum_{i=0}^{n-1}2^{ki}d_i.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of the digit type, or if
                    /// `log_base` is zero.
                    ///
                    /// # Examples
                    /// See [here](super::power_of_2_digits#from_power_of_2_digits_asc).
                    #[inline]
                    fn from_power_of_2_digits_asc<I: Iterator<Item = $u>>(
                        log_base: u64,
                        digits: I,
                    ) -> Option<$t> {
                        from_power_of_2_digits_asc(log_base, digits)
                    }

                    /// Converts an iterator of base-$2^k$ digits into a value.
                    ///
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// descending order (most- to least-significant). `log_base` must be no larger
                    /// than the width of the digit type. The function returns `None` if the input
                    /// represents a number that can't fit in the output type.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{n-1}, k) = \sum_{i=0}^{n-1}2^{k (n-i-1)}d_i.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of the digit type, or if
                    /// `log_base` is zero.
                    ///
                    /// # Examples
                    /// See [here](super::power_of_2_digits#from_power_of_2_digits_desc).
                    fn from_power_of_2_digits_desc<I: Iterator<Item = $u>>(
                        log_base: u64,
                        digits: I,
                    ) -> Option<$t> {
                        from_power_of_2_digits_desc(log_base, digits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_power_of_2_digits_inner);
    };
}
apply_to_unsigneds!(impl_power_of_2_digits);
