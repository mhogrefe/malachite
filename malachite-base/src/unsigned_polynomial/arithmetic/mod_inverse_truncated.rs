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
use crate::polynomial::ModInverseTruncated;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_mul_middle::mod_mul_middle_to_out;
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, column_sum};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Sets `out` to the first `out.len()` coefficients of the inverse modulo `m` of the power series
// with coefficients `xs`, reduced, where `xs` and `out` are nonempty and `inverse` is the inverse
// of `xs[0]` modulo `m`: coefficient $i$ is $-x_0^{-1} \sum_{j=1}^{i} x_j y_{i-j}$.
//
// This is equivalent to `_nmod_poly_inv_series_basecase_preinv1` from `nmod_poly/inv_series.c`,
// FLINT 3.6.0.
fn mod_inverse_truncated_basecase_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    m: T,
    inverse: T,
) {
    let n = out.len();
    let xs = &xs[..min(xs.len(), n)];
    out[0] = inverse;
    out[1..].fill(T::ZERO);
    if xs.len() > 1 {
        let d = ModData::new(m, xs.len() - 1);
        let data = T::precompute_mod_mul_data(&m);
        for i in 1..n {
            let l = min(i, xs.len() - 1);
            let s = d.reduce_sum(column_sum(&xs[1..=l], &out[i - l..i], &d));
            out[i] = if inverse == T::ONE {
                s
            } else {
                s.mod_mul_precomputed(inverse, m, &data)
            }
            .mod_neg(m);
        }
    }
}

// The first `n` coefficients of the inverse modulo `m` of the power series with coefficients `xs`,
// reduced, where `xs` is nonempty and `n` is positive, and `inverse` is the inverse of `xs[0]`
// modulo `m`, coefficient by coefficient (see `mod_inverse_truncated_basecase_to_out`).
crate_test_fn! {mod_inverse_truncated_basecase<T: PrimitiveUnsigned>(
    xs: &[T],
    n: usize,
    m: T,
    inverse: T,
) -> Vec<T> {
    let mut out = vec![T::ZERO; n];
    mod_inverse_truncated_basecase_to_out(&mut out, xs, m, inverse);
    out
}}

// The lengths of the series from which inversion modulo $m$ uses Newton's iteration, indexed by the
// number of bits of $m$, minus 1. Measured on an Apple M-series machine, 2026-10, for `u64`, as the
// length at which a single step of Newton's iteration on a dense series overtakes inverting it
// coefficient by coefficient: about 400 for most moduli, 286 for 28- and 60-bit moduli, whose
// coefficient-by-coefficient sums need one more word, 633 for 62-bit ones, and 204 for 64-bit ones.
// Moduli of the other sizes take the value of the nearest size measured. FLINT's table
// `inv_series_cutoff_tab` in `gr/nmod.c` serves the same purpose.
const NEWTON_CUTOFFS: [usize; 64] = [
    402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402,
    402, 402, 402, 402, 402, 402, 402, 286, 286, 286, 286, 402, 402, 402, 402, 402, 402, 402, 402,
    402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402, 402,
    402, 286, 286, 286, 633, 633, 204,
];

// The length of the series from which inversion modulo a positive number of `bits` bits uses
// Newton's iteration (see `NEWTON_CUTOFFS`). For moduli of more than 64 bits, measured as above for
// `u128`: Newton's iteration barely breaks even below 1000 coefficients for 80-bit moduli, and
// overtakes from 505 for 128-bit ones.
pub(crate) fn newton_cutoff(bits: u64) -> usize {
    match bits {
        0..=64 => NEWTON_CUTOFFS[usize::saturating_from(bits) - 1],
        65..=96 => 1000,
        _ => 505,
    }
}

// The scratch length needed by `inverse_truncated_newton_to_out` for an inverse of `len`
// coefficients: up to `len / 2` coefficients of a middle product, and a window of up to `len - 1`
// coefficients of the series.
pub(crate) const fn inverse_truncated_newton_scratch_len(len: usize) -> usize {
    (len >> 1) + len.saturating_sub(1)
}

