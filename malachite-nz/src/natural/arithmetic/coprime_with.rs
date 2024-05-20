// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::arithmetic::traits::{CoprimeWith, Gcd, Parity};

pub_test! {coprime_with_check_2(x: Natural, y: Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1u32
}}

#[cfg(feature = "test_build")]
pub fn coprime_with_check_2_3(x: Natural, y: Natural) -> bool {
    (x.odd() || y.odd())
        && (!(&x).divisible_by(Natural::from(3u32)) || !(&y).divisible_by(Natural::from(3u32)))
        && x.gcd(y) == 1u32
}

#[cfg(feature = "test_build")]
pub fn coprime_with_check_2_3_5(x: Natural, y: Natural) -> bool {
    if x.even() && y.even() {
        false
    } else {
        let x15 = &x % Natural::from(15u32);
        let y15 = &y % Natural::from(15u32);
        if (x15 == 0u32 || x15 == 3u32 || x15 == 6u32 || x15 == 9u32 || x15 == 12u32)
            && (y15 == 0u32 || y15 == 3u32 || y15 == 6u32 || y15 == 9u32 || y15 == 12u32)
        {
            return false;
        }
        if (x15 == 0u32 || x15 == 5u32 || x15 == 10u32)
            && (y15 == 0u32 || y15 == 5u32 || y15 == 10u32)
        {
            return false;
        }
        x.gcd(y) == 1u32
    }
}

pub_test! {coprime_with_check_2_val_ref(x: Natural, y: &Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1u32
}}

pub_test! {coprime_with_check_2_ref_val(x: &Natural, y: Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1u32
}}

pub_test! {coprime_with_check_2_ref_ref(x: &Natural, y: &Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1u32
}}

impl CoprimeWith<Natural> for Natural {
    /// Returns whether two [`Natural`]s are coprime; that is, whether they have no common factor
    /// other than 1. Both [`Natural`]s are taken by value.
    ///
    /// Every [`Natural`] is coprime with 1. No [`Natural`] is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).coprime_with(Natural::from(5u32)), true);
    /// assert_eq!(
    ///     Natural::from(12u32).coprime_with(Natural::from(90u32)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn coprime_with(self, other: Natural) -> bool {
        coprime_with_check_2(self, other)
    }
}

impl<'a> CoprimeWith<&'a Natural> for Natural {
    /// Returns whether two [`Natural`]s are coprime; that is, whether they have no common factor
    /// other than 1. The first [`Natural`] is taken by value and the second by reference.
    ///
    /// Every [`Natural`] is coprime with 1. No [`Natural`] is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).coprime_with(&Natural::from(5u32)), true);
    /// assert_eq!(
    ///     Natural::from(12u32).coprime_with(&Natural::from(90u32)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn coprime_with(self, other: &'a Natural) -> bool {
        coprime_with_check_2_val_ref(self, other)
    }
}

impl<'a> CoprimeWith<Natural> for &'a Natural {
    /// Returns whether two [`Natural`]s are coprime; that is, whether they have no common factor
    /// other than 1. The first [`Natural`] is taken by reference and the second by value.
    ///
    /// Every [`Natural`] is coprime with 1. No [`Natural`] is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).coprime_with(Natural::from(5u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from(12u32)).coprime_with(Natural::from(90u32)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn coprime_with(self, other: Natural) -> bool {
        coprime_with_check_2_ref_val(self, other)
    }
}

impl<'a, 'b> CoprimeWith<&'b Natural> for &'a Natural {
    /// Returns whether two [`Natural`]s are coprime; that is, whether they have no common factor
    /// other than 1. Both [`Natural`]s are taken by reference.
    ///
    /// Every [`Natural`] is coprime with 1. No [`Natural`] is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).coprime_with(Natural::from(5u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from(12u32)).coprime_with(Natural::from(90u32)),
    ///     false
    /// );
    /// ```
    fn coprime_with(self, other: &'b Natural) -> bool {
        coprime_with_check_2_ref_ref(self, other)
    }
}
