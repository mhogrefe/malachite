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
use crate::natural_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_ref_ref;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, ModAssign, ModInverse, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign,
    ModNegAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{ModInverseTruncated, Polynomial};
use malachite_base::unsigned_polynomial::arithmetic::mod_inverse_truncated::*;

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// The arithmetic that Newton's iteration for inverting a power series of `Natural`s is done in, as
// closures: `basecase(xs, n)` returns the first `n` coefficients of the inverse of `xs` directly,
// `mul_truncated(ys, zs, n)` the first `n` coefficients of the product of `ys` and `zs`, which are
// nonempty, without zeros at the end, and `neg_assign` negates.
pub(crate) struct NaturalInverseTruncatedOps<B, MT, N> {
    pub(crate) basecase: B,
    pub(crate) mul_truncated: MT,
    pub(crate) neg_assign: N,
}

// The first `n` coefficients of the inverse of the power series with coefficients `xs`, where `xs`
// is nonempty, `n` is positive, and `cutoff` is at least 2, by Newton's iteration: with $y$ the
// inverse to the first $m' = \lceil n/2 \rceil$ coefficients, the inverse to $n$ coefficients is $y
// - y(xy - 1)$, where $xy - 1$ is divisible by $x^{m'}$, so only its coefficients from $m'$ on are
// needed. They are taken from a truncated product, there being no middle product of `Natural`s.
// Lengths below `cutoff` are left to the base case of `ops`, which supplies the arithmetic.
//
// This is equivalent to `_gr_poly_inv_series_newton` from `gr_poly/inv_series_newton.c`, FLINT
// 3.6.0, without the middle product, except that the lengths are halved by recursion.
pub(crate) fn natural_inverse_truncated_newton<
    B: Fn(&[Natural], usize) -> Vec<Natural>,
    MT: Fn(&[Natural], &[Natural], usize) -> Vec<Natural>,
    N: Fn(&mut Natural),
>(
    xs: &[Natural],
    n: usize,
    cutoff: usize,
    ops: &NaturalInverseTruncatedOps<B, MT, N>,
) -> Vec<Natural> {
    if n < cutoff {
        return (ops.basecase)(xs, n);
    }
    let previous = n.div_ceil(2);
    let mut out = natural_inverse_truncated_newton(xs, previous, cutoff, ops);
    let q_len = min(xs.len(), n);
    let w_len = min(q_len + previous - 1, n);
    let product = (ops.mul_truncated)(&xs[..q_len], &out, w_len);
    out.resize(n, Natural::ZERO);
    if product.len() > previous {
        let high = (ops.mul_truncated)(&out[..previous], &product[previous..], n - previous);
        for (y, mut h) in out[previous..].iter_mut().zip(high) {
            (ops.neg_assign)(&mut h);
            *y = h;
        }
    }
    out
}

// The first `n` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// which are nonempty, modulo `m`, without zeros at the end.
fn mul_truncated(xs: &[Natural], ys: &[Natural], n: usize, m: &Natural) -> Vec<Natural> {
    mod_mul_truncated_ref_ref(xs, ys, u64::exact_from(n), m).into_coefficients_asc()
}

// The first `n` coefficients of the inverse modulo `m` of the power series with coefficients `xs`,
// reduced, where `xs` is nonempty, `n` is positive, and `inverse` is the inverse of `xs[0]` modulo
// `m`: coefficient $i$ is $-x_0^{-1} \sum_{j=1}^{i} x_j y_{i-j}$, each sum computed exactly and
// reduced once.
//
// This is equivalent to `_gr_poly_inv_series_basecase_preinv1` from
// `gr_poly/inv_series_basecase.c`, FLINT 3.6.0, with coefficients modulo `m`, except that binomials
// are not special-cased.
crate_test_fn! {mod_inverse_truncated_basecase(
    xs: &[Natural],
    n: usize,
    m: &Natural,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), n)];
    let data: ModMulData =
        <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    let mut out = Vec::with_capacity(n);
    out.push(inverse.clone());
    for i in 1..n {
        let l = min(i, xs.len() - 1);
        let mut sum = Natural::ZERO;
        for (x, y) in xs[1..=l].iter().zip(out[i - l..i].iter().rev()) {
            sum.add_mul_assign(x, y);
        }
        sum.mod_assign(m);
        if *inverse != 1u32 {
            sum.mod_mul_precomputed_assign(inverse, m, &data);
        }
        sum.mod_neg_assign(m);
        out.push(sum);
    }
    out
}}

