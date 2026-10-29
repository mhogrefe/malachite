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
use crate::natural_polynomial::arithmetic::mod_inverse_truncated::{
    NaturalInverseTruncatedOps, natural_inverse_truncated_newton, newton_cutoff,
};
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, ModPowerOf2Assign, ModPowerOf2Inverse, ModPowerOf2IsReduced,
    ModPowerOf2MulAssign, ModPowerOf2NegAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::polynomial::{ModPowerOf2InverseTruncated, Polynomial};
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;

fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// The first `n` coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// which are nonempty, modulo $2^k$, where $k$ is `pow`, without zeros at the end.
fn mul_truncated(xs: &[Natural], ys: &[Natural], n: usize, pow: u64) -> Vec<Natural> {
    mod_power_of_2_mul_truncated_ref_ref(xs, ys, u64::exact_from(n), pow).into_coefficients_asc()
}

// The first `n` coefficients of the inverse modulo $2^k$, where $k$ is `pow`, of the power series
// with coefficients `xs`, reduced, where `xs` is nonempty, `n` is positive, and `inverse` is the
// inverse of `xs[0]` modulo $2^k$: coefficient $i$ is $-x_0^{-1} \sum_{j=1}^{i} x_j y_{i-j}$, each
// sum computed exactly and reduced once.
//
// This is equivalent to `_gr_poly_inv_series_basecase_preinv1` from
// `gr_poly/inv_series_basecase.c`, FLINT 3.6.0, with coefficients modulo $2^k$, except that
// binomials are not special-cased.
crate_test_fn! {mod_power_of_2_inverse_truncated_basecase(
    xs: &[Natural],
    n: usize,
    pow: u64,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), n)];
    let mut out = Vec::with_capacity(n);
    out.push(inverse.clone());
    for i in 1..n {
        let l = min(i, xs.len() - 1);
        let mut sum = Natural::ZERO;
        for (x, y) in xs[1..=l].iter().zip(out[i - l..i].iter().rev()) {
            sum.add_mul_assign(x, y);
        }
        sum.mod_power_of_2_assign(pow);
        if *inverse != 1u32 {
            sum.mod_power_of_2_mul_assign(inverse, pow);
        }
        sum.mod_power_of_2_neg_assign(pow);
        out.push(sum);
    }
    out
}}

// The first `len` coefficients of the inverse modulo $2^k$, where $k$ is `pow`, of the power series
// with coefficients `xs`, reduced, where `xs` is nonempty, `len` is positive, and `inverse` is the
// inverse of `xs[0]` modulo $2^k$, by Newton's iteration (see `natural_inverse_truncated_newton`)
// down to lengths below `cutoff`.
crate_test_fn! {mod_power_of_2_inverse_truncated_newton(
    xs: &[Natural],
    len: usize,
    pow: u64,
    inverse: &Natural,
    cutoff: usize,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), len)];
    let ops = NaturalInverseTruncatedOps {
        basecase: |xs: &[Natural], n| {
            mod_power_of_2_inverse_truncated_basecase(xs, n, pow, inverse)
        },
        mul_truncated: |ys: &[Natural], zs: &[Natural], n| mul_truncated(ys, zs, n, pow),
        neg_assign: |y: &mut Natural| y.mod_power_of_2_neg_assign(pow),
    };
    natural_inverse_truncated_newton(xs, len, max(cutoff, 2), &ops)
}}

// For each $k$ of at least the first number, the length of the series from which inversion modulo
// $2^k$ uses Newton's iteration, measured as for `NEWTON_CUTOFFS`. Reduction modulo $2^k$ is cheap,
// so the truncated products of Newton's iteration overtake earlier than modulo other numbers. Only
// $k$ greater than two limbs' worth of bits gets here.
const NEWTON_POWER_OF_2_CUTOFFS: [(u64, usize); 10] = [
    (160, 14),
    (192, 20),
    (256, 23),
    (362, 46),
    (512, 18),
    (724, 13),
    (1024, 10),
    (2048, 14),
    (4096, 14),
    (8192, 16),
];

// The first `n` coefficients of the inverse modulo $2^k$, where $k$ is `pow`, of the power series
// with coefficients `xs`, reduced, where `xs` is nonempty, `n` is positive, and `inverse` is the
// inverse of `xs[0]` modulo $2^k$.
//
// This is equivalent to `_gr_fmpz_mod_poly_inv_series` from `gr/fmpz_mod.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that the cutoff was measured (see `NEWTON_POWER_OF_2_CUTOFFS`), and that
// short series are not always inverted coefficient by coefficient.
pub(crate) fn mod_power_of_2_inverse_truncated_dispatch(
    xs: &[Natural],
    n: usize,
    pow: u64,
    inverse: &Natural,
) -> Vec<Natural> {
    let xs = &xs[..min(xs.len(), n)];
    let cutoff = newton_cutoff(&NEWTON_POWER_OF_2_CUTOFFS, pow);
    if xs.len() < cutoff {
        mod_power_of_2_inverse_truncated_basecase(xs, n, pow, inverse)
    } else {
        mod_power_of_2_inverse_truncated_newton(xs, n, pow, inverse, cutoff)
    }
}

