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
use crate::polynomial::ModPowerOf2InverseTruncated;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_inverse_truncated::{
    InverseTruncatedOps, inverse_truncated_newton_scratch_len, inverse_truncated_newton_to_out,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_middle::mul_middle_wrapping;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::mul_truncated_wrapping;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Sets `out` to the first `out.len()` coefficients of the inverse modulo $2^W$ of the power series
// with coefficients `xs`, correct modulo $2^k$ but not reduced, where `xs` and `out` are nonempty
// and `inverse` is an inverse of `xs[0]` modulo $2^k$: coefficient $i$ is $-x_0^{-1} \sum_{j=1}^{i}
// x_j y_{i-j}$.
//
// This is equivalent to `_nmod_poly_inv_series_basecase_preinv1` from `nmod_poly/inv_series.c`,
// FLINT 3.6.0, in wrapping arithmetic.
fn mod_power_of_2_inverse_truncated_basecase_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    inverse: T,
) {
    let n = out.len();
    let xs = &xs[..min(xs.len(), n)];
    out[0] = inverse;
    for i in 1..n {
        let l = min(i, xs.len() - 1);
        let mut s = T::ZERO;
        for (&x, &y) in xs[1..=l].iter().zip(out[i - l..i].iter().rev()) {
            s.wrapping_add_assign(x.wrapping_mul(y));
        }
        out[i] = s.wrapping_mul(inverse).wrapping_neg();
    }
}

// The first `n` coefficients of the inverse of the power series with coefficients `xs`, correct
// modulo $2^k$ but not reduced, where `xs` is nonempty, `n` is positive, and `inverse` is an
// inverse of `xs[0]` modulo $2^k$, coefficient by coefficient (see
// `mod_power_of_2_inverse_truncated_basecase_to_out`).
crate_test_fn! {mod_power_of_2_inverse_truncated_basecase<T: PrimitiveUnsigned>(
    xs: &[T],
    n: usize,
    inverse: T,
) -> Vec<T> {
    let mut out = vec![T::ZERO; n];
    mod_power_of_2_inverse_truncated_basecase_to_out(&mut out, xs, inverse);
    out
}}

// The first `len` coefficients of the inverse of the power series with coefficients `xs`, correct
// modulo $2^k$, where $k$ is `pow`, but not reduced, where `xs` is nonempty, `len` is positive, and
// `inverse` is an inverse of `xs[0]` modulo $2^k$, by Newton's iteration (see
// `inverse_truncated_newton_to_out`) in wrapping arithmetic, down to lengths below `cutoff`.
//
// This is equivalent to `_gr_poly_inv_series_newton` from `gr_poly/inv_series_newton.c`, FLINT
// 3.6.0, in wrapping arithmetic, except as described for `inverse_truncated_newton_to_out`.
crate_test_fn! {mod_power_of_2_inverse_truncated_newton<T: PrimitiveUnsigned>(
    xs: &[T],
    len: usize,
    pow: u64,
    inverse: T,
    cutoff: usize,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), len)];
    // The scratch follows the inverse in one allocation.
    let mut out = vec![T::ZERO; len + inverse_truncated_newton_scratch_len(len)];
    let (inverse_out, scratch) = out.split_at_mut(len);
    let ops = InverseTruncatedOps {
        basecase: |out: &mut [T], xs: &[T]| {
            mod_power_of_2_inverse_truncated_basecase_to_out(out, xs, inverse);
        },
        mul_middle: |out: &mut [T], ys: &[T], zs: &[T]| mul_middle_wrapping(out, ys, zs, pow),
        mul_truncated: |out: &mut [T], ys: &[T], zs: &[T]| {
            mul_truncated_wrapping(out, ys, zs, pow);
        },
        neg_assign: |y: &mut T| y.wrapping_neg_assign(),
    };
    inverse_truncated_newton_to_out(inverse_out, xs, max(cutoff, 2), scratch, &ops);
    out.truncate(len);
    out
}}

// The length of the series from which inversion modulo $2^k$ uses Newton's iteration. Measured on
// an Apple M-series machine, 2026-10, as the length at which a single step of Newton's iteration on
// a dense series overtakes inverting it coefficient by coefficient: 162 for `u64` with exponents
// from 2 to 64, and 129 for `u128` with exponents of 80 and 128, whose coefficient-by-coefficient
// products cost more.
const fn newton_cutoff<T: PrimitiveUnsigned>() -> usize {
    if T::WIDTH > 64 { 129 } else { 162 }
}

