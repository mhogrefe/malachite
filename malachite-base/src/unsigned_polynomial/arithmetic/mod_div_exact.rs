// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{CheckedModDivExact, ModDivExact, ModDivExactAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem::{
    assert_reduced, leading_inverse, mod_div_rem_with, mod_quotient_with,
};
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use alloc::vec::Vec;

// The cutoffs below which exact division modulo `m` computes the whole quotient from the top: the
// divisor's length, and the quotient's.
//
// These are the values of `_gr_nmod_poly_divexact` from `gr/nmod.c`, FLINT 3.6.0.
const BIDIRECTIONAL_DIVISOR_CUTOFF: usize = 40;
const BIDIRECTIONAL_QUOTIENT_CUTOFF: usize = 20;

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, assuming
// that the division is exact, where `ys` is nonempty and no longer than `xs`, `inverse` is the
// inverse of the last coefficient of `ys` modulo `m`, and `series` divides power series. The low
// zeros of `ys` are first discarded, along with as many coefficients of `xs`; then, if the lowest
// coefficient of `ys` is invertible, the low half of the quotient is found by dividing the
// polynomials as power series, and the high half from the top, as in `mod_quotient_with`, by the
// high coefficients of `xs`. Otherwise the whole quotient is found from the top.
//
// This is equivalent to `_gr_poly_divexact_bidirectional` from `gr_poly/divexact_bidirectional.c`,
// FLINT 3.6.0, with coefficients modulo `m` and `series` in place of `_gr_poly_div_series`, except
// that a lowest coefficient that is not invertible is handled.
crate_test_fn! {mod_div_exact_bidirectional<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
    inverse: T,
    series: fn(&[T], &[T], usize, T, T) -> Vec<T>,
) -> Vec<T> {
    let zeros = ys.iter().take_while(|&&y| y == T::ZERO).count();
    let (xs, ys) = (&xs[zeros..], &ys[zeros..]);
    let q_len = xs.len() - ys.len() + 1;
    if ys.len() == 1 || q_len == 1 {
        return mod_quotient_with(xs, ys, m, inverse, series);
    }
    let Some(low_inverse) = ys[0].mod_inverse(m) else {
        return mod_quotient_with(xs, ys, m, inverse, series);
    };
    let low_len = q_len - (q_len >> 1);
    let mut qs = series(xs, ys, low_len, m, low_inverse);
    qs.extend(mod_quotient_with(&xs[low_len..], ys, m, inverse, series));
    qs
}}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, assuming
// that the division is exact, where `ys` is nonempty and no longer than `xs`, and `inverse` is the
// inverse of the last coefficient of `ys` modulo `m`.
//
// This is equivalent to `_gr_nmod_poly_divexact` from `gr/nmod.c`, FLINT 3.6.0.
fn mod_div_exact_dispatch<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T, inverse: T) -> Vec<T> {
    if ys.len() <= BIDIRECTIONAL_DIVISOR_CUTOFF
        || xs.len() - ys.len() < BIDIRECTIONAL_QUOTIENT_CUTOFF
    {
        mod_quotient_with(xs, ys, m, inverse, mod_div_truncated_dispatch)
    } else {
        mod_div_exact_bidirectional(xs, ys, m, inverse, mod_div_truncated_dispatch)
    }
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, assuming
// that the division is exact, with `kernel` doing the work in the general case, or `None` if `ys`
// is zero or its leading coefficient is not invertible. Modulo 1, every polynomial is 0, and so is
// the result.
//
// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, except that
// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
crate_test_fn! {mod_div_exact_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
    kernel: fn(&[T], &[T], T, T) -> Vec<T>,
) -> Option<UnsignedPolynomial<T>> {
    if m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    let inverse = leading_inverse(ys, m)?;
    if xs.len() < ys.len() {
        return Some(UnsignedPolynomial::ZERO);
    }
    Some(from_coefficients_trimmed(kernel(xs, ys, m, inverse)))
}}

/// The coefficients of the quotient modulo `m` of the polynomials with coefficients `xs` and `ys`,
/// assuming that the division is exact, without zeros at the end, or `None` if `ys` is empty or its
/// last coefficient is not invertible. The coefficients must be reduced modulo `m`, `m` must be
/// positive, and neither `xs` nor `ys` may end with a zero.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
#[inline]
pub fn mod_div_exact_vec<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Option<Vec<T>> {
    mod_div_exact_with(xs, ys, m, mod_div_exact_dispatch).map(|q| q.coefficients)
}

// Panics if `q` is zero, unless `m` is 1, or if its leading coefficient is not invertible modulo
// `m`.
pub(crate) fn assert_invertible<T: PrimitiveUnsigned>(q: &UnsignedPolynomial<T>, m: T) {
    if m != T::ONE {
        assert_ne!(*q, UnsignedPolynomial::ZERO, "division by zero");
        assert!(
            leading_inverse(&q.coefficients, m).is_some(),
            "the leading coefficient of the divisor must be invertible modulo m"
        );
    }
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, q, m);
    assert_invertible(q, m);
    mod_div_exact_with(&p.coefficients, &q.coefficients, m, mod_div_exact_dispatch).unwrap()
}

// The quotient modulo `m` of `p` by `q`, or `None` if `q` does not divide `p`.
//
// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, except that
// Malachite panics where FLINT assumes the leading coefficient invertible.
fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, m);
    assert_invertible(q, m);
    let (q, r) = mod_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        m,
        mod_div_truncated_dispatch,
    )
    .unwrap();
    if r == UnsignedPolynomial::ZERO {
        Some(q)
    } else {
        None
    }
}

impl<T: PrimitiveUnsigned> ModDivExact<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by value and the divisor by value. The coefficients of both must already
    /// be reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_div_exact(q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: Self, m: T) -> Self {
        helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivExact<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by value and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(p.mod_div_exact(&q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: &Self, m: T) -> Self {
        helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivExact<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by reference and the divisor by value. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_div_exact(q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivExact<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by reference and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!((&p).mod_div_exact(&q, 7).to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivExactAssign<Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, assuming that the second
    /// divides the first, taking the divisor by value. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExactAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// p.mod_div_exact_assign(q, 7);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact_assign(&mut self, other: Self, m: T) {
        *self = helper(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModDivExactAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, assuming that the second
    /// divides the first, taking the divisor by reference. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExactAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// p.mod_div_exact_assign(&q, 7);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact_assign(&mut self, other: &Self, m: T) {
        *self = helper(self, other, m);
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivExact<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value and the divisor by value. The coefficients of
    /// both must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by [`ModDivRem`](crate::polynomial::ModDivRem), and
    /// the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let s = p.checked_mod_div_exact(q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!(p.checked_mod_div_exact(q, 7).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_exact(self, other: Self, m: T) -> Option<Self> {
        checked_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivExact<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value and the divisor by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by [`ModDivRem`](crate::polynomial::ModDivRem), and
    /// the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let s = p.checked_mod_div_exact(&q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!(p.checked_mod_div_exact(&q, 7).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_exact(self, other: &Self, m: T) -> Option<Self> {
        checked_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivExact<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference and the divisor by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by [`ModDivRem`](crate::polynomial::ModDivRem), and
    /// the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let s = (&p).checked_mod_div_exact(q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!((&p).checked_mod_div_exact(q, 7).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_exact(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivExact<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference and the divisor by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by [`ModDivRem`](crate::polynomial::ModDivRem), and
    /// the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let s = (&p).checked_mod_div_exact(&q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// assert!((&p).checked_mod_div_exact(&q, 7).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_exact(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, other, m)
    }
}
