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
use crate::polynomial::modular::traits::{CheckedModLcm, ModLcm, ModLcmAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::hgcd::{div_rem, mul};
use crate::unsigned_polynomial::modular::mod_gcd::{GcdModulus, assert_reduced, checked_monic_gcd};
use crate::unsigned_polynomial::modular::subproduct_tree::Modulus;
use alloc::vec::Vec;

/// The monic LCM of the polynomials with coefficients `xs` and `ys`, neither ending with a zero, or
/// `None` if a leading coefficient met along the way has no inverse, including the product of the
/// leading coefficients of `xs` and `ys`. The LCM of an empty polynomial and any other is empty.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute LCMs of
/// `NaturalPolynomial`s.
#[doc(hidden)]
pub fn checked_monic_lcm<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
    if xs.is_empty() || ys.is_empty() {
        return Some(Vec::new());
    }
    let inverse = m.inverse(&m.mul(xs.last().unwrap(), ys.last().unwrap()))?;
    let g = checked_monic_gcd(xs, ys, m)?;
    let mut l = mul(xs, &div_rem(ys, &g, m).unwrap().0, m);
    for c in &mut l {
        *c = m.mul(c, &inverse);
    }
    Some(l)
}

pub(crate) fn checked_mod_lcm_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, m);
    if m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    checked_monic_lcm(&p.coefficients, &q.coefficients, &Modulus(m))
        .map(UnsignedPolynomial::from_coefficients_asc)
}

fn mod_lcm_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    checked_mod_lcm_helper(p, q, m)
        .expect("a leading coefficient met by Euclid's algorithm is not invertible modulo m")
}

impl<T: PrimitiveUnsigned> ModLcm<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, taking both by value. The
    /// coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then this function panics. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_lcm(q, 7).to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: Self, m: T) -> Self {
        mod_lcm_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModLcm<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, taking the first by value
    /// and the second by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then this function panics. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(p.mod_lcm(&q, 7).to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: &Self, m: T) -> Self {
        mod_lcm_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModLcm<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, taking the first by
    /// reference and the second by value. The coefficients of both must already be reduced modulo
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then this function panics. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_lcm(q, 7).to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_lcm_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModLcm<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, taking both by reference.
    /// The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then this function panics. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!((&p).mod_lcm(&q, 7).to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_lcm_helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModLcmAssign<Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its monic LCM with another modulo $m$, taking the
    /// other polynomial by value. The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then this function panics. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// p.mod_lcm_assign(q, 7);
    /// assert_eq!(p.to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm_assign(&mut self, other: Self, m: T) {
        *self = mod_lcm_helper(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModLcmAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its monic LCM with another modulo $m$, taking the
    /// other polynomial by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then this function panics. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// p.mod_lcm_assign(&q, 7);
    /// assert_eq!(p.to_string(), "x^3+6*x^2+4*x+6");
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn mod_lcm_assign(&mut self, other: &Self, m: T) {
        *self = mod_lcm_helper(self, other, m);
    }
}

impl<T: PrimitiveUnsigned> CheckedModLcm<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both by value.
    /// The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then the result is `None`. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_lcm(q, 7).unwrap().to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap();
    /// assert!(p.checked_mod_lcm(q, 6).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: Self, m: T) -> Option<Self> {
        checked_mod_lcm_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModLcm<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first by
    /// value and the second by reference. The coefficients of both must already be reduced modulo
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then the result is `None`. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     p.checked_mod_lcm(&q, 7).unwrap().to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap();
    /// assert!(p.checked_mod_lcm(&q, 6).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: &Self, m: T) -> Option<Self> {
        checked_mod_lcm_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModLcm<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking the first by
    /// reference and the second by value. The coefficients of both must already be reduced modulo
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then the result is `None`. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_lcm(q, 7).unwrap().to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap();
    /// assert!((&p).checked_mod_lcm(q, 6).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: UnsignedPolynomial<T>, m: T) -> Option<UnsignedPolynomial<T>> {
        checked_mod_lcm_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModLcm<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the monic LCM of two [`UnsignedPolynomial`]s modulo $m$, returning `None` if a
    /// leading coefficient met along the way is not invertible modulo $m$, taking both by
    /// reference. The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found as by [`ModGcd`](crate::polynomial::modular::traits::ModGcd), and the
    /// result is the product of $a$ and $b/g$, made monic. When $m$ is composite, a leading
    /// coefficient met along the way, or the product of the leading coefficients of $a$ and $b$,
    /// may not be invertible modulo $m$; then the result is `None`. When $m$ is prime, this never
    /// happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModLcm;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+4*x+3").unwrap();
    /// assert_eq!(
    ///     (&p).checked_mod_lcm(&q, 7).unwrap().to_string(),
    ///     "x^3+6*x^2+4*x+6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap();
    /// assert!((&p).checked_mod_lcm(&q, 6).is_none());
    /// ```
    ///
    /// FLINT has no LCM modulo $m$.
    #[inline]
    fn checked_mod_lcm(self, other: &UnsignedPolynomial<T>, m: T) -> Option<UnsignedPolynomial<T>> {
        checked_mod_lcm_helper(self, other, m)
    }
}