// The inverse modulo $2^k$, where $k$ is `pow`, of the power series with coefficients `xs`,
// reduced, truncated to `len` coefficients, with `kernel` doing the work in the general case, or
// `None` if the constant coefficient is even.
//
// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT 3.6.0,
// with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that the inverse
// modulo $x^0$, or modulo 1, is 0.
crate_test_fn! {mod_power_of_2_inverse_truncated_with(
    xs: &[Natural],
    len: u64,
    pow: u64,
    kernel: fn(&[Natural], usize, u64, &Natural) -> Vec<Natural>,
) -> Option<NaturalPolynomial> {
    if len == 0 || pow == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    let x0 = xs.first()?;
    if *x0 == 0u32 {
        return None;
    }
    let inverse = x0.mod_power_of_2_inverse(pow)?;
    if xs.len() == 1 {
        return Some(NaturalPolynomial::from(inverse));
    }
    let n = usize::saturating_from(len);
    let mut p = NaturalPolynomial {
        coefficients: kernel(xs, n, pow, &inverse),
    };
    p.trim();
    Some(p)
}}

// The inverse modulo $2^k$, where $k$ is `pow`, of the power series with coefficients `xs`,
// reduced, truncated to `len` coefficients, computed in wrapping arithmetic in `T`, which $2^k$
// must fit in.
fn mod_power_of_2_inverse_truncated_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial>
where
    Natural: From<T>,
{
    let kept = usize::saturating_from(len);
    let xs: Vec<T> = xs.iter().take(kept).map(T::exact_from).collect();
    Some(NaturalPolynomial {
        coefficients: mod_power_of_2_inverse_truncated_vec(&xs, len, pow)?
            .into_iter()
            .map(Natural::from)
            .collect(),
    })
}

// The inverse modulo $2^k$, where $k$ is `pow`, of the power series with coefficients `xs`,
// reduced, truncated to `len` coefficients. When $2^k$ fits in one or two limbs, this is done in
// wrapping word arithmetic.
fn mod_power_of_2_inverse_truncated_helper(
    xs: &[Natural],
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    if pow <= Limb::WIDTH {
        mod_power_of_2_inverse_truncated_word::<Limb>(xs, len, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_inverse_truncated_word::<DoubleLimb>(xs, len, pow)
    } else {
        mod_power_of_2_inverse_truncated_with(
            xs,
            len,
            pow,
            mod_power_of_2_inverse_truncated_dispatch,
        )
    }
}

impl ModPowerOf2InverseTruncated for NaturalPolynomial {
    type Output = Self;

    /// Inverts a [`NaturalPolynomial`] as a power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking it by value. Its coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = q, \quad \deg q < n, \quad pq \equiv 1 \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient is even; otherwise the inverse exists and is
    /// unique. Only the first `len` coefficients of the polynomial are read. When `len` is 0, or
    /// $k$ is 0, the ring has only one element and the result is `Some` of the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, the inverse is computed in wrapping word arithmetic, as
    /// for an [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial);
    /// otherwise short series are inverted one coefficient at a time, and longer ones by Newton's
    /// iteration, in which each step doubles the number of correct coefficients with two truncated
    /// products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2InverseTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("9*x+1").unwrap())
    ///         .mod_power_of_2_inverse_truncated(7, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^6+7*x^5+x^4+7*x^3+x^2+7*x+1"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (NaturalPolynomial::from_str("2*x+4").unwrap())
    ///         .mod_power_of_2_inverse_truncated(3, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that
    /// the inverse modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_inverse_truncated(self, len: u64, pow: u64) -> Option<Self> {
        (&self).mod_power_of_2_inverse_truncated(len, pow)
    }
}

impl ModPowerOf2InverseTruncated for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Inverts a [`NaturalPolynomial`] as a power series modulo $2^k$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking it by reference. Its coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = q, \quad \deg q < n, \quad pq \equiv 1 \pmod{x^n, 2^k}.
    /// $$
    ///
    /// Returns `None` if the constant coefficient is even; otherwise the inverse exists and is
    /// unique. Only the first `len` coefficients of the polynomial are read. When `len` is 0, or
    /// $k$ is 0, the ring has only one element and the result is `Some` of the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, the inverse is computed in wrapping word arithmetic, as
    /// for an [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial);
    /// otherwise short series are inverted one coefficient at a time, and longer ones by Newton's
    /// iteration, in which each step doubles the number of correct coefficients with two truncated
    /// products.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2InverseTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("9*x+1").unwrap())
    ///         .mod_power_of_2_inverse_truncated(7, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^6+7*x^5+x^4+7*x^3+x^2+7*x+1"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (&NaturalPolynomial::from_str("2*x+4").unwrap())
    ///         .mod_power_of_2_inverse_truncated(3, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_inv_series` from `fmpz_mod_poly/inv_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `None` is returned where FLINT aborts, and that
    /// the inverse modulo $x^0$ or modulo 1 is 0.
    fn mod_power_of_2_inverse_truncated(self, len: u64, pow: u64) -> Option<NaturalPolynomial> {
        assert_reduced(self, pow);
        mod_power_of_2_inverse_truncated_helper(&self.coefficients, len, pow)
    }
}
