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
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_inverse_truncated::{
    mod_inverse_truncated_dispatch, newton_cutoff,
};
use crate::natural_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_ref_ref;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, ModAssign, ModInverse, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign,
    ModSub,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{
    CheckedModDivTruncated, ModDivTruncated, ModDivTruncatedAssign, Polynomial,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_div_truncated::*;

fn assert_reduced(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        q.mod_is_reduced(m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
}

// The arithmetic that the method of Karp and Markstein for dividing power series of `Natural`s is
// done in, as closures: `inverse(ys, n)` returns the first `n` coefficients of the inverse of `ys`,
// `mul_truncated(ys, zs, n)` the first `n` coefficients of the product of `ys` and `zs`, which are
// nonempty, without zeros at the end, and `sub(x, y)` the difference $x - y$.
pub(crate) struct NaturalDivTruncatedOps<I, MT, S> {
    pub(crate) inverse: I,
    pub(crate) mul_truncated: MT,
    pub(crate) sub: S,
}

// The first `len` coefficients of the quotient of the power series with coefficients `xs` and `ys`,
// where `xs` is nonempty, `ys` has at least 2 coefficients, and `len` is at least 2, by the method
// of Karp and Markstein: with $y$ the inverse of `ys` to $m' = \lceil n/2 \rceil$ coefficients, the
// quotient to $m'$ coefficients is $t = yx \bmod x^{m'}$, and the rest is $y(x - qt)$, where $x -
// qt$ is divisible by $x^{m'}$. The coefficients of $qt$ from $m'$ on are taken from a truncated
// product, there being no middle product of `Natural`s. `ops` supplies the arithmetic.
//
// This is equivalent to `_gr_poly_div_series_newton` from `gr_poly/div_series_newton.c`, FLINT
// 3.6.0, without the middle product, except that the inverse is computed by `ops`.
pub(crate) fn natural_div_truncated_newton<
    I: Fn(&[Natural], usize) -> Vec<Natural>,
    MT: Fn(&[Natural], &[Natural], usize) -> Vec<Natural>,
    S: Fn(&Natural, &Natural) -> Natural,
>(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    ops: &NaturalDivTruncatedOps<I, MT, S>,
) -> Vec<Natural> {
    let half = len.div_ceil(2);
    let y = (ops.inverse)(ys, half);
    let mut out = (ops.mul_truncated)(&y, xs, half);
    out.resize(len, Natural::ZERO);
    let w_len = min(ys.len() + half - 1, len);
    let product = if out[..half].iter().all(|t| *t == 0u32) {
        Vec::new()
    } else {
        (ops.mul_truncated)(ys, &out[..half], w_len)
    };
    let zero = Natural::ZERO;
    let w: Vec<Natural> = (half..len)
        .map(|i| (ops.sub)(xs.get(i).unwrap_or(&zero), product.get(i).unwrap_or(&zero)))
        .collect();
    if w.iter().any(|w| *w != 0u32) {
        for (o, h) in out[half..]
            .iter_mut()
            .zip((ops.mul_truncated)(&y, &w, len - half))
        {
            *o = h;
        }
    }
    out
}

// The first `n` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// which are nonempty, modulo `m`, without zeros at the end.
fn mul_truncated(xs: &[Natural], ys: &[Natural], n: usize, m: &Natural) -> Vec<Natural> {
    mod_mul_truncated_ref_ref(xs, ys, u64::exact_from(n), m).into_coefficients_asc()
}

// The first `n` coefficients of the quotient modulo `m` of the power series with coefficients `xs`
// and `ys`, reduced, where `xs` and `ys` are nonempty, `n` is positive, and `inverse` is the
// inverse of `ys[0]` modulo `m`: coefficient $i$ is $y_0^{-1} (x_i - \sum_{j=1}^{i} y_j q_{i-j})$,
// each sum computed exactly and reduced once.
//
// This is equivalent to `_gr_poly_div_series_basecase_preinv1` from
// `gr_poly/div_series_basecase.c`, FLINT 3.6.0, with coefficients modulo `m`.
crate_test_fn! {mod_div_truncated_basecase(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    m: &Natural,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    let data: ModMulData =
        <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    let mut out: Vec<Natural> = Vec::with_capacity(n);
    for i in 0..n {
        let l = min(i, ys.len() - 1);
        let mut sum = Natural::ZERO;
        for (y, q) in ys[1..=l].iter().zip(out[i - l..i].iter().rev()) {
            sum.add_mul_assign(y, q);
        }
        sum.mod_assign(m);
        let mut s = xs.get(i).unwrap_or(&Natural::ZERO).mod_sub(sum, m);
        if *inverse != 1u32 {
            s.mod_mul_precomputed_assign(inverse, m, &data);
        }
        out.push(s);
    }
    out
}}

// The first `len` coefficients of the quotient modulo `m` of the power series with coefficients
// `xs` and `ys`, reduced, where `xs` is nonempty, `len` is positive, and `inverse` is the inverse
// of `ys[0]` modulo `m`, by the method of Karp and Markstein (see `natural_div_truncated_newton`).
crate_test_fn! {mod_div_truncated_newton(
    xs: &[Natural],
    ys: &[Natural],
    len: usize,
    m: &Natural,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), len)];
    let ys = &ys[..min(ys.len(), len)];
    if ys.len() == 1 || len < 2 {
        return mod_div_truncated_basecase(xs, ys, len, m, inverse);
    }
    let ops = NaturalDivTruncatedOps {
        inverse: |ys: &[Natural], n| mod_inverse_truncated_dispatch(ys, n, m, inverse),
        mul_truncated: |ys: &[Natural], zs: &[Natural], n| mul_truncated(ys, zs, n, m),
        sub: |x: &Natural, y: &Natural| x.mod_sub(y, m),
    };
    natural_div_truncated_newton(xs, ys, len, &ops)
}}