// The arithmetic that Newton's iteration for inverting a power series is done in, as closures:
// `basecase(out, xs)` sets `out` to the first `out.len()` coefficients of the inverse of `xs`
// directly, `mul_middle(out, ys, zs)` and `mul_truncated(out, ys, zs)` are the middle and truncated
// products of `mod_mul_middle_to_out` and `mod_mul_truncated_to_out`, and `neg_assign` negates.
pub(crate) struct InverseTruncatedOps<B, MM, MT, N> {
    pub(crate) basecase: B,
    pub(crate) mul_middle: MM,
    pub(crate) mul_truncated: MT,
    pub(crate) neg_assign: N,
}

// Sets `out` to the first `out.len()` coefficients of the inverse of the power series with
// coefficients `xs`, where `xs` and `out` are nonempty and `cutoff` is at least 2, by Newton's
// iteration: with $y$ the inverse to the first $m' = \lceil n/2 \rceil$ coefficients, the inverse
// to $n$ coefficients is $y - y(xy - 1)$, where $xy - 1$ is divisible by $x^{m'}$, so that only its
// coefficients from $m'$ on, a middle product, are needed. Lengths below `cutoff` are left to the
// base case of `ops`, which supplies the arithmetic. `scratch` must have length at least
// `inverse_truncated_newton_scratch_len(out.len())`.
//
// This is equivalent to `_gr_poly_inv_series_newton` from `gr_poly/inv_series_newton.c`, FLINT
// 3.6.0, using the middle product, except that the lengths are halved by recursion, and that the
// coefficients of `xs` that the middle product reads are padded with zeros only when `xs` is
// shorter than them.
pub(crate) fn inverse_truncated_newton_to_out<
    T: PrimitiveUnsigned,
    B: Fn(&mut [T], &[T]),
    MM: Fn(&mut [T], &[T], &[T]),
    MT: Fn(&mut [T], &[T], &[T]),
    N: Fn(&mut T),
>(
    out: &mut [T],
    xs: &[T],
    cutoff: usize,
    scratch: &mut [T],
    ops: &InverseTruncatedOps<B, MM, MT, N>,
) {
    let n = out.len();
    if n < cutoff {
        (ops.basecase)(out, xs);
        return;
    }
    let previous = n.div_ceil(2);
    inverse_truncated_newton_to_out(&mut out[..previous], xs, cutoff, scratch, ops);
    // The coefficients of xy from `previous` to `w_len` are the middle product of y and the
    // coefficients of x from 1 to `w_len`, which are 0 beyond the end of `xs`.
    let w_len = min(min(xs.len(), n) + previous - 1, n);
    let (w, scratch) = scratch.split_at_mut(n >> 1);
    let window = if xs.len() >= w_len {
        &xs[1..w_len]
    } else {
        let window = &mut scratch[..w_len - 1];
        let (copied, padding) = window.split_at_mut(xs.len() - 1);
        copied.copy_from_slice(&xs[1..]);
        padding.fill(T::ZERO);
        window
    };
    let w = &mut w[..w_len - previous];
    (ops.mul_middle)(w, &out[..previous], window);
    let (low, high) = out.split_at_mut(previous);
    (ops.mul_truncated)(high, low, w);
    for y in high {
        (ops.neg_assign)(y);
    }
}

