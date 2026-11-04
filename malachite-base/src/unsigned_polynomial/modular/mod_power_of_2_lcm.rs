// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::polynomial::modular::traits::CheckedModPowerOf2Lcm;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_lcm::checked_monic_lcm;
use crate::unsigned_polynomial::modular::mod_power_of_2_gcd::assert_reduced_mod_power_of_2;
use crate::unsigned_polynomial::modular::subproduct_tree::PowerOf2Modulus;

fn checked_mod_power_of_2_lcm_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced_mod_power_of_2(p, q, pow);
    if pow == 0 {
        return Some(UnsignedPolynomial::ZERO);
    }
    checked_monic_lcm(&p.coefficients, &q.coefficients, &PowerOf2Modulus(pow))
        .map(UnsignedPolynomial::from_coefficients_asc)
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Lcm<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking both by value. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The GCD is found as by
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd), and
    /// the result is the product of $a$ and $b/g$, made monic. A leading coefficient is invertible
    /// modulo $2^k$ exactly when it is odd; when one met along the way is even, including the
    /// product of the leading coefficients of $a$ and $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_lcm(q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_lcm(q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(self, other: Self, pow: u64) -> Option<Self> {
        checked_mod_power_of_2_lcm_helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Lcm<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking the first by value and the second by
    /// reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The GCD is found as by
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd), and
    /// the result is the product of $a$ and $b/g$, made monic. A leading coefficient is invertible
    /// modulo $2^k$ exactly when it is odd; when one met along the way is even, including the
    /// product of the leading coefficients of $a$ and $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_lcm(&q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_lcm(&q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(self, other: &Self, pow: u64) -> Option<Self> {
        checked_mod_power_of_2_lcm_helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Lcm<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking the first by reference and the second
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The GCD is found as by
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd), and
    /// the result is the product of $a$ and $b/g$, made monic. A leading coefficient is invertible
    /// modulo $2^k$ exactly when it is odd; when one met along the way is even, including the
    /// product of the leading coefficients of $a$ and $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_lcm(q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_lcm(q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_mod_power_of_2_lcm_helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2Lcm<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $2^k$, returning `None` if a
    /// leading coefficient met along the way is even, taking both by reference. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The GCD is found as by
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd), and
    /// the result is the product of $a$ and $b/g$, made monic. A leading coefficient is invertible
    /// modulo $2^k$ exactly when it is odd; when one met along the way is even, including the
    /// product of the leading coefficients of $a$ and $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_lcm(&q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_lcm(&q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_mod_power_of_2_lcm_helper(self, other, pow)
    }
}