// For each modulus of at least the first number of bits, the length of the divisor from which
// division modulo it uses the method of Karp and Markstein. Measured on an Apple M-series machine,
// 2026-10, with $m = 2^b - 1$, as the length from which the method overtakes dividing coefficient
// by coefficient for dense series with random coefficients. Only moduli of more than two limbs get
// here.
const NEWTON_CUTOFFS: [(u64, usize); 17] = [
    (129, 106),
    (181, 264),
    (256, 188),
    (362, 188),
    (512, 119),
    (724, 76),
    (1024, 54),
    (1448, 54),
    (2048, 48),
    (2896, 43),
    (4096, 34),
    (5793, 30),
    (8192, 27),
    (11585, 24),
    (16384, 21),
    (23170, 19),
    (32768, 17),
];

// The first `n` coefficients of the quotient modulo `m` of the power series with coefficients `xs`
// and `ys`, reduced, where `xs` and `ys` are nonempty, `n` is positive, and `inverse` is the
// inverse of `ys[0]` modulo `m`.
//
// This is equivalent to `_gr_fmpz_mod_poly_div_series` from `gr/fmpz_mod.c`, FLINT 3.6.0, except
// that the cutoff was measured (see `NEWTON_CUTOFFS`).
pub(crate) fn mod_div_truncated_dispatch(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    m: &Natural,
    inverse: &Natural,
) -> Vec<Natural> {
    if min(ys.len(), n) < newton_cutoff(&NEWTON_CUTOFFS, m.significant_bits()) {
        mod_div_truncated_basecase(xs, ys, n, m, inverse)
    } else {
        mod_div_truncated_newton(xs, ys, n, m, inverse)
    }
}

