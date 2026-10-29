// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011, 2014, 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::SaturatingFrom;
use crate::polynomial::{
    CheckedModPowerOf2DivTruncated, ModPowerOf2DivTruncated, ModPowerOf2DivTruncatedAssign,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_middle::mul_middle_wrapping;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::mul_truncated_wrapping;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;

fn assert_reduced<T: PrimitiveUnsigned>(
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

// The first `n` coefficients of the quotient modulo $2^W$ of the power series with coefficients
// `xs` and `ys`, correct modulo $2^k$ but not reduced, where `ys` is nonempty, `n` is positive, and
// `inverse` is an inverse of `ys[0]` modulo $2^k$: coefficient $i$ is $y_0^{-1} (x_i -
// \sum_{j=1}^{i} y_j q_{i-j})$.
//
// This is equivalent to `_nmod_poly_div_series_basecase_preinv1` from `nmod_poly/div_series.c`,
// FLINT 3.6.0, in wrapping arithmetic.
crate_test_fn! {mod_power_of_2_div_truncated_basecase<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    inverse: T,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    let mut out = vec![T::ZERO; n];
    for i in 0..n {
        let l = min(i, ys.len() - 1);
        let mut s = xs.get(i).copied().unwrap_or(T::ZERO);
        for (&y, &q) in ys[1..=l].iter().zip(out[i - l..i].iter().rev()) {
            s.wrapping_sub_assign(y.wrapping_mul(q));
        }
        out[i] = s.wrapping_mul(inverse);
    }
    out
}}

// The first `len` coefficients of the quotient of the power series with coefficients `xs` and `ys`,
// correct modulo $2^k$, where $k$ is `pow`, but not reduced, where `xs` is nonempty, `ys` has at
// least 2 coefficients, `len` is at least 2, and `inverse` is an inverse of `ys[0]` modulo $2^k$,
// by the method of Karp and Markstein (see `mod_div_truncated_newton`) in wrapping arithmetic.
//
// This is equivalent to `_gr_poly_div_series_newton` from `gr_poly/div_series_newton.c`, FLINT
// 3.6.0, in wrapping arithmetic, using the middle product, except that the inverse is computed by
// `mod_power_of_2_inverse_truncated_dispatch`.
crate_test_fn! {mod_power_of_2_div_truncated_newton<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: usize,
    pow: u64,
    inverse: T,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), len)];
    let ys = &ys[..min(ys.len(), len)];
    if ys.len() == 1 || len < 2 {
        return mod_power_of_2_div_truncated_basecase(xs, ys, len, inverse);
    }
    let half = len.div_ceil(2);
    let y = mod_power_of_2_inverse_truncated_dispatch(ys, half, pow, inverse);
    let mut out = vec![T::ZERO; len];
    let (low, high) = out.split_at_mut(half);
    mul_truncated_wrapping(low, &y, xs, pow);
    // The coefficients of qt from `half` to `w_len` are the middle product of t and the
    // coefficients of q from 1 to `w_len`, which are 0 beyond the end of `ys`.
    let w_len = min(ys.len() + half - 1, len);
    let mut w = vec![T::ZERO; len - half];
    let window_storage;
    let window = if ys.len() >= w_len {
        &ys[1..w_len]
    } else {
        let mut window = vec![T::ZERO; w_len - 1];
        window[..ys.len() - 1].copy_from_slice(&ys[1..]);
        window_storage = window;
        &window_storage[..]
    };
    mul_middle_wrapping(&mut w[..w_len - half], low, window, pow);
    for (i, w) in w.iter_mut().enumerate() {
        *w = xs.get(half + i).copied().unwrap_or(T::ZERO).wrapping_sub(*w);
    }
    mul_truncated_wrapping(high, &y, &w, pow);
    out
}}

// The length of the divisor from which division modulo $2^k$ uses the method of Karp and Markstein.
// Measured on an Apple M-series machine, 2026-10, as the length from which the method overtakes
// dividing coefficient by coefficient for dense series with random coefficients: 268 for `u64`, for
// every exponent from 1 to 64, and 310 for `u128`, for exponents from 65 to 128.
const fn newton_cutoff<T: PrimitiveUnsigned>() -> usize {
    if T::WIDTH > 64 { 310 } else { 268 }
}

