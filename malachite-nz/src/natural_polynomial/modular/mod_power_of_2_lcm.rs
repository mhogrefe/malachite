// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_gcd::checked_mod_power_of_2_gcd_operation;
use crate::natural_polynomial::modular::mod_lcm::LcmOp;
use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;

fn checked_mod_power_of_2_lcm_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<NaturalPolynomial> {
    checked_mod_power_of_2_gcd_operation::<LcmOp>(p, q, pow)
}

impl CheckedModPowerOf2Lcm<Self> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by `checked_mod_power_of_2_gcd`, and the result is the product of $a$ and $b/g$,
    /// made monic. A leading coefficient is invertible modulo $2^k$ exactly when it is odd; when
    /// one met along the way is even, including the product of the leading coefficients of $a$ and
    /// $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_lcm(q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_lcm(q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(self, other: Self, pow: u64) -> Option<Self> {
        checked_mod_power_of_2_lcm_helper(&self, &other, pow)
    }
}

impl CheckedModPowerOf2Lcm<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by `checked_mod_power_of_2_gcd`, and the result is the product of $a$ and $b/g$,
    /// made monic. A leading coefficient is invertible modulo $2^k$ exactly when it is odd; when
    /// one met along the way is even, including the product of the leading coefficients of $a$ and
    /// $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_power_of_2_lcm(&q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_lcm(&q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(self, other: &Self, pow: u64) -> Option<Self> {
        checked_mod_power_of_2_lcm_helper(&self, other, pow)
    }
}

impl CheckedModPowerOf2Lcm<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by `checked_mod_power_of_2_gcd`, and the result is the product of $a$ and $b/g$,
    /// made monic. A leading coefficient is invertible modulo $2^k$ exactly when it is odd; when
    /// one met along the way is even, including the product of the leading coefficients of $a$ and
    /// $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_lcm(q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_lcm(q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_mod_power_of_2_lcm_helper(self, &other, pow)
    }
}

impl CheckedModPowerOf2Lcm<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $2^k$, returning `None` if a
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
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by `checked_mod_power_of_2_gcd`, and the result is the product of $a$ and $b/g$,
    /// made monic. A leading coefficient is invertible modulo $2^k$ exactly when it is odd; when
    /// one met along the way is even, including the product of the leading coefficients of $a$ and
    /// $b$, the result is `None`.
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
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Lcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_power_of_2_lcm(&q, 3).unwrap().to_string(),
    ///     "x^4+3*x^3+5*x^2+x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_lcm(&q, 3).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo a power of 2.
    #[inline]
    fn checked_mod_power_of_2_lcm(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_mod_power_of_2_lcm_helper(self, other, pow)
    }
}
