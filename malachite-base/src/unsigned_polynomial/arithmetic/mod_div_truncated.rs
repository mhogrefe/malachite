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

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::SaturatingFrom;
use crate::polynomial::{CheckedModDivTruncated, ModDivTruncated, ModDivTruncatedAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_inverse_truncated::mod_inverse_truncated_dispatch;
use crate::unsigned_polynomial::arithmetic::mod_mul_middle::mod_mul_middle_to_out;
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, column_sum};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;

fn assert_reduced<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        q.mod_is_reduced(&m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
}

// The first `n` coefficients of the quotient modulo `m` of the power series with coefficients `xs`
// and `ys`, reduced, where `xs` and `ys` are nonempty, `n` is positive, and `inverse` is the
// inverse of `ys[0]` modulo `m`: coefficient $i$ is $y_0^{-1} (x_i - \sum_{j=1}^{i} y_j q_{i-j})$.
//
// This is equivalent to `_nmod_poly_div_series_basecase_preinv1` from `nmod_poly/div_series.c`,
// FLINT 3.6.0.
crate_test_fn! {mod_div_truncated_basecase<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    m: T,
    inverse: T,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    let data = T::precompute_mod_mul_data(&m);
    let mut out = vec![T::ZERO; n];
    if ys.len() == 1 {
        for (o, &x) in out.iter_mut().zip(xs) {
            *o = x.mod_mul_precomputed(inverse, m, &data);
        }
        return out;
    }
    let d = ModData::new(m, ys.len() - 1);
    out[0] = xs[0].mod_mul_precomputed(inverse, m, &data);
    for i in 1..n {
        let l = min(i, ys.len() - 1);
        let s = d.reduce_sum(column_sum(&ys[1..=l], &out[i - l..i], &d));
        let s = if let Some(&x) = xs.get(i) {
            x.mod_sub(s, m)
        } else {
            s.mod_neg(m)
        };
        out[i] = if inverse == T::ONE {
            s
        } else {
            s.mod_mul_precomputed(inverse, m, &data)
        };
    }
    out
}}

// The first `len` coefficients of the quotient modulo `m` of the power series with coefficients
// `xs` and `ys`, reduced, where `xs` is nonempty, `ys` has at least 2 coefficients, `len` is at
// least 2, and `inverse` is the inverse of `ys[0]` modulo `m`, by the method of Karp and Markstein:
// with $y$ the inverse of `ys` to $m' = \lceil n/2 \rceil$ coefficients, the quotient to $m'$
// coefficients is $t = yx \bmod x^{m'}$, and the rest is $y(x - qt)$, where $x - qt$ is divisible
// by $x^{m'}$, so that only the middle coefficients of $qt$ are needed.
//
// This is equivalent to `_gr_poly_div_series_newton` from `gr_poly/div_series_newton.c`, FLINT
// 3.6.0, with coefficients modulo `m`, using the middle product, except that the inverse is
// computed by `mod_inverse_truncated_dispatch`.
crate_test_fn! {mod_div_truncated_newton<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: usize,
    m: T,
    inverse: T,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), len)];
    let ys = &ys[..min(ys.len(), len)];
    if ys.len() == 1 || len < 2 {
        return mod_div_truncated_basecase(xs, ys, len, m, inverse);
    }
    let half = len.div_ceil(2);
    let y = mod_inverse_truncated_dispatch(ys, half, m, inverse);
    let mut out = vec![T::ZERO; len];
    let (low, high) = out.split_at_mut(half);
    mod_mul_truncated_to_out(low, &y, xs, m);
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
    mod_mul_middle_to_out(&mut w[..w_len - half], low, window, m);
    for (i, w) in w.iter_mut().enumerate() {
        *w = xs.get(half + i).copied().unwrap_or(T::ZERO).mod_sub(*w, m);
    }
    mod_mul_truncated_to_out(high, &y, &w, m);
    out
}}

// The lengths of the divisor from which division modulo $m$ uses the method of Karp and Markstein,
// indexed by the number of bits of $m$, minus 1. Measured on an Apple M-series machine, 2026-10,
// with $m = 2^b - 1$ (the entry for 1 bit copies the one for 2), as the length from which the
// method overtakes dividing coefficient by coefficient for dense series with random coefficients.
const NEWTON_CUTOFFS: [usize; 64] = [
    833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833, 833,
    833, 833, 833, 833, 833, 1171, 1645, 1045, 933, 933, 933, 933, 933, 1045, 933, 933, 933, 933,
    933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 933, 1311,
    1645, 1645, 1645, 1045, 1469, 1469, 1469, 1469, 1469,
];

// The length of the divisor from which division modulo a positive number of `bits` bits uses the
// method of Karp and Markstein (see `NEWTON_CUTOFFS`). For moduli of more than 64 bits, measured as
// above for `u128`: the method overtakes from 1210 coefficients for moduli of up to 87 bits, and
// from about 860 for wider ones.
pub(crate) fn newton_cutoff(bits: u64) -> usize {
    match bits {
        0..=64 => NEWTON_CUTOFFS[usize::saturating_from(bits) - 1],
        65..=87 => 1210,
        _ => 861,
    }
}

