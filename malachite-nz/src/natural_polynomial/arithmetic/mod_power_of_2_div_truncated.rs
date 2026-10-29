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

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_div_truncated::{
    NaturalDivTruncatedOps, natural_div_truncated_newton,
};
use crate::natural_polynomial::arithmetic::mod_inverse_truncated::newton_cutoff;
use crate::natural_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, ModPowerOf2Assign, ModPowerOf2Inverse, ModPowerOf2IsReduced,
    ModPowerOf2MulAssign, ModPowerOf2Sub,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::polynomial::{
    CheckedModPowerOf2DivTruncated, ModPowerOf2DivTruncated, ModPowerOf2DivTruncatedAssign,
    Polynomial,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;

fn assert_reduced(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
}

// The first `n` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// which are nonempty, modulo $2^k$, where $k$ is `pow`, without zeros at the end.
fn mul_truncated(xs: &[Natural], ys: &[Natural], n: usize, pow: u64) -> Vec<Natural> {
    mod_power_of_2_mul_truncated_ref_ref(xs, ys, u64::exact_from(n), pow).into_coefficients_asc()
}

// For each $k$ of at least the first number, the length of the divisor from which division modulo
// $2^k$ uses the method of Karp and Markstein, measured as for `NEWTON_CUTOFFS` in
// `mod_div_truncated`. Only $k$ greater than two limbs' worth of bits gets here.
const NEWTON_POWER_OF_2_CUTOFFS: [(u64, usize); 16] = [
    (129, 38),
    (181, 264),
    (256, 38),
    (362, 188),
    (512, 30),
    (724, 19),
    (1024, 8),
    (1448, 21),
    (2048, 34),
    (2896, 30),
    (4096, 24),
    (5793, 21),
    (8192, 17),
    (11585, 13),
    (16384, 12),
    (23170, 9),
];

// The first `n` coefficients of the quotient modulo $2^k$, where $k$ is `pow`, of the power series
// with coefficients `xs` and `ys`, reduced, where `xs` and `ys` are nonempty, `n` is positive, and
// `inverse` is the inverse of `ys[0]` modulo $2^k$: coefficient $i$ is $y_0^{-1} (x_i -
// \sum_{j=1}^{i} y_j q_{i-j})$, each sum computed exactly and reduced once.
//
// This is equivalent to `_gr_poly_div_series_basecase_preinv1` from
// `gr_poly/div_series_basecase.c`, FLINT 3.6.0, with coefficients modulo $2^k$.
crate_test_fn! {mod_power_of_2_div_truncated_basecase(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    pow: u64,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    let mut out: Vec<Natural> = Vec::with_capacity(n);
    for i in 0..n {
        let l = min(i, ys.len() - 1);
        let mut sum = Natural::ZERO;
        for (y, q) in ys[1..=l].iter().zip(out[i - l..i].iter().rev()) {
            sum.add_mul_assign(y, q);
        }
        sum.mod_power_of_2_assign(pow);
        let mut s = xs.get(i).unwrap_or(&Natural::ZERO).mod_power_of_2_sub(sum, pow);
        if *inverse != 1u32 {
            s.mod_power_of_2_mul_assign(inverse, pow);
        }
        out.push(s);
    }
    out
}}

// The first `len` coefficients of the quotient modulo $2^k$, where $k$ is `pow`, of the power
// series with coefficients `xs` and `ys`, reduced, where `xs` is nonempty, `len` is positive, and
// `inverse` is the inverse of `ys[0]` modulo $2^k$, by the method of Karp and Markstein (see
// `natural_div_truncated_newton`).
crate_test_fn! {mod_power_of_2_div_truncated_newton(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    pow: u64,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), len)];
    let ys = &ys[..min(ys.len(), len)];
    if ys.len() == 1 || len < 2 {
        return mod_power_of_2_div_truncated_basecase(xs, ys, len, pow, inverse);
    }
    let ops = NaturalDivTruncatedOps {
        inverse: |ys: &[Natural], n| {
            mod_power_of_2_inverse_truncated_dispatch(ys, n, pow, inverse)
        },
        mul_truncated: |ys: &[Natural], zs: &[Natural], n| mul_truncated(ys, zs, n, pow),
        sub: |x: &Natural, y: &Natural| x.mod_power_of_2_sub(y, pow),
    };
    natural_div_truncated_newton(xs, ys, len, &ops)
}}