// The first `len` coefficients of the inverse modulo `m` of the power series with coefficients
// `xs`, reduced, where `xs` is nonempty, `len` is positive, and `inverse` is the inverse of `xs[0]`
// modulo `m`, by Newton's iteration (see `natural_inverse_truncated_newton`) down to lengths below
// `cutoff`.
crate_test_fn! {mod_inverse_truncated_newton(
    xs: &[Natural],
    len: usize,
    m: &Natural,
    inverse: &Natural,
    cutoff: usize,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), len)];
    let ops = NaturalInverseTruncatedOps {
        basecase: |xs: &[Natural], n| mod_inverse_truncated_basecase(xs, n, m, inverse),
        mul_truncated: |ys: &[Natural], zs: &[Natural], n| mul_truncated(ys, zs, n, m),
        neg_assign: |y: &mut Natural| y.mod_neg_assign(m),
    };
    natural_inverse_truncated_newton(xs, len, max(cutoff, 2), &ops)
}}

// The length of the series from which inversion modulo a number of `bits` bits uses Newton's
// iteration, from `cutoffs`, pairs of a number of bits and a length sorted by the number of bits:
// the length paired with the greatest number of bits no greater than `bits`, or the first length if
// there is none.
pub(crate) fn newton_cutoff(cutoffs: &[(u64, usize)], bits: u64) -> usize {
    cutoffs
        .iter()
        .take_while(|&&(b, _)| b <= bits)
        .last()
        .map_or(cutoffs[0].1, |&(_, cutoff)| cutoff)
}

// For each modulus of at least the first number of bits, the length of the series from which
// inversion modulo it uses Newton's iteration. Measured on an Apple M-series machine, 2026-10, as
// the length at which a single step of Newton's iteration on a dense series with random
// coefficients overtakes inverting it coefficient by coefficient. Only moduli of more than two
// limbs get here. FLINT's `inv_series_cutoff_tab` in `gr/fmpz_mod.c` serves the same purpose; it
// has lower values for moduli of fewer than about 700 bits.
const NEWTON_CUTOFFS: [(u64, usize); 10] = [
    (160, 114),
    (192, 102),
    (256, 127),
    (362, 150),
    (512, 81),
    (724, 51),
    (1024, 41),
    (2048, 32),
    (4096, 26),
    (8192, 20),
];

// The first `n` coefficients of the inverse modulo `m` of the power series with coefficients `xs`,
// reduced, where `xs` is nonempty, `n` is positive, and `inverse` is the inverse of `xs[0]` modulo
// `m`.
//
// This is equivalent to `_gr_fmpz_mod_poly_inv_series` from `gr/fmpz_mod.c`, FLINT 3.6.0, except
// that the cutoff was measured (see `NEWTON_CUTOFFS`), and that short series are not always
// inverted coefficient by coefficient.
pub(crate) fn mod_inverse_truncated_dispatch(
    xs: &[Natural],
    n: usize,
    m: &Natural,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), n)];
    let cutoff = newton_cutoff(&NEWTON_CUTOFFS, m.significant_bits());
    if xs.len() < cutoff {
        mod_inverse_truncated_basecase(xs, n, m, inverse)
    } else {
        mod_inverse_truncated_newton(xs, n, m, inverse, cutoff)
    }
}

// The inverse modulo `m` of the power series with coefficients `xs`, reduced, truncated to `len`
// coefficients, with `kernel` doing the work in the general case, or `None` if the constant
// coefficient is not invertible.
//
// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT 3.6.0,
// except that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$, or modulo
// 1, is 0.
crate_test_fn! {mod_inverse_truncated_with(
    xs: &[Natural],
    len: u64,
    m: &Natural,
    kernel: fn(&[Natural], usize, &Natural, &Natural) -> Vec<Natural>,
) -> Option<NaturalPolynomial> {
    if len == 0 || *m == 1u32 {
        return Some(NaturalPolynomial::ZERO);
    }
    let x0 = xs.first()?;
    if *x0 == 0u32 {
        return None;
    }
    let inverse = x0.mod_inverse(m)?;
    if xs.len() == 1 {
        return Some(NaturalPolynomial::from(inverse));
    }
    let n = usize::saturating_from(len);
    let mut p = NaturalPolynomial {
        coefficients: kernel(xs, n, m, &inverse),
    };
    p.trim();
    Some(p)
}}

// The inverse modulo `m` of the power series with coefficients `xs`, reduced, truncated to `len`
// coefficients, computed in `T`, which `m` must fit in.
fn mod_inverse_truncated_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    len: u64,
    m: &Natural,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    let kept = usize::saturating_from(len);
    let xs: Vec<T> = xs.iter().take(kept).map(T::exact_from).collect();
    Some(NaturalPolynomial {
        coefficients: mod_inverse_truncated_vec(&xs, len, T::exact_from(m))?
            .into_iter()
            .map(Natural::from)
            .collect(),
    })
}

