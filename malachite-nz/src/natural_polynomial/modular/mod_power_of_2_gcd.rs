// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_gcd::{GcdOp, checked_mod_power_of_2_gcd_operation};
use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;

fn checked_mod_power_of_2_gcd_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<NaturalPolynomial> {
    checked_mod_power_of_2_gcd_operation::<GcdOp>(p, q, pow)
}

impl CheckedModPowerOf2Gcd<Self> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking both by value. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and
    /// every common divisor of $a$ and $b$ divides it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_gcd(q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_gcd(q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_gcd` from
    /// `nmod_poly/gcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_gcd(self, other: Self, pow: u64) -> Option<Self> {
        checked_mod_power_of_2_gcd_helper(&self, &other, pow)
    }
}

impl CheckedModPowerOf2Gcd<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking the first by value and the second by
    /// reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and
    /// every common divisor of $a$ and $b$ divides it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_gcd(&q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_gcd(&q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_gcd` from
    /// `nmod_poly/gcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_gcd(self, other: &Self, pow: u64) -> Option<Self> {
        checked_mod_power_of_2_gcd_helper(&self, other, pow)
    }
}

impl CheckedModPowerOf2Gcd<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking the first by reference and the second
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and
    /// every common divisor of $a$ and $b$ divides it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_gcd(q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_gcd(q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_gcd` from
    /// `nmod_poly/gcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_gcd(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_mod_power_of_2_gcd_helper(self, &other, pow)
    }
}

impl CheckedModPowerOf2Gcd<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking both by reference. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(g) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0. It divides both, and
    /// every common divisor of $a$ and $b$ divides it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_gcd(&q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_gcd(&q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_gcd` from
    /// `nmod_poly/gcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_gcd(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_mod_power_of_2_gcd_helper(self, other, pow)
    }
}
