// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Div, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul,
};
use malachite_base::num::basic::traits::Zero;

fn mod_power_of_2_div(x: &Natural, y: &Natural, pow: u64) -> Option<Natural> {
    assert!(
        x.mod_power_of_2_is_reduced(pow),
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    assert!(
        y.mod_power_of_2_is_reduced(pow),
        "y must be reduced mod 2^pow, but {y} >= 2^{pow}"
    );
    let Some(x_zeros) = x.trailing_zeros() else {
        return Some(Natural::ZERO);
    };
    // y is nonzero and less than 2^pow, so s < pow.
    let s = y.trailing_zeros()?;
    if x_zeros < s {
        return None;
    }
    let k = pow - s;
    Some((x >> s).mod_power_of_2_mul((y >> s).mod_power_of_2_inverse(k).unwrap(), k))
}

impl ModPowerOf2Div<Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`] modulo $2^k$, taking the first by value and the
    /// second by value, and returning `None` if no quotient exists. The inputs must be already
    /// reduced modulo $2^k$.
    ///
    /// Let $s$ be the number of trailing zeros of $y$. A quotient exists if and only if $x = 0$, or
    /// $y \neq 0$ and $2^s$ divides $x$. If $y$ is even, the quotient is not unique; all quotients
    /// differ by multiples of $2^{k-s}$, and this function returns the smallest,
    /// $(x/2^s)(y/2^s)^{-1} \bmod 2^{k-s}$.
    ///
    /// $f(x, y, k) = \operatorname{Some}(q)$, where $q$ is the least $q < 2^k$ with $qy \equiv x
    /// \mod 2^k$, if such a $q$ exists.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_power_of_2_div(Natural::from(3u32), 4),
    ///     Some(Natural::from(2u32))
    /// );
    /// assert_eq!(Natural::from(6u32).mod_power_of_2_div(Natural::from(4u32), 4), None);
    /// assert_eq!(
    ///     Natural::from(12u32).mod_power_of_2_div(Natural::from(4u32), 4),
    ///     Some(Natural::from(3u32))
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, other: Self, pow: u64) -> Option<Self> {
        mod_power_of_2_div(&self, &other, pow)
    }
}

impl ModPowerOf2Div<&Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`] modulo $2^k$, taking the first by value and the
    /// second by reference, and returning `None` if no quotient exists. The inputs must be already
    /// reduced modulo $2^k$.
    ///
    /// Let $s$ be the number of trailing zeros of $y$. A quotient exists if and only if $x = 0$, or
    /// $y \neq 0$ and $2^s$ divides $x$. If $y$ is even, the quotient is not unique; all quotients
    /// differ by multiples of $2^{k-s}$, and this function returns the smallest,
    /// $(x/2^s)(y/2^s)^{-1} \bmod 2^{k-s}$.
    ///
    /// $f(x, y, k) = \operatorname{Some}(q)$, where $q$ is the least $q < 2^k$ with $qy \equiv x
    /// \mod 2^k$, if such a $q$ exists.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_power_of_2_div(&Natural::from(3u32), 4),
    ///     Some(Natural::from(2u32))
    /// );
    /// assert_eq!(Natural::from(6u32).mod_power_of_2_div(&Natural::from(4u32), 4), None);
    /// assert_eq!(
    ///     Natural::from(12u32).mod_power_of_2_div(&Natural::from(4u32), 4),
    ///     Some(Natural::from(3u32))
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, other: &Self, pow: u64) -> Option<Self> {
        mod_power_of_2_div(&self, other, pow)
    }
}

impl ModPowerOf2Div<Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`] modulo $2^k$, taking the first by reference and
    /// the second by value, and returning `None` if no quotient exists. The inputs must be already
    /// reduced modulo $2^k$.
    ///
    /// Let $s$ be the number of trailing zeros of $y$. A quotient exists if and only if $x = 0$, or
    /// $y \neq 0$ and $2^s$ divides $x$. If $y$ is even, the quotient is not unique; all quotients
    /// differ by multiples of $2^{k-s}$, and this function returns the smallest,
    /// $(x/2^s)(y/2^s)^{-1} \bmod 2^{k-s}$.
    ///
    /// $f(x, y, k) = \operatorname{Some}(q)$, where $q$ is the least $q < 2^k$ with $qy \equiv x
    /// \mod 2^k$, if such a $q$ exists.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_power_of_2_div(Natural::from(3u32), 4),
    ///     Some(Natural::from(2u32))
    /// );
    /// assert_eq!((&Natural::from(6u32)).mod_power_of_2_div(Natural::from(4u32), 4), None);
    /// assert_eq!(
    ///     (&Natural::from(12u32)).mod_power_of_2_div(Natural::from(4u32), 4),
    ///     Some(Natural::from(3u32))
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, other: Natural, pow: u64) -> Option<Natural> {
        mod_power_of_2_div(self, &other, pow)
    }
}

impl ModPowerOf2Div<&Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`] modulo $2^k$, taking the first by reference and
    /// the second by reference, and returning `None` if no quotient exists. The inputs must be
    /// already reduced modulo $2^k$.
    ///
    /// Let $s$ be the number of trailing zeros of $y$. A quotient exists if and only if $x = 0$, or
    /// $y \neq 0$ and $2^s$ divides $x$. If $y$ is even, the quotient is not unique; all quotients
    /// differ by multiples of $2^{k-s}$, and this function returns the smallest,
    /// $(x/2^s)(y/2^s)^{-1} \bmod 2^{k-s}$.
    ///
    /// $f(x, y, k) = \operatorname{Some}(q)$, where $q$ is the least $q < 2^k$ with $qy \equiv x
    /// \mod 2^k$, if such a $q$ exists.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_power_of_2_div(&Natural::from(3u32), 4),
    ///     Some(Natural::from(2u32))
    /// );
    /// assert_eq!((&Natural::from(6u32)).mod_power_of_2_div(&Natural::from(4u32), 4), None);
    /// assert_eq!(
    ///     (&Natural::from(12u32)).mod_power_of_2_div(&Natural::from(4u32), 4),
    ///     Some(Natural::from(3u32))
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, other: &Natural, pow: u64) -> Option<Natural> {
        mod_power_of_2_div(self, other, pow)
    }
}
