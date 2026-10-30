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
use crate::polynomial::{
    CheckedModPowerOf2DivExact, ModPowerOf2DivExact, ModPowerOf2DivExactAssign,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::{
    assert_reduced, leading_inverse, mod_power_of_2_div_rem_with, mod_power_of_2_quotient_with,
    reduced,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use alloc::vec::Vec;

// The cutoffs below which exact division modulo $2^k$ computes the whole quotient from the top: the
// divisor's length, and the quotient's.
//
// These are the values of `_gr_nmod_poly_divexact` from `gr/nmod.c`, FLINT 3.6.0.
const BIDIRECTIONAL_DIVISOR_CUTOFF: usize = 40;
const BIDIRECTIONAL_QUOTIENT_CUTOFF: usize = 20;

// The quotient of the polynomials with coefficients `xs` and `ys`, correct modulo $2^k$, where $k$
// is `pow`, which is positive, but not reduced, assuming that the division is exact, where `ys` is
// nonempty and no longer than `xs`, `inverse` is an inverse of the last coefficient of `ys` modulo
// $2^k$, and `series` divides power series. The low zeros of `ys` are first discarded, along with
// as many coefficients of `xs`; then, if the lowest coefficient of `ys` is odd, the low half of the
// quotient is found by dividing the polynomials as power series, and the high half from the top, as
// in `mod_power_of_2_quotient_with`, by the high coefficients of `xs`. Otherwise the whole quotient
// is found from the top.
//
// This is equivalent to `_gr_poly_divexact_bidirectional` from `gr_poly/divexact_bidirectional.c`,
// FLINT 3.6.0, with the modulus $2^k$, in wrapping arithmetic, with `series` in place of
// `_gr_poly_div_series`, except that an even lowest coefficient is handled.
crate_test_fn! {mod_power_of_2_div_exact_bidirectional<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
    inverse: T,
    series: fn(&[T], &[T], usize, u64, T) -> Vec<T>,
) -> Vec<T> {
    let zeros = ys.iter().take_while(|&&y| y == T::ZERO).count();
    let (xs, ys) = (&xs[zeros..], &ys[zeros..]);
    let q_len = xs.len() - ys.len() + 1;
    if ys.len() == 1 || q_len == 1 {
        return mod_power_of_2_quotient_with(xs, ys, pow, inverse, series);
    }
    let Some(low_inverse) = ys[0].mod_power_of_2_inverse(pow) else {
        return mod_power_of_2_quotient_with(xs, ys, pow, inverse, series);
    };
    let low_len = q_len - (q_len >> 1);
    let mut qs = series(xs, ys, low_len, pow, low_inverse);
    qs.extend(mod_power_of_2_quotient_with(&xs[low_len..], ys, pow, inverse, series));
    qs
}}

// The quotient of the polynomials with coefficients `xs` and `ys`, correct modulo $2^k$, where $k$
// is `pow`, which is positive, but not reduced, assuming that the division is exact, where `ys` is
// nonempty and no longer than `xs`, and `inverse` is an inverse of the last coefficient of `ys`
// modulo $2^k$.
//
// This is equivalent to `_gr_nmod_poly_divexact` from `gr/nmod.c`, FLINT 3.6.0, with the modulus
// $2^k$, in wrapping arithmetic.
fn mod_power_of_2_div_exact_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
    inverse: T,
) -> Vec<T> {
    if ys.len() <= BIDIRECTIONAL_DIVISOR_CUTOFF
        || xs.len() - ys.len() < BIDIRECTIONAL_QUOTIENT_CUTOFF
    {
        mod_power_of_2_quotient_with(xs, ys, pow, inverse, mod_power_of_2_div_truncated_dispatch)
    } else {
        mod_power_of_2_div_exact_bidirectional(
            xs,
            ys,
            pow,
            inverse,
            mod_power_of_2_div_truncated_dispatch,
        )
    }
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, assuming that the division is exact, with `kernel` doing the work in the general
// case, or `None` if `ys` is zero or its leading coefficient is even. Modulo 1, every polynomial is
// 0, and so is the result.
//
// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that `None` is returned where FLINT aborts or assumes the leading
// coefficient invertible.
crate_test_fn! {mod_power_of_2_div_exact_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
    kernel: fn(&[T], &[T], u64, T) -> Vec<T>,
) -> Option<UnsignedPolynomial<T>> {
    if pow == 0 {
        return Some(UnsignedPolynomial::ZERO);
    }
    let inverse = leading_inverse(ys, pow)?;
    if xs.len() < ys.len() {
        return Some(UnsignedPolynomial::ZERO);
    }
    Some(reduced(kernel(xs, ys, pow, inverse), pow))
}}

