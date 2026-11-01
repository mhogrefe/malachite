// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, FactoradicDigits};

// Divides by 2, 3, 4, and so on in turn, each a single limb, in place; the remainders are the
// digits.
fn to_factoradic_digits_asc(x: &Natural) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut x = x.clone();
    let mut base: Limb = 2;
    while x != 0u32 {
        digits.push(u64::exact_from(x.div_assign_mod_limb(base)));
        base += 1;
    }
    digits
}

// Horner's rule, as for the primitive types: $x = (\cdots((d_m)m + d_{m-1})(m-1) + \cdots + d_1)
// \cdot 1$, each step adding a digit and multiplying by a single limb.
fn from_factoradic_digits_desc_slice(digits: &[u64]) -> Option<Natural> {
    let mut x = Natural::ZERO;
    let len = u64::exact_from(digits.len());
    for (place, &digit) in (1..=len).rev().zip(digits) {
        if digit > place {
            return None;
        }
        if x == 0u32 && digit == 0 {
            continue;
        }
        x += Natural::from(digit);
        x.mul_assign_limb(Limb::exact_from(place));
    }
    Some(x)
}

impl FactoradicDigits for Natural {
    /// Returns a [`Vec`] containing the factorial-number-system digits of a [`Natural`] in
    /// ascending order: the digit at index $i$ is the coefficient of $(i+1)!$.
    ///
    /// If `self` is 0, the [`Vec`] is empty; otherwise, it ends with a nonzero digit.
    ///
    /// $f(x) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i \leq i + 1$ for all $i$, $k=0$ or $d_{k-1}
    /// \neq 0$, and
    ///
    /// $$
    /// \sum_{i=0}^{k-1}(i+1)! d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::FactoradicDigits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_factoradic_digits_asc().is_empty());
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .to_factoradic_digits_asc()
    ///         .to_debug_string(),
    ///     "[1, 2, 3]"
    /// );
    /// assert_eq!(
    ///     Natural::from_str("100000000000000000000")
    ///         .unwrap()
    ///         .to_factoradic_digits_asc()
    ///         .to_debug_string(),
    ///     "[0, 2, 2, 1, 5, 1, 1, 8, 8, 1, 8, 10, 10, 4, 5, 12, 3, 1, 2, 20, 1]"
    /// );
    /// ```
    #[inline]
    fn to_factoradic_digits_asc(&self) -> Vec<u64> {
        to_factoradic_digits_asc(self)
    }

    /// Returns a [`Vec`] containing the factorial-number-system digits of a [`Natural`] in
    /// descending order: the last digit is the coefficient of $1!$.
    ///
    /// If `self` is 0, the [`Vec`] is empty; otherwise, it begins with a nonzero digit.
    ///
    /// $f(x) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i \leq k - i$ for all $i$, $k=0$ or $d_0 \neq
    /// 0$, and
    ///
    /// $$
    /// \sum_{i=0}^{k-1}(k-i)! d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::FactoradicDigits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::ZERO.to_factoradic_digits_desc().is_empty());
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .to_factoradic_digits_desc()
    ///         .to_debug_string(),
    ///     "[3, 2, 1]"
    /// );
    /// assert_eq!(
    ///     Natural::from_str("100000000000000000000")
    ///         .unwrap()
    ///         .to_factoradic_digits_desc()
    ///         .to_debug_string(),
    ///     "[1, 20, 2, 1, 3, 12, 5, 4, 10, 10, 8, 1, 8, 8, 1, 1, 5, 1, 2, 2, 0]"
    /// );
    /// ```
    fn to_factoradic_digits_desc(&self) -> Vec<u64> {
        let mut digits = to_factoradic_digits_asc(self);
        digits.reverse();
        digits
    }

    /// Converts an iterator of factorial-number-system digits into a [`Natural`].
    ///
    /// The input digits are in ascending order: the digit at index $i$ is the coefficient of
    /// $(i+1)!$. The function returns `None` if any digit at index $i$ is greater than $i + 1$.
    /// Trailing zeros are allowed.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}(i+1)! d_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::FactoradicDigits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from_factoradic_digits_asc([1, 2, 3, 0, 0].iter().cloned()).to_debug_string(),
    ///     "Some(23)"
    /// );
    /// assert_eq!(
    ///     Natural::from_factoradic_digits_asc(
    ///         [0, 2, 2, 1, 5, 1, 1, 8, 8, 1, 8, 10, 10, 4, 5, 12, 3, 1, 2, 20, 1]
    ///             .iter()
    ///             .cloned()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(100000000000000000000)"
    /// );
    /// // the coefficient of 2! must be at most 2
    /// assert!(Natural::from_factoradic_digits_asc([1, 3].iter().cloned()).is_none());
    /// ```
    fn from_factoradic_digits_asc<I: Iterator<Item = u64>>(digits: I) -> Option<Self> {
        let mut digits: Vec<u64> = digits.collect();
        digits.reverse();
        from_factoradic_digits_desc_slice(&digits)
    }

    /// Converts an iterator of factorial-number-system digits into a [`Natural`].
    ///
    /// The input digits are in descending order: the last digit is the coefficient of $1!$. The
    /// function returns `None` if any digit is greater than its place, the number of digits from it
    /// to the end. Leading zeros are allowed.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}(k-i)! d_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::FactoradicDigits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from_factoradic_digits_desc([0, 0, 3, 2, 1].iter().cloned()).to_debug_string(),
    ///     "Some(23)"
    /// );
    /// assert_eq!(
    ///     Natural::from_factoradic_digits_desc(
    ///         [1, 20, 2, 1, 3, 12, 5, 4, 10, 10, 8, 1, 8, 8, 1, 1, 5, 1, 2, 2, 0]
    ///             .iter()
    ///             .cloned()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(100000000000000000000)"
    /// );
    /// // the coefficient of 2! must be at most 2
    /// assert!(Natural::from_factoradic_digits_desc([3, 1].iter().cloned()).is_none());
    /// ```
    #[inline]
    fn from_factoradic_digits_desc<I: Iterator<Item = u64>>(digits: I) -> Option<Self> {
        let digits: Vec<u64> = digits.collect();
        from_factoradic_digits_desc_slice(&digits)
    }
}