// The first `len` coefficients of the inverse modulo `m` of the power series with coefficients
// `xs`, reduced, where `xs` is nonempty and `len` is positive, and `inverse` is the inverse of
// `xs[0]` modulo `m`, by Newton's iteration (see `inverse_truncated_newton_to_out`) down to lengths
// below `cutoff`.
//
// This is equivalent to `_gr_poly_inv_series_newton` from `gr_poly/inv_series_newton.c`, FLINT
// 3.6.0, with coefficients modulo `m`, except as described for `inverse_truncated_newton_to_out`.
crate_test_fn! {mod_inverse_truncated_newton<T: PrimitiveUnsigned>(
    xs: &[T],
    len: usize,
    m: T,
    inverse: T,
    cutoff: usize,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), len)];
    // The scratch follows the inverse in one allocation.
    let mut out = vec![T::ZERO; len + inverse_truncated_newton_scratch_len(len)];
    let (inverse_out, scratch) = out.split_at_mut(len);
    let ops = InverseTruncatedOps {
        basecase: |out: &mut [T], xs: &[T]| {
            mod_inverse_truncated_basecase_to_out(out, xs, m, inverse);
        },
        mul_middle: |out: &mut [T], ys: &[T], zs: &[T]| mod_mul_middle_to_out(out, ys, zs, m),
        mul_truncated: |out: &mut [T], ys: &[T], zs: &[T]| {
            mod_mul_truncated_to_out(out, ys, zs, m);
        },
        neg_assign: |y: &mut T| y.mod_neg_assign(m),
    };
    inverse_truncated_newton_to_out(inverse_out, xs, max(cutoff, 2), scratch, &ops);
    out.truncate(len);
    out
}}

// The first `n` coefficients of the inverse modulo `m` of the power series with coefficients `xs`,
// reduced, where `xs` is nonempty and `n` is positive, and `inverse` is the inverse of `xs[0]`
// modulo `m`.
//
// This is equivalent to `_nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0, with
// `_gr_nmod_poly_inv_series` from `gr/nmod.c` inlined.
fn mod_inverse_truncated_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    n: usize,
    m: T,
    inverse: T,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), n)];
    if xs.len() <= 10 {
        return mod_inverse_truncated_basecase(xs, n, m, inverse);
    }
    let cutoff = newton_cutoff(m.significant_bits());
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
// This is equivalent to `nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0, except
// that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$, or modulo 1, is 0.
crate_test_fn! {mod_inverse_truncated_with<T: PrimitiveUnsigned>(
    xs: &[T],
    len: u64,
    m: T,
    kernel: fn(&[T], usize, T, T) -> Vec<T>,
) -> Option<UnsignedPolynomial<T>> {
    if len == 0 || m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    let x0 = *xs.first()?;
    if x0 == T::ZERO {
        return None;
    }
    let inverse = x0.mod_inverse(m)?;
    if xs.len() == 1 {
        return Some(UnsignedPolynomial {
            coefficients: vec![inverse],
        });
    }
    let n = usize::saturating_from(len);
    Some(from_coefficients_trimmed(kernel(xs, n, m, inverse)))
}}

impl<T: PrimitiveUnsigned> ModInverseTruncated<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Inverts an [`UnsignedPolynomial`] as a power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking it by value. Its coefficients must
    /// already be reduced modulo $m$.
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
    /// Short series are inverted one coefficient at a time, and longer ones by Newton's iteration,
    /// in which each step doubles the number of correct coefficients with a middle product and a
    /// truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModInverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_inverse_truncated(6, 7)
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+3*x^3+x+4"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("2*x+4").unwrap())
    ///         .mod_inverse_truncated(3, 6)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$ or
    /// modulo 1 is 0.
    #[inline]
    fn mod_inverse_truncated(self, len: u64, m: T) -> Option<Self> {
        (&self).mod_inverse_truncated(len, m)
    }
}

impl<T: PrimitiveUnsigned> ModInverseTruncated<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Inverts an [`UnsignedPolynomial`] as a power series modulo $m$, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking it by reference. Its coefficients must
    /// already be reduced modulo $m$.
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
    /// Short series are inverted one coefficient at a time, and longer ones by Newton's iteration,
    /// in which each step doubles the number of correct coefficients with a middle product and a
    /// truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModInverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_inverse_truncated(6, 7)
    ///         .unwrap()
    ///         .to_string(),
    ///     "6*x^4+3*x^3+x+4"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x+4").unwrap())
    ///         .mod_inverse_truncated(3, 6)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts, and that the inverse modulo $x^0$ or
    /// modulo 1 is 0.
    fn mod_inverse_truncated(self, len: u64, m: T) -> Option<UnsignedPolynomial<T>> {
        assert_reduced(self, m);
        mod_inverse_truncated_with(&self.coefficients, len, m, mod_inverse_truncated_dispatch)
    }
}