// The first `n` coefficients of the quotient of the power series with coefficients `xs` and `ys`,
// correct modulo $2^k$, where $k$ is `pow`, which is positive, but not reduced, where `ys` is
// nonempty, `n` is positive, and `inverse` is an inverse of `ys[0]` modulo $2^k$.
//
// This is equivalent to `_gr_nmod_poly_div_series` from `gr/nmod.c`, FLINT 3.6.0, with the modulus
// $2^k$, in wrapping arithmetic, except that the cutoff was measured (see `newton_cutoff`).
pub(crate) fn mod_power_of_2_div_truncated_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    pow: u64,
    inverse: T,
) -> Vec<T> {
    if min(ys.len(), n) < newton_cutoff::<T>() {
        mod_power_of_2_div_truncated_basecase(xs, ys, n, inverse)
    } else {
        mod_power_of_2_div_truncated_newton(xs, ys, n, pow, inverse)
    }
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the power series with coefficients `xs` and
// `ys`, reduced, truncated to `len` coefficients, with `kernel` doing the work in the general case,
// or `None` if the constant coefficient of `ys` is even.
//
// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that `None` is returned where FLINT aborts, and that the quotient modulo
// $x^0$, or modulo 1, is 0.
crate_test_fn! {mod_power_of_2_div_truncated_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    pow: u64,
    kernel: fn(&[T], &[T], usize, u64, T) -> Vec<T>,
) -> Option<UnsignedPolynomial<T>> {
    if len == 0 || pow == 0 {
        return Some(UnsignedPolynomial::ZERO);
    }
    let y0 = *ys.first()?;
    if y0 == T::ZERO {
        return None;
    }
    let inverse = y0.mod_power_of_2_inverse(pow)?;
    if xs.is_empty() {
        return Some(UnsignedPolynomial::ZERO);
    }
    let n = usize::saturating_from(len);
    let mut out = kernel(xs, ys, n, pow, inverse);
    for q in &mut out {
        q.mod_power_of_2_assign(pow);
    }
    Some(from_coefficients_trimmed(out))
}}

/// The coefficients of the quotient as power series, modulo $2^k$, of the polynomials with
/// coefficients `xs` and `ys`, keeping only the coefficients of $x^i$ for $i$ less than `len`,
/// trimmed, or `None` if the constant coefficient of `ys` is even. The coefficients must be reduced
/// modulo $2^k$, where $k$ is `pow`, which must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a power of 2.
#[doc(hidden)]
#[inline]
pub fn mod_power_of_2_div_truncated_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    pow: u64,
) -> Option<Vec<T>> {
    mod_power_of_2_div_truncated_with(xs, ys, len, pow, mod_power_of_2_div_truncated_dispatch)
        .map(|p| p.coefficients)
}

fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, pow);
    mod_power_of_2_div_truncated_with(
        &p.coefficients,
        &q.coefficients,
        len,
        pow,
        mod_power_of_2_div_truncated_dispatch,
    )
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    checked_helper(p, q, len, pow).expect("the constant coefficient of the divisor must be odd")
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivTruncated<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by value. The coefficients of both must already be reduced modulo $2^k$, and the constant
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if the constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(self, other: Self, len: u64, pow: u64) -> Self {
        helper(&self, &other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivTruncated<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by reference. The coefficients of both must already be reduced modulo $2^k$, and the
    /// constant coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if the constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(self, other: &Self, len: u64, pow: u64) -> Self {
        helper(&self, other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivTruncated<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by value. The coefficients of both must already be reduced modulo $2^k$, and the
    /// constant coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if the constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(
        self,
        other: UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        helper(self, &other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivTruncated<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $2^k$, and the
    /// constant coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if the constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        helper(self, other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivTruncatedAssign<Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another as a power series modulo $2^k$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by value.
    /// The coefficients of both must already be reduced modulo $2^k$, and the constant coefficient
    /// of the divisor must be odd.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if the constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_div_truncated_assign(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "5*x^4+7*x^3+3*x^2+5*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated_assign(&mut self, other: Self, len: u64, pow: u64) {
        *self = helper(self, &other, len, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivTruncatedAssign<&Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another as a power series modulo $2^k$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$, and the constant
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if the constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_div_truncated_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "5*x^4+7*x^3+3*x^2+5*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        *self = helper(self, other, len, pow);
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivTruncated<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not odd; otherwise the quotient
    /// exists and is unique. Only the first `len` coefficients of each polynomial are read. When
    /// `len` is 0, or $k$ is 0, the ring has only one element and the result is `Some` of the zero
    /// polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn checked_mod_power_of_2_div_truncated(self, other: Self, len: u64, pow: u64) -> Option<Self> {
        checked_helper(&self, &other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivTruncated<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not odd; otherwise the quotient
    /// exists and is unique. Only the first `len` coefficients of each polynomial are read. When
    /// `len` is 0, or $k$ is 0, the ring has only one element and the result is `Some` of the zero
    /// polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn checked_mod_power_of_2_div_truncated(
        self,
        other: &Self,
        len: u64,
        pow: u64,
    ) -> Option<Self> {
        checked_helper(&self, other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivTruncated<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not odd; otherwise the quotient
    /// exists and is unique. Only the first `len` coefficients of each polynomial are read. When
    /// `len` is 0, or $k$ is 0, the ring has only one element and the result is `Some` of the zero
    /// polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn checked_mod_power_of_2_div_truncated(
        self,
        other: UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, &other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivTruncated<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not odd; otherwise the quotient
    /// exists and is unique. Only the first `len` coefficients of each polynomial are read. When
    /// `len` is 0, or $k$ is 0, the ring has only one element and the result is `Some` of the zero
    /// polynomial.
    ///
    /// Short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with one middle product and two truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn checked_mod_power_of_2_div_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, other, len, pow)
    }
}