// The inverse modulo `m` of the power series with coefficients `xs`, reduced, truncated to `len`
// coefficients. When `m` fits in one or two limbs, this is done in word arithmetic.
fn mod_inverse_truncated_helper(
    xs: &[Natural],
    len: u64,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_inverse_truncated_word::<Limb>(xs, len, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_inverse_truncated_word::<DoubleLimb>(xs, len, m)
    } else {
        mod_inverse_truncated_with(xs, len, m, mod_inverse_truncated_dispatch)
    }
}

impl ModInverseTruncated<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Inverts a [`NaturalPolynomial`] as a power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the polynomial by value and the modulus by value.
    /// Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad pq \equiv 1 \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient is not invertible modulo $m$, which is when it is
    /// not coprime to $m$; otherwise the inverse exists and is unique. Only the first `len`
    /// coefficients of the polynomial are read. When `len` is 0, or $m$ is 1, the ring has only one
    /// element and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short series are inverted one coefficient at a time, and longer ones by Newton's iteration,
    /// in which each step doubles the number of correct coefficients with two truncated products.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_inverse_truncated(6, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+3*x^3+x+4"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("2*x+4").unwrap())
    ///         .mod_inverse_truncated(3, Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_inverse_truncated(self, len: u64, m: Natural) -> Option<Self> {
        (&self).mod_inverse_truncated(len, &m)
    }
}

impl ModInverseTruncated<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Inverts a [`NaturalPolynomial`] as a power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the polynomial by value and the modulus by
    /// reference. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad pq \equiv 1 \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient is not invertible modulo $m$, which is when it is
    /// not coprime to $m$; otherwise the inverse exists and is unique. Only the first `len`
    /// coefficients of the polynomial are read. When `len` is 0, or $m$ is 1, the ring has only one
    /// element and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short series are inverted one coefficient at a time, and longer ones by Newton's iteration,
    /// in which each step doubles the number of correct coefficients with two truncated products.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_inverse_truncated(6, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+3*x^3+x+4"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("2*x+4").unwrap())
    ///         .mod_inverse_truncated(3, &Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_inverse_truncated(self, len: u64, m: &Natural) -> Option<Self> {
        (&self).mod_inverse_truncated(len, m)
    }
}

impl ModInverseTruncated<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Inverts a [`NaturalPolynomial`] as a power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the polynomial by reference and the modulus by
    /// value. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad pq \equiv 1 \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient is not invertible modulo $m$, which is when it is
    /// not coprime to $m$; otherwise the inverse exists and is unique. Only the first `len`
    /// coefficients of the polynomial are read. When `len` is 0, or $m$ is 1, the ring has only one
    /// element and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short series are inverted one coefficient at a time, and longer ones by Newton's iteration,
    /// in which each step doubles the number of correct coefficients with two truncated products.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_inverse_truncated(6, Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+3*x^3+x+4"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("2*x+4").unwrap())
    ///         .mod_inverse_truncated(3, Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$
    /// or modulo 1 is 0.
    #[inline]
    fn mod_inverse_truncated(self, len: u64, m: Natural) -> Option<NaturalPolynomial> {
        self.mod_inverse_truncated(len, &m)
    }
}

impl ModInverseTruncated<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Inverts a [`NaturalPolynomial`] as a power series modulo $m$, keeping only the coefficients
    /// of $x^i$ for $i$ less than `len`, taking the polynomial by reference and the modulus by
    /// reference. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad pq \equiv 1 \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient is not invertible modulo $m$, which is when it is
    /// not coprime to $m$; otherwise the inverse exists and is unique. Only the first `len`
    /// coefficients of the polynomial are read. When `len` is 0, or $m$ is 1, the ring has only one
    /// element and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, the inverse is computed in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// short series are inverted one coefficient at a time, and longer ones by Newton's iteration,
    /// in which each step doubles the number of correct coefficients with two truncated products.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModInverseTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_inverse_truncated(6, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+3*x^3+x+4"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("2*x+4").unwrap())
    ///         .mod_inverse_truncated(3, &Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$
    /// or modulo 1 is 0.
    fn mod_inverse_truncated(self, len: u64, m: &Natural) -> Option<NaturalPolynomial> {
        assert_reduced(self, m);
        mod_inverse_truncated_helper(&self.coefficients, len, m)
    }
}