/// The coefficients of the quotient modulo $2^k$ of the polynomials with coefficients `xs` and
/// `ys`, assuming that the division is exact, without zeros at the end, or `None` if `ys` is empty
/// or its last coefficient is even. The coefficients must be reduced modulo $2^k$, where $k$ is
/// `pow`, which must be no greater than `T::WIDTH`, and neither `xs` nor `ys` may end with a zero.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a power of 2.
#[doc(hidden)]
#[inline]
pub fn mod_power_of_2_div_exact_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
) -> Option<Vec<T>> {
    mod_power_of_2_div_exact_with(xs, ys, pow, mod_power_of_2_div_exact_dispatch)
        .map(|q| q.coefficients)
}

// Panics if `q` is zero, unless `pow` is 0, or if its leading coefficient is even.
pub(crate) fn assert_invertible<T: PrimitiveUnsigned>(q: &UnsignedPolynomial<T>, pow: u64) {
    if pow != 0 {
        assert_ne!(*q, UnsignedPolynomial::ZERO, "division by zero");
        assert!(
            leading_inverse(&q.coefficients, pow).is_some(),
            "the leading coefficient of the divisor must be odd"
        );
    }
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, q, pow);
    assert_invertible(q, pow);
    mod_power_of_2_div_exact_with(
        &p.coefficients,
        &q.coefficients,
        pow,
        mod_power_of_2_div_exact_dispatch,
    )
    .unwrap()
}

// The quotient modulo $2^k$, where $k$ is `pow`, of `p` by `q`, or `None` if `q` does not divide
// `p`.
//
// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that Malachite panics where FLINT assumes the leading coefficient
// invertible.
fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, pow);
    assert_invertible(q, pow);
    let (q, r) = mod_power_of_2_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        pow,
        mod_power_of_2_div_truncated_dispatch,
    )
    .unwrap();
    if r == UnsignedPolynomial::ZERO {
        Some(q)
    } else {
        None
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivExact<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, assuming that the second divides the
    /// first, taking the dividend by value and the divisor by value. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_div_exact(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, with
    /// the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(self, other: Self, pow: u64) -> Self {
        helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivExact<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, assuming that the second divides the
    /// first, taking the dividend by value and the divisor by reference. The coefficients of both
    /// must already be reduced modulo $2^k$, and the leading coefficient of the divisor must be
    /// odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_div_exact(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, with
    /// the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(self, other: &Self, pow: u64) -> Self {
        helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivExact<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, assuming that the second divides the
    /// first, taking the dividend by reference and the divisor by value. The coefficients of both
    /// must already be reduced modulo $2^k$, and the leading coefficient of the divisor must be
    /// odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_div_exact(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, with
    /// the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivExact<&UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, assuming that the second divides the
    /// first, taking the dividend by reference and the divisor by reference. The coefficients of
    /// both must already be reduced modulo $2^k$, and the leading coefficient of the divisor must
    /// be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_div_exact(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, with
    /// the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        helper(self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivExactAssign<Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, assuming that the second
    /// divides the first, taking the divisor by value. The coefficients of both must already be
    /// reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExactAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_div_exact_assign(q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, with
    /// the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact_assign(&mut self, other: Self, pow: u64) {
        *self = helper(self, &other, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivExactAssign<&Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, assuming that the second
    /// divides the first, taking the divisor by reference. The coefficients of both must already be
    /// reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// The low zeros of the divisor are discarded first. Long quotients are then found half from
    /// the bottom, by dividing the polynomials as power series, and half from the top, by dividing
    /// the reversed polynomials as power series; when the divisor's lowest nonzero coefficient is
    /// not odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExactAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_div_exact_assign(&q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divexact` from `nmod_poly/divexact.c`, FLINT 3.6.0, with
    /// the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact_assign(&mut self, other: &Self, pow: u64) {
        *self = helper(self, other, pow);
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivExact<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value and the divisor by value. The coefficients of
    /// both must already be reduced modulo $2^k$, and the leading coefficient of the divisor must
    /// be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](crate::polynomial::ModPowerOf2DivRem), and the quotient is returned if
    /// the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let s = p.checked_mod_power_of_2_div_exact(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_exact(q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(self, other: Self, pow: u64) -> Option<Self> {
        checked_helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivExact<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value and the divisor by reference. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](crate::polynomial::ModPowerOf2DivRem), and the quotient is returned if
    /// the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let s = p.checked_mod_power_of_2_div_exact(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_exact(&q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(self, other: &Self, pow: u64) -> Option<Self> {
        checked_helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivExact<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference and the divisor by value. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](crate::polynomial::ModPowerOf2DivRem), and the quotient is returned if
    /// the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let s = (&p).checked_mod_power_of_2_div_exact(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_exact(q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivExact<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference and the divisor by reference. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// The quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](crate::polynomial::ModPowerOf2DivRem), and the quotient is returned if
    /// the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+4").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let s = (&p).checked_mod_power_of_2_div_exact(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_exact(&q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divides` from `nmod_poly/divides.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, other, pow)
    }
}
