// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{
    ConvertibleFrom, Digits, ExactFrom, PowerOf2Digits, WrappingFrom,
};
use alloc::vec::Vec;
use itertools::Itertools;

pub_test! {unsigned_to_digits_asc_naive<
    T: ExactFrom<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<T>,
>(
    x: &T,
    base: U,
) -> Vec<U> {
    assert!(base > U::ONE);
    let mut digits = Vec::new();
    let mut remainder = *x;
    let base = T::exact_from(base);
    while remainder != T::ZERO {
        digits.push(U::wrapping_from(remainder.div_assign_mod(base)));
    }
    digits
}}

fn to_digits_asc<
    T: ConvertibleFrom<U> + ExactFrom<U> + PrimitiveUnsigned + PowerOf2Digits<U>,
    U: PrimitiveUnsigned + WrappingFrom<T>,
>(
    x: &T,
    base: &U,
) -> Vec<U> {
    assert!(T::convertible_from(*base));
    if *x == T::ZERO {
        Vec::new()
    } else if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_asc(log_base)
    } else {
        unsigned_to_digits_asc_naive(x, *base)
    }
}

fn to_digits_desc<
    T: ConvertibleFrom<U> + ExactFrom<U> + PrimitiveUnsigned + PowerOf2Digits<U>,
    U: PrimitiveUnsigned + WrappingFrom<T>,
>(
    x: &T,
    base: &U,
) -> Vec<U> {
    assert!(T::convertible_from(*base));
    if *x == T::ZERO {
        Vec::new()
    } else if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_desc(log_base)
    } else {
        let mut digits = unsigned_to_digits_asc_naive(x, *base);
        digits.reverse();
        digits
    }
}

fn from_digits_asc<
    T: Digits<U> + PowerOf2Digits<U>,
    U: PrimitiveUnsigned,
    I: Iterator<Item = U>,
>(
    base: &U,
    digits: I,
) -> Option<T> {
    if let Some(log_base) = base.checked_log_base_2() {
        T::from_power_of_2_digits_asc(log_base, digits)
    } else {
        let mut digits = digits.collect_vec();
        digits.reverse();
        T::from_digits_desc(base, digits.into_iter())
    }
}

fn from_digits_desc<
    T: Digits<U> + TryFrom<U> + PrimitiveUnsigned + PowerOf2Digits<U>,
    U: PrimitiveUnsigned,
    I: Iterator<Item = U>,
>(
    base: &U,
    digits: I,
) -> Option<T> {
    assert!(*base > U::ONE);
    if let Some(log_base) = base.checked_log_base_2() {
        T::from_power_of_2_digits_desc(log_base, digits)
    } else {
        let base = T::try_from(*base).ok()?;
        let mut x = T::ZERO;
        for digit in digits {
            let digit = T::try_from(digit).ok()?;
            if digit >= base {
                return None;
            }
            x = x.checked_mul(base)?.checked_add(digit)?;
        }
        Some(x)
    }
}

macro_rules! impl_digits {
    ($t:ident) => {
        macro_rules! impl_digits_inner {
            ($u:ident) => {
                impl Digits<$u> for $t {
                    /// Returns a [`Vec`] containing the digits of a number in ascending order
                    /// (least- to most-significant).
                    ///
                    /// The base must be convertible to `Self`. If `self` is 0, the [`Vec`] is
                    /// empty; otherwise, it ends with a nonzero digit.
                    ///
                    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
                    /// $d_{k-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}b^i d_i = x.
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
                    /// Panics if `base` is less than 2 or greater than `Self::MAX`.
                    ///
                    /// # Examples
                    /// See [here](super::general_digits#to_digits_asc).
                    #[inline]
                    fn to_digits_asc(&self, base: &$u) -> Vec<$u> {
                        to_digits_asc(self, base)
                    }

                    /// Returns a [`Vec`] containing the digits of a number in descending order
                    /// (most- to least-significant).
                    ///
                    /// The base must be convertible to `Self`. If `self` is 0, the [`Vec`] is
                    /// empty; otherwise, it begins with a nonzero digit.
                    ///
                    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
                    /// $d_{k-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}b^i d_{k-i-1} = x.
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
                    /// Panics if `base` is less than 2 or greater than `$t::MAX`.
                    ///
                    /// # Examples
                    /// See [here](super::general_digits#to_digits_desc).
                    #[inline]
                    fn to_digits_desc(&self, base: &$u) -> Vec<$u> {
                        to_digits_desc(self, base)
                    }

                    /// Converts an iterator of digits into a value.
                    ///
                    /// The input digits are in ascending order (least- to most-significant). The
                    /// base must be no larger than `Self::MAX`. The function returns `None` if the
                    /// input represents a number that can't fit in `Self`, if `base` is greater
                    /// than `Self::MAX`, or if any of the digits are greater than or equal to the
                    /// base.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^id_i.
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
                    /// Panics if `base` is less than 2.
                    ///
                    /// # Examples
                    /// See [here](super::general_digits#from_digits_asc).
                    #[inline]
                    fn from_digits_asc<I: Iterator<Item = $u>>(base: &$u, digits: I) -> Option<$t> {
                        from_digits_asc(base, digits)
                    }

                    /// Converts an iterator of digits into a value.
                    ///
                    /// The input digits are in descending order (most- to least-significant). The
                    /// base must be no larger than `Self::MAX`. The function returns `None` if the
                    /// input represents a number that can't fit in `Self`, if `base` is greater
                    /// than `Self::MAX`, or if any of the digits are greater than or equal to the
                    /// base.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^{k-i-1}d_i.
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
                    /// Panics if `base` is less than 2.
                    ///
                    /// # Examples
                    /// See [here](super::general_digits#from_digits_desc).
                    #[inline]
                    fn from_digits_desc<I: Iterator<Item = $u>>(
                        base: &$u,
                        digits: I,
                    ) -> Option<$t> {
                        from_digits_desc(base, digits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_digits_inner);
    };
}
apply_to_unsigneds!(impl_digits);