// The first `n` coefficients of the quotient modulo `m` of the power series with coefficients `xs`
// and `ys`, reduced, where `xs` and `ys` are nonempty, `n` is positive, and `inverse` is the
// inverse of `ys[0]` modulo `m`.
//
// This is equivalent to `_gr_nmod_poly_div_series` from `gr/nmod.c`, FLINT 3.6.0, except that the
// cutoff was measured (see `newton_cutoff`).
pub(crate) fn mod_div_truncated_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    m: T,
    inverse: T,
) -> Vec<T> {
    if min(ys.len(), n) < newton_cutoff(m.significant_bits()) {
        mod_div_truncated_basecase(xs, ys, n, m, inverse)
    } else {
        mod_div_truncated_newton(xs, ys, n, m, inverse)
    }
}

// The quotient modulo `m` of the power series with coefficients `xs` and `ys`, reduced, truncated
// to `len` coefficients, with `kernel` doing the work in the general case, or `None` if the
// constant coefficient of `ys` is not invertible.
//
// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0, except
// that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$, or modulo 1, is
// 0.
crate_test_fn! {mod_div_truncated_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    m: T,
    kernel: fn(&[T], &[T], usize, T, T) -> Vec<T>,
) -> Option<UnsignedPolynomial<T>> {
    if len == 0 || m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    let y0 = *ys.first()?;
    if y0 == T::ZERO {
        return None;
    }
    let inverse = y0.mod_inverse(m)?;
    if xs.is_empty() {
        return Some(UnsignedPolynomial::ZERO);
    }
    let n = usize::saturating_from(len);
    Some(from_coefficients_trimmed(kernel(xs, ys, n, m, inverse)))
}}

/// The coefficients of the quotient as power series, modulo `m`, of the polynomials with
/// coefficients `xs` and `ys`, keeping only the coefficients of $x^i$ for $i$ less than `len`,
/// trimmed, or `None` if the constant coefficient of `ys` is not invertible. The coefficients must
/// be reduced modulo `m`, and `m` must be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
#[inline]
pub fn mod_div_truncated_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    m: T,
) -> Option<Vec<T>> {
    mod_div_truncated_with(xs, ys, len, m, mod_div_truncated_dispatch).map(|p| p.coefficients)
}

fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    assert_reduced(p, q, m);
    mod_div_truncated_with(
        &p.coefficients,
        &q.coefficients,
        len,
        m,
        mod_div_truncated_dispatch,
    )
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    checked_helper(p, q, len, m)
        .expect("the constant coefficient of the divisor must be invertible modulo m")
}

impl<T: PrimitiveUnsigned> ModDivTruncated<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by value. The coefficients of both must already be reduced modulo $m$, and the constant
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(self, other: Self, len: u64, m: T) -> Self {
        helper(&self, &other, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivTruncated<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by reference. The coefficients of both must already be reduced modulo $m$, and the constant
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(self, other: &Self, len: u64, m: T) -> Self {
        helper(&self, other, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivTruncated<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by value. The coefficients of both must already be reduced modulo $m$, and the
    /// constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(
        self,
        other: UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        helper(self, &other, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivTruncated<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $m$, and the
    /// constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        helper(self, other, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivTruncatedAssign<Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another as a power series modulo $m$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by value. The
    /// coefficients of both must already be reduced modulo $m$, and the constant coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_div_truncated_assign(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     7,
    /// );
    /// assert_eq!(p.to_string(), "6*x^4+5*x^3+6*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn mod_div_truncated_assign(&mut self, other: Self, len: u64, m: T) {
        *self = helper(self, &other, len, m);
    }
}

impl<T: PrimitiveUnsigned> ModDivTruncatedAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another as a power series modulo $m$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the constant coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_div_truncated_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     7,
    /// );
    /// assert_eq!(p.to_string(), "6*x^4+5*x^3+6*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn mod_div_truncated_assign(&mut self, other: &Self, len: u64, m: T) {
        *self = helper(self, other, len, m);
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivTruncated<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not invertible modulo $m$,
    /// which is when it is not coprime to $m$; otherwise the quotient exists and is unique. Only
    /// the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1, the
    /// ring has only one element and the result is `Some` of the zero polynomial.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(UnsignedPolynomial::<u8>::from_str("x+2").unwrap(), 3, 6)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(self, other: Self, len: u64, m: T) -> Option<Self> {
        checked_helper(&self, &other, len, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivTruncated<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor
    /// by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not invertible modulo $m$,
    /// which is when it is not coprime to $m$; otherwise the quotient exists and is unique. Only
    /// the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1, the
    /// ring has only one element and the result is `Some` of the zero polynomial.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x+2").unwrap(),
    ///             3,
    ///             6,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(self, other: &Self, len: u64, m: T) -> Option<Self> {
        checked_helper(&self, other, len, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivTruncated<UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not invertible modulo $m$,
    /// which is when it is not coprime to $m$; otherwise the quotient exists and is unique. Only
    /// the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1, the
    /// ring has only one element and the result is `Some` of the zero polynomial.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(UnsignedPolynomial::<u8>::from_str("x+2").unwrap(), 3, 6)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(
        self,
        other: UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, &other, len, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivTruncated<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s as power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the dividend by reference and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient of the divisor is not invertible modulo $m$,
    /// which is when it is not coprime to $m$; otherwise the quotient exists and is unique. Only
    /// the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1, the
    /// ring has only one element and the result is `Some` of the zero polynomial.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             7,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x+2").unwrap(),
    ///             3,
    ///             6,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_series` from `nmod_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> Option<UnsignedPolynomial<T>> {
        checked_helper(self, other, len, m)
    }
}
