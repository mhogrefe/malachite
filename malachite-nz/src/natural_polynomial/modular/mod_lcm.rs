// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_gcd::{
    GcdOperation, OnePolynomial, checked_mod_gcd_operation,
};
use alloc::vec::Vec;
use malachite_base::polynomial::modular::traits::{CheckedModLcm, ModLcm, ModLcmAssign};
use malachite_base::unsigned_polynomial::modular::mod_gcd::GcdModulus;
use malachite_base::unsigned_polynomial::modular::mod_lcm::checked_monic_lcm;

pub(crate) struct LcmOp;

impl GcdOperation for LcmOp {
    type Shape = OnePolynomial;

    #[inline]
    fn run<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
        checked_monic_lcm(xs, ys, m)
    }
}

fn checked_mod_lcm_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    checked_mod_gcd_operation::<LcmOp>(p, q, m)
}

fn mod_lcm_helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    checked_mod_lcm_helper(p, q, m)
        .expect("a leading coefficient met by Euclid's algorithm is not invertible modulo m")
}

impl ModLcm<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// value and the modulus by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.mod_lcm(q, Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: Self, m: Natural) -> Self {
        mod_lcm_helper(&self, &other, &m)
    }
}

impl ModLcm<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// value and the modulus by reference. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.mod_lcm(q, &Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: Self, m: &Natural) -> Self {
        mod_lcm_helper(&self, &other, m)
    }
}

impl ModLcm<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by value, the second by reference, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.mod_lcm(&q, Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: &Self, m: Natural) -> Self {
        mod_lcm_helper(&self, other, &m)
    }
}

impl ModLcm<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by value, the second by reference, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.mod_lcm(&q, &Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: &Self, m: &Natural) -> Self {
        mod_lcm_helper(&self, other, m)
    }
}

impl ModLcm<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by reference, the second by value, and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_lcm(q, Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        mod_lcm_helper(self, &other, &m)
    }
}

impl ModLcm<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking the first polynomial
    /// by reference, the second by value, and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_lcm(q, &Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        mod_lcm_helper(self, &other, m)
    }
}

impl ModLcm<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// reference and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_lcm(&q, Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        mod_lcm_helper(self, other, &m)
    }
}

impl ModLcm<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, taking both polynomials by
    /// reference and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_lcm(&q, &Natural::from(7u32)).to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        mod_lcm_helper(self, other, m)
    }
}

impl ModLcmAssign<Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic LCM with another modulo $m$, taking the
    /// other polynomial by value and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// a \gets \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcmAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_lcm_assign(q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm_assign(&mut self, other: Self, m: Natural) {
        *self = mod_lcm_helper(self, &other, &m);
    }
}

impl ModLcmAssign<Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic LCM with another modulo $m$, taking the
    /// other polynomial by value and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcmAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_lcm_assign(q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm_assign(&mut self, other: Self, m: &Natural) {
        *self = mod_lcm_helper(self, &other, m);
    }
}

impl ModLcmAssign<&Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic LCM with another modulo $m$, taking the
    /// other polynomial by reference and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcmAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_lcm_assign(&q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm_assign(&mut self, other: &Self, m: Natural) {
        *self = mod_lcm_helper(self, other, &m);
    }
}

impl ModLcmAssign<&Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its monic LCM with another modulo $m$, taking the
    /// other polynomial by reference and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// a \gets \frac{ab}{cg},
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then this function panics. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way, or the product of the leading
    /// coefficients of `self` and `other`, is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModLcmAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// p.mod_lcm_assign(&q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm_assign(&mut self, other: &Self, m: &Natural) {
        *self = mod_lcm_helper(self, other, m);
    }
}

impl CheckedModLcm<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by value and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_lcm(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!(p.checked_mod_lcm(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: Self, m: Natural) -> Option<Self> {
        checked_mod_lcm_helper(&self, &other, &m)
    }
}

impl CheckedModLcm<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by value and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_lcm(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!(p.checked_mod_lcm(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: Self, m: &Natural) -> Option<Self> {
        checked_mod_lcm_helper(&self, &other, m)
    }
}

impl CheckedModLcm<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by value, the second by reference, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_lcm(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!(p.checked_mod_lcm(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: &Self, m: Natural) -> Option<Self> {
        checked_mod_lcm_helper(&self, other, &m)
    }
}

impl CheckedModLcm<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by value, the second by reference, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_lcm(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!(p.checked_mod_lcm(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: &Self, m: &Natural) -> Option<Self> {
        checked_mod_lcm_helper(&self, other, m)
    }
}

impl CheckedModLcm<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by reference, the second by value, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_lcm(q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!((&p).checked_mod_lcm(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: NaturalPolynomial, m: Natural) -> Option<NaturalPolynomial> {
        checked_mod_lcm_helper(self, &other, &m)
    }
}

impl CheckedModLcm<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first
    /// polynomial by reference, the second by value, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_lcm(q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!((&p).checked_mod_lcm(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
        checked_mod_lcm_helper(self, &other, m)
    }
}

impl CheckedModLcm<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by reference and the modulus by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_lcm(&q, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!((&p).checked_mod_lcm(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: &NaturalPolynomial, m: Natural) -> Option<NaturalPolynomial> {
        checked_mod_lcm_helper(self, other, &m)
    }
}

impl CheckedModLcm<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the monic LCM of two [`NaturalPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both polynomials
    /// by reference and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(\frac{ab}{cg}) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $g$ is the monic GCD of $a$ and $b$ and $c$ is the product of their leading
    /// coefficients, so that the result is monic; it is 0 if either is 0. Every common multiple of
    /// $a$ and $b$ is a multiple of it.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found as by [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd), and the result
    /// is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading coefficient
    /// met along the way, or the product of the leading coefficients of $a$ and $b$, may not be
    /// invertible modulo $m$; then the result is `None`. When $m$ is prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_lcm(&q, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x+1").unwrap();
    /// assert!((&p).checked_mod_lcm(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: &NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
        checked_mod_lcm_helper(self, other, m)
    }
}