// The quotient modulo `m` of the power series with coefficients `xs` and `ys`, reduced, truncated
// to `len` coefficients, with `kernel` doing the work in the general case, or `None` if the
// constant coefficient of `ys` is not invertible.
//
// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT 3.6.0,
// except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$, or modulo
// 1, is 0.
crate_test_fn! {mod_div_truncated_with(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    m: &Natural,
    kernel: fn(&[Natural], &[Natural], usize, &Natural, &Natural) -> Vec<Natural>,
) -> Option<NaturalPolynomial> {
    if len == 0 || *m == 1u32 {
        return Some(NaturalPolynomial::ZERO);
    }
    let y0 = ys.first()?;
    if *y0 == 0u32 {
        return None;
    }
    let inverse = y0.mod_inverse(m)?;
    if xs.is_empty() {
        return Some(NaturalPolynomial::ZERO);
    }
    let n = usize::saturating_from(len);
    let mut p = NaturalPolynomial {
        coefficients: kernel(xs, ys, n, m, &inverse),
    };
    p.trim();
    Some(p)
}}

// The quotient modulo `m` of the power series with coefficients `xs` and `ys`, reduced, truncated
// to `len` coefficients, computed in `T`, which `m` must fit in.
fn mod_div_truncated_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    m: &Natural,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    let kept = usize::saturating_from(len);
    let xs: Vec<T> = xs.iter().take(kept).map(T::exact_from).collect();
    let ys: Vec<T> = ys.iter().take(kept).map(T::exact_from).collect();
    Some(NaturalPolynomial {
        coefficients: mod_div_truncated_vec(&xs, &ys, len, T::exact_from(m))?
            .into_iter()
            .map(Natural::from)
            .collect(),
    })
}

// The quotient modulo `m` of the power series with coefficients `xs` and `ys`, reduced, truncated
// to `len` coefficients, or `None` if the divisor is not invertible. When `m` fits in one or two
// limbs, this is done in word arithmetic.
fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    assert_reduced(p, q, m);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_div_truncated_word::<Limb>(xs, ys, len, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_div_truncated_word::<DoubleLimb>(xs, ys, len, m)
    } else {
        mod_div_truncated_with(xs, ys, len, m, mod_div_truncated_dispatch)
    }
}

fn helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    checked_helper(p, q, len, m)
        .expect("the constant coefficient of the divisor must be invertible modulo m")
}

impl ModDivTruncated<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by value, and
    /// the modulus by value. The coefficients of both must already be reduced modulo $m$, and the
    /// constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(self, other: Self, len: u64, m: Natural) -> Self {
        helper(&self, &other, len, &m)
    }
}

impl ModDivTruncated<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by value, and
    /// the modulus by reference. The coefficients of both must already be reduced modulo $m$, and
    /// the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(self, other: Self, len: u64, m: &Natural) -> Self {
        helper(&self, &other, len, m)
    }
}

impl ModDivTruncated<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by reference,
    /// and the modulus by value. The coefficients of both must already be reduced modulo $m$, and
    /// the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(self, other: &Self, len: u64, m: Natural) -> Self {
        helper(&self, other, len, &m)
    }
}

impl ModDivTruncated<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by reference,
    /// and the modulus by reference. The coefficients of both must already be reduced modulo $m$,
    /// and the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(self, other: &Self, len: u64, m: &Natural) -> Self {
        helper(&self, other, len, m)
    }
}

impl ModDivTruncated<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by value,
    /// and the modulus by value. The coefficients of both must already be reduced modulo $m$, and
    /// the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        helper(self, &other, len, &m)
    }
}

impl ModDivTruncated<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by value,
    /// and the modulus by reference. The coefficients of both must already be reduced modulo $m$,
    /// and the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        helper(self, &other, len, m)
    }
}

impl ModDivTruncated<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by
    /// reference, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        helper(self, other, len, &m)
    }
}

impl ModDivTruncated<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by
    /// reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        helper(self, other, len, m)
    }
}