// The first `n` coefficients of the inverse of the power series with coefficients `xs`, correct
// modulo $2^k$, where $k$ is `pow`, which is positive, but not reduced, where `xs` is nonempty, `n`
// is positive, and `inverse` is an inverse of `xs[0]` modulo $2^k$.
//
// This is equivalent to `_nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0, with
// `_gr_nmod_poly_inv_series` from `gr/nmod.c` inlined, in wrapping arithmetic.
pub(crate) fn mod_power_of_2_inverse_truncated_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    n: usize,
    pow: u64,
    inverse: T,
) -> Vec<T> {
    let xs = &xs[..min(xs.len(), n)];
    if xs.len() <= 10 {
        return mod_power_of_2_inverse_truncated_basecase(xs, n, inverse);
    }
    let cutoff = newton_cutoff::<T>();
    if xs.len() < cutoff {
        mod_power_of_2_inverse_truncated_basecase(xs, n, inverse)
    } else {
        mod_power_of_2_inverse_truncated_newton(xs, n, pow, inverse, cutoff)
    }
}

// The inverse modulo $2^k$ of the power series with coefficients `xs`, reduced, truncated to `len`
// coefficients, with `kernel` doing the work in the general case, or `None` if the constant
// coefficient is even.
//
// This is equivalent to `nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that `None` is returned where FLINT aborts, and that the inverse modulo
// $x^0$, or modulo 1, is 0.
crate_test_fn! {mod_power_of_2_inverse_truncated_with<T: PrimitiveUnsigned>(
    xs: &[T],
    len: u64,
    pow: u64,
    kernel: fn(&[T], usize, u64, T) -> Vec<T>,
) -> Option<UnsignedPolynomial<T>> {
    if len == 0 || pow == 0 {
        return Some(UnsignedPolynomial::ZERO);
    }
    let x0 = *xs.first()?;
    if x0 == T::ZERO {
        return None;
    }
    let inverse = x0.mod_power_of_2_inverse(pow)?;
    if xs.len() == 1 {
        return Some(UnsignedPolynomial {
            coefficients: vec![inverse],
        });
    }
    let n = usize::saturating_from(len);
    let mut out = kernel(xs, n, pow, inverse);
    for y in &mut out {
        y.mod_power_of_2_assign(pow);
    }
    Some(from_coefficients_trimmed(out))
}}

/// The coefficients of the inverse as a power series, modulo $2^k$, of the polynomial with
/// coefficients `xs`, keeping only the coefficients of $x^i$ for $i$ less than `len`, trimmed, or
/// `None` if the constant coefficient is not invertible. The coefficients must be reduced modulo
/// $2^k$, where $k$ is `pow`, which must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can invert
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
#[inline]
pub fn mod_power_of_2_inverse_truncated_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    len: u64,
    pow: u64,
) -> Option<Vec<T>> {
    mod_power_of_2_inverse_truncated_with(xs, len, pow, mod_power_of_2_inverse_truncated_dispatch)
        .map(|p| p.coefficients)
}

impl<T: PrimitiveUnsigned> ModPowerOf2InverseTruncated for UnsignedPolynomial<T> {
    type Output = Self;

    /// Inverts an [`UnsignedPolynomial`] as a power series modulo $2^k$, keeping only the
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2InverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("9*x+1").unwrap())
    ///         .mod_power_of_2_inverse_truncated(7, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^6+7*x^5+x^4+7*x^3+x^2+7*x+1"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("2*x+4").unwrap())
    ///         .mod_power_of_2_inverse_truncated(3, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$ and in wrapping arithmetic, except that `None` is returned where
    /// FLINT aborts, and that the inverse modulo $x^0$ or modulo 1 is 0.
    #[inline]
    fn mod_power_of_2_inverse_truncated(self, len: u64, pow: u64) -> Option<Self> {
        (&self).mod_power_of_2_inverse_truncated(len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2InverseTruncated for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Inverts an [`UnsignedPolynomial`] as a power series modulo $2^k$, keeping only the
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2InverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("9*x+1").unwrap())
    ///         .mod_power_of_2_inverse_truncated(7, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^6+7*x^5+x^4+7*x^3+x^2+7*x+1"
    /// );
    /// // The constant coefficient is not invertible.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x+4").unwrap())
    ///         .mod_power_of_2_inverse_truncated(3, 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_inv_series` from `nmod_poly/inv_series.c`, FLINT 3.6.0,
    /// with the modulus $2^k$ and in wrapping arithmetic, except that `None` is returned where
    /// FLINT aborts, and that the inverse modulo $x^0$ or modulo 1 is 0.
    fn mod_power_of_2_inverse_truncated(self, len: u64, pow: u64) -> Option<UnsignedPolynomial<T>> {
        assert_reduced(self, pow);
        mod_power_of_2_inverse_truncated_with(
            &self.coefficients,
            len,
            pow,
            mod_power_of_2_inverse_truncated_dispatch,
        )
    }
}
