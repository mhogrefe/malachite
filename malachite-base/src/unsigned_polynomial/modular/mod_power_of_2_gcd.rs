// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::polynomial::Polynomial;
use crate::polynomial::modular::traits::CheckedModPowerOf2Gcd;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_gcd::checked_monic_gcd;
use crate::unsigned_polynomial::modular::subproduct_tree::PowerOf2Modulus;

pub(crate) fn assert_reduced_mod_power_of_2<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
}

pub(crate) fn checked_mod_power_of_2_gcd_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced_mod_power_of_2(p, q, pow);
    if pow == 0 {
        return Some(UnsignedPolynomial::ZERO);
    }
    checked_monic_gcd(&p.coefficients, &q.coefficients, &PowerOf2Modulus(pow))
        .map(UnsignedPolynomial::from_coefficients_asc)
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Gcd<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_gcd(q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
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

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Gcd<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_gcd(&q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
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

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Gcd<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_gcd(q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_gcd(q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_gcd` from
    /// `nmod_poly/gcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_gcd(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_mod_power_of_2_gcd_helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Gcd<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_gcd(&q, 3).unwrap().to_string(),
    ///     "x+1"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_gcd(&q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_gcd` from
    /// `nmod_poly/gcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_gcd(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_mod_power_of_2_gcd_helper(self, other, pow)
    }
}