// The first `n` coefficients of the quotient modulo $2^k$, where $k$ is `pow`, of the power series
// with coefficients `xs` and `ys`, reduced, where `xs` and `ys` are nonempty, `n` is positive, and
// `inverse` is the inverse of `ys[0]` modulo $2^k$.
//
// This is equivalent to `_gr_fmpz_mod_poly_div_series` from `gr/fmpz_mod.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that the cutoff was measured (see `NEWTON_POWER_OF_2_CUTOFFS`).
fn mod_power_of_2_div_truncated_dispatch(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    pow: u64,
    inverse: &Natural,
) -> Vec<Natural> {
    if min(ys.len(), n) < newton_cutoff(&NEWTON_POWER_OF_2_CUTOFFS, pow) {
        mod_power_of_2_div_truncated_basecase(xs, ys, n, pow, inverse)
    } else {
        mod_power_of_2_div_truncated_newton(xs, ys, n, pow, inverse)
    }
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the power series with coefficients `xs` and
// `ys`, reduced, truncated to `len` coefficients, with `kernel` doing the work in the general case,
// or `None` if the constant coefficient of `ys` is even.
//
// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT 3.6.0,
// with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that the quotient
// modulo $x^0$, or modulo 1, is 0.
crate_test_fn! {mod_power_of_2_div_truncated_with(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    pow: u64,
    kernel: fn(&[Natural], &[Natural], usize, u64, &Natural) -> Vec<Natural>,
) -> Option<NaturalPolynomial> {
    if len == 0 || pow == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    let y0 = ys.first()?;
    if *y0 == 0u32 {
        return None;
    }
    let inverse = y0.mod_power_of_2_inverse(pow)?;
    if xs.is_empty() {
        return Some(NaturalPolynomial::ZERO);
    }
    let n = usize::saturating_from(len);
    let mut p = NaturalPolynomial {
        coefficients: kernel(xs, ys, n, pow, &inverse),
    };
    p.trim();
    Some(p)
}}

// The quotient modulo $2^k$, where $k$ is `pow`, of the power series with coefficients `xs` and
// `ys`, reduced, truncated to `len` coefficients, computed in wrapping arithmetic in `T`, which
// $2^k$ must fit in.
fn mod_power_of_2_div_truncated_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    let kept = usize::saturating_from(len);
    let xs: Vec<T> = xs.iter().take(kept).map(T::exact_from).collect();
    let ys: Vec<T> = ys.iter().take(kept).map(T::exact_from).collect();
    Some(NaturalPolynomial {
        coefficients: mod_power_of_2_div_truncated_vec(&xs, &ys, len, pow)?
            .into_iter()
            .map(Natural::from)
            .collect(),
    })
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the power series with coefficients `xs` and
// `ys`, reduced, truncated to `len` coefficients, or `None` if the divisor is not invertible. When
// $2^k$ fits in one or two limbs, this is done in wrapping word arithmetic.
fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    assert_reduced(p, q, pow);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if pow <= Limb::WIDTH {
        mod_power_of_2_div_truncated_word::<Limb>(xs, ys, len, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_div_truncated_word::<DoubleLimb>(xs, ys, len, pow)
    } else {
        mod_power_of_2_div_truncated_with(xs, ys, len, pow, mod_power_of_2_div_truncated_dispatch)
    }
}

fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, len: u64, pow: u64) -> NaturalPolynomial {
    checked_helper(p, q, len, pow).expect("the constant coefficient of the divisor must be odd")
}

impl ModPowerOf2DivTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if the
    /// constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(self, other: Self, len: u64, pow: u64) -> Self {
        helper(&self, &other, len, pow)
    }
}

impl ModPowerOf2DivTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if the
    /// constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(self, other: &Self, len: u64, pow: u64) -> Self {
        helper(&self, other, len, pow)
    }
}

impl ModPowerOf2DivTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if the
    /// constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        helper(self, &other, len, pow)
    }
}

impl ModPowerOf2DivTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if the
    /// constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        helper(self, other, len, pow)
    }
}

impl ModPowerOf2DivTruncatedAssign<Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another as a power series modulo $2^k$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by value. The
    /// coefficients of both must already be reduced modulo $2^k$, and the constant coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if the
    /// constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_div_truncated_assign(
    ///     NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "5*x^4+7*x^3+3*x^2+5*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated_assign(&mut self, other: Self, len: u64, pow: u64) {
        *self = helper(self, &other, len, pow);
    }
}

impl ModPowerOf2DivTruncatedAssign<&Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another as a power series modulo $2^k$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by reference. The
    /// coefficients of both must already be reduced modulo $2^k$, and the constant coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $k$ is 0,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if the
    /// constant coefficient of `other` is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_div_truncated_assign(
    ///     &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "5*x^4+7*x^3+3*x^2+5*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that Malachite panics where FLINT aborts, and that the
    /// quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_div_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        *self = helper(self, other, len, pow);
    }
}

impl CheckedModPowerOf2DivTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that
    /// the quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn checked_mod_power_of_2_div_truncated(self, other: Self, len: u64, pow: u64) -> Option<Self> {
        checked_helper(&self, &other, len, pow)
    }
}

impl CheckedModPowerOf2DivTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that
    /// the quotient modulo $x^0$ or modulo 1 is 0.
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

impl CheckedModPowerOf2DivTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that
    /// the quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn checked_mod_power_of_2_div_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, len, pow)
    }
}

impl CheckedModPowerOf2DivTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $2^k$, keeping only the
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
    /// When $2^k$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             3,
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "5*x^4+7*x^3+3*x^2+5*x+6"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_power_of_2_div_truncated(
    ///             &NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             3,
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that
    /// the quotient modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn checked_mod_power_of_2_div_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, len, pow)
    }
}
