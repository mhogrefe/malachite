// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, FactoradicDigits, WrappingFrom};
use alloc::vec::Vec;

// The digit for $i!$ is the remainder of $\lfloor x / i! \rfloor$ divided by $i + 1$, so the digits
// come from dividing by 2, 3, 4, and so on in turn. Every base fits in any unsigned type, since a
// type with $w$ bits holds no $x$ with more than $w$ digits.
fn to_factoradic_digits_asc<T: PrimitiveUnsigned>(x: T) -> Vec<u64>
where
    u64: WrappingFrom<T>,
{
    let mut digits = Vec::new();
    let mut x = x;
    let mut base = 2u64;
    while x != T::ZERO {
        digits.push(u64::wrapping_from(x.div_assign_mod(T::exact_from(base))));
        base += 1;
    }
    digits
}

// Evaluates digits in descending order by Horner's rule: with the coefficient of $p!$ coming $p$-th
// from the end, $x = (\cdots((d_m)m + d_{m-1})(m-1) + \cdots + d_1) \cdot 1$. Leading zeros are
// skipped before any multiplication, so that a long run of them doesn't overflow a place.
pub(crate) fn from_factoradic_digits_desc_slice<T: PrimitiveUnsigned>(digits: &[u64]) -> Option<T> {
    let mut x = T::ZERO;
    let len = u64::exact_from(digits.len());
    for (place, &digit) in (1..=len).rev().zip(digits) {
        if digit > place {
            return None;
        }
        if x == T::ZERO && digit == 0 {
            continue;
        }
        x = x
            .checked_add(T::try_from(digit).ok()?)?
            .checked_mul(T::try_from(place).ok()?)?;
    }
    Some(x)
}

macro_rules! impl_factoradic_digits {
    ($t:ident) => {
        impl FactoradicDigits for $t {
            /// Returns a [`Vec`] containing the factorial-number-system digits of a number in
            /// ascending order: the digit at index $i$ is the coefficient of $(i+1)!$.
            ///
            /// If `self` is 0, the [`Vec`] is empty; otherwise, it ends with a nonzero digit.
            ///
            /// $f(x) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i \leq i + 1$ for all $i$, $k=0$ or
            /// $d_{k-1} \neq 0$, and
            ///
            /// $$
            /// \sum_{i=0}^{k-1}(i+1)! d_i = x.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::factoradic_digits#to_factoradic_digits_asc).
            #[inline]
            fn to_factoradic_digits_asc(&self) -> Vec<u64> {
                to_factoradic_digits_asc(*self)
            }

            /// Returns a [`Vec`] containing the factorial-number-system digits of a number in
            /// descending order: the last digit is the coefficient of $1!$.
            ///
            /// If `self` is 0, the [`Vec`] is empty; otherwise, it begins with a nonzero digit.
            ///
            /// $f(x) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i \leq k - i$ for all $i$, $k=0$ or
            /// $d_0 \neq 0$, and
            ///
            /// $$
            /// \sum_{i=0}^{k-1}(k-i)! d_i = x.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::factoradic_digits#to_factoradic_digits_desc).
            #[inline]
            fn to_factoradic_digits_desc(&self) -> Vec<u64> {
                let mut digits = to_factoradic_digits_asc(*self);
                digits.reverse();
                digits
            }

            /// Converts an iterator of factorial-number-system digits into a value.
            ///
            /// The input digits are in ascending order: the digit at index $i$ is the coefficient
            /// of $(i+1)!$. The function returns `None` if any digit at index $i$ is greater than
            /// $i + 1$, or if the input represents a number that can't fit in `Self`. Trailing
            /// zeros are allowed.
            ///
            /// $$
            /// f((d_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}(i+1)! d_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
            ///
            /// # Examples
            /// See [here](super::factoradic_digits#from_factoradic_digits_asc).
            #[inline]
            fn from_factoradic_digits_asc<I: Iterator<Item = u64>>(digits: I) -> Option<$t> {
                let mut digits: Vec<u64> = digits.collect();
                digits.reverse();
                from_factoradic_digits_desc_slice(&digits)
            }

            /// Converts an iterator of factorial-number-system digits into a value.
            ///
            /// The input digits are in descending order: the last digit is the coefficient of $1!$.
            /// The function returns `None` if any digit is greater than its place, the number of
            /// digits from it to the end, or if the input represents a number that can't fit in
            /// `Self`. Leading zeros are allowed.
            ///
            /// $$
            /// f((d_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}(k-i)! d_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
            ///
            /// # Examples
            /// See [here](super::factoradic_digits#from_factoradic_digits_desc).
            #[inline]
            fn from_factoradic_digits_desc<I: Iterator<Item = u64>>(digits: I) -> Option<$t> {
                let digits: Vec<u64> = digits.collect();
                from_factoradic_digits_desc_slice(&digits)
            }
        }
    };
}
apply_to_unsigneds!(impl_factoradic_digits);