impl ModDivTruncatedAssign<Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another as a power series modulo $m$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by value, and the
    /// modulus by value. The coefficients of both must already be reduced modulo $m$, and the
    /// constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_div_truncated_assign(
    ///     NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^4+5*x^3+6*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated_assign(&mut self, other: Self, len: u64, m: Natural) {
        *self = helper(self, &other, len, &m);
    }
}

impl ModDivTruncatedAssign<Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another as a power series modulo $m$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by value, and the
    /// modulus by reference. The coefficients of both must already be reduced modulo $m$, and the
    /// constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_div_truncated_assign(
    ///     NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^4+5*x^3+6*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated_assign(&mut self, other: Self, len: u64, m: &Natural) {
        *self = helper(self, &other, len, m);
    }
}

impl ModDivTruncatedAssign<&Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another as a power series modulo $m$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by reference, and
    /// the modulus by value. The coefficients of both must already be reduced modulo $m$, and the
    /// constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_div_truncated_assign(
    ///     &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^4+5*x^3+6*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated_assign(&mut self, other: &Self, len: u64, m: Natural) {
        *self = helper(self, other, len, &m);
    }
}

impl ModDivTruncatedAssign<&Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another as a power series modulo $m$ in place, keeping
    /// only the coefficients of $x^i$ for $i$ less than `len`, taking the divisor by reference, and
    /// the modulus by reference. The coefficients of both must already be reduced modulo $m$, and
    /// the constant coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. When `len` is 0, or $m$ is 1,
    /// the ring has only one element and the result is the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if the constant coefficient of `other` is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_div_truncated_assign(
    ///     &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///     5,
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "6*x^4+5*x^3+6*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that Malachite panics where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_div_truncated_assign(&mut self, other: &Self, len: u64, m: &Natural) {
        *self = helper(self, other, len, m);
    }
}

impl CheckedModDivTruncated<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by value, and
    /// the modulus by value. The coefficients of both must already be reduced modulo $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(self, other: Self, len: u64, m: Natural) -> Option<Self> {
        checked_helper(&self, &other, len, &m)
    }
}

impl CheckedModDivTruncated<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by value, and
    /// the modulus by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             &Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(self, other: Self, len: u64, m: &Natural) -> Option<Self> {
        checked_helper(&self, &other, len, m)
    }
}

impl CheckedModDivTruncated<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by reference,
    /// and the modulus by value. The coefficients of both must already be reduced modulo $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(self, other: &Self, len: u64, m: Natural) -> Option<Self> {
        checked_helper(&self, other, len, &m)
    }
}

impl CheckedModDivTruncated<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by value and the divisor by reference,
    /// and the modulus by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             &Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(self, other: &Self, len: u64, m: &Natural) -> Option<Self> {
        checked_helper(&self, other, len, m)
    }
}

impl CheckedModDivTruncated<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by value,
    /// and the modulus by value. The coefficients of both must already be reduced modulo $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, len, &m)
    }
}

impl CheckedModDivTruncated<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by value,
    /// and the modulus by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             &Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, len, m)
    }
}

impl CheckedModDivTruncated<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by
    /// reference, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, len, &m)
    }
}

impl CheckedModDivTruncated<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s as power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the dividend by reference and the divisor by
    /// reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$.
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
    /// When $m$ fits in one or two limbs, the quotient is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short divisors are handled one coefficient at a time, and longer ones by the method of Karp
    /// and Markstein: the divisor is inverted to half the length, and the quotient is completed
    /// with three truncated products.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x+3").unwrap(),
    ///             5,
    ///             &Natural::from(7u32),
    ///         )
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+5*x^3+6*x+3"
    /// );
    /// // The constant coefficient of the divisor is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("3*x+5").unwrap())
    ///         .checked_mod_div_truncated(
    ///             &NaturalPolynomial::from_str("x+2").unwrap(),
    ///             3,
    ///             &Natural::from(6u32),
    ///         )
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_series` from `fmpz_mod_poly/div_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the quotient modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn checked_mod_div_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, len, m)
    }
}
