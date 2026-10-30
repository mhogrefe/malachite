// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2014, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::coefficient::{SMALL_FMPZ_BITCOUNT_MAX, small_value};
use crate::integer_polynomial::arithmetic::mul_middle::mul_middle_to_out;
use crate::integer_polynomial::arithmetic::mul_truncated::mul_truncated_to_out;
use crate::integer_polynomial::arithmetic::pseudo_div_rem::trimmed;
use crate::integer_vector::arithmetic::dot_general::{DotScratch, vec_dot_general_strided_to_out};
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::TWICE_WIDTH;
use crate::platform::{Limb, SignedDoubleLimb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{NegAssign, XXXAddYYYToZZZ};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::InverseTruncated;

// The length at and above which `inverse_truncated_newton` uses Newton iteration.
//
// This is the cutoff in `_fmpz_poly_inv_series` and `_fmpz_poly_inv_series_newton` from
// `fmpz_poly/inv_series.c`, FLINT 3.6.0.
const INVERSE_TRUNCATED_NEWTON_CUTOFF: usize = 64;

// The length at and above which a Newton step beats `inverse_truncated_basecase`, for a series
// whose coefficients have at most `bits` bits: the shortest length at which one Newton step, from
// the basecase's inverse to half the precision, is faster. The inverse's coefficients grow with
// both the length and the size of the series' coefficients, so the basecase's cost rises steeply;
// it wins longest for coefficients of 8 to 64 bits. Measured on an Apple M-series machine, 2026-10,
// on dense random series (see `tune_inverse_truncated_cutoff` in `bin_util/tune.rs`); FLINT uses
// `INVERSE_TRUNCATED_NEWTON_CUTOFF` for every size.
const fn inverse_truncated_newton_cutoff(bits: u64) -> usize {
    // The cutoff rises to a peak at 17 to 64 bits and then falls, so tiny and huge coefficients
    // share a value.
    match bits {
        0..=4 | 513..=1024 => 48,
        5..=8 => 512,
        9..=16 => 768,
        17..=64 => 1024,
        65..=128 => 320,
        129..=256 => 192,
        257..=512 => 96,
        _ => 32,
    }
}

// The sum of `ys[j] * qs[i - j]` for $1 \leq j \leq \min(i, $ `ys.len()` $- 1)$, where every term
// is a small `fmpz` and the sum and every partial sum fit in a signed number of `bits` bits, which
// is less than `3 * Limb::WIDTH`. The sum is accumulated in one, two, or three words, as few as
// `bits` allows.
//
// This is equivalent to the small-coefficient loops in `_fmpz_poly_inv_series_basecase` from
// `fmpz_poly/inv_series.c` and in `_fmpz_poly_div_series_basecase` from
// `fmpz_poly/div_series_basecase.c`, FLINT 3.6.0.
pub(crate) fn small_dot(ys: &[Integer], qs: &[Integer], i: usize, bits: u64) -> Integer {
    let l = min(i, ys.len() - 1);
    let terms = (1..=l).map(|j| {
        (
            small_value(&ys[j]).unwrap(),
            small_value(&qs[i - j]).unwrap(),
        )
    });
    if bits < Limb::WIDTH {
        let mut s: SignedLimb = 0;
        for (x, y) in terms {
            s += x * y;
        }
        Integer::from(s)
    } else if bits < TWICE_WIDTH {
        let mut s: SignedDoubleLimb = 0;
        for (x, y) in terms {
            s += SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y);
        }
        Integer::from(s)
    } else {
        let (mut s_2, mut s_1, mut s_0): (Limb, Limb, Limb) = (0, 0, 0);
        for (x, y) in terms {
            let p = SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y);
            let (hi, lo) = (
                Limb::wrapping_from(p >> Limb::WIDTH),
                Limb::wrapping_from(p),
            );
            let extension = if p < 0 { Limb::MAX } else { 0 };
            (s_2, s_1, s_0) = Limb::xxx_add_yyy_to_zzz(s_2, s_1, s_0, extension, hi, lo);
        }
        Integer::from_twos_complement_limbs_asc(&[s_0, s_1, s_2])
    }
}

// Sets `out` to the sum of `ys[j] * qs[j - 1]` for $1 \leq j \leq$ `qs.len()`, read in reverse,
// that is, the sum of `ys[j] * qs[i - j]` for `qs` the `l` coefficients before coefficient `i`;
// negated if `subtract` is set. `ys` must have more than `qs.len()` elements.
//
// This is equivalent to the `_fmpz_vec_dot_general` calls in `_fmpz_poly_inv_series_basecase` from
// `fmpz_poly/inv_series.c` and in `_fmpz_poly_div_series_basecase` from
// `fmpz_poly/div_series_basecase.c`, FLINT 3.6.0.
pub(crate) fn dot_to_out(
    out: &mut Integer,
    subtract: bool,
    ys: &[Integer],
    qs: &[Integer],
    buffers: &mut DotScratch,
) {
    vec_dot_general_strided_to_out(out, None, subtract, &ys[1..=qs.len()], qs, 1, true, buffers);
}

// The number of bits that bounds a sum computed by `small_dot` at step `i`, or `None` if the fast
// code cannot be used: when the coefficients up to `i` of `ys` are not all small, or the largest
// coefficient computed so far has `q_bits` bits, too many for a small `fmpz`. Since the
// coefficients of `ys` up to `i` are small, `ys_bits[i]` is at most `SMALL_FMPZ_BITCOUNT_MAX`, so
// the bound is less than `3 * Limb::WIDTH`.
//
// This is equivalent to the bit bound in `_fmpz_poly_inv_series_basecase` from
// `fmpz_poly/inv_series.c` and in `_fmpz_poly_div_series_basecase` from
// `fmpz_poly/div_series_basecase.c`, FLINT 3.6.0, except that FLINT also checks `ys_bits[i]` and
// compares the bound with `3 * Limb::WIDTH`, checks which cannot fail.
pub(crate) fn small_dot_bits(
    ys_len: usize,
    ys_bits: &[u64],
    n_small: usize,
    q_bits: u64,
    i: usize,
) -> Option<u64> {
    if i >= n_small || q_bits > SMALL_FMPZ_BITCOUNT_MAX {
        None
    } else {
        let b = min(i, ys_len - 1);
        Some(b.significant_bits() + ys_bits[i] + q_bits + 1)
    }
}

// The running maxima of the numbers of bits of the coefficients of `ys`, for as long as they are
// small `fmpz`s, and the number of them that are, starting from index `start`; the entries before
// `start` must already be set.
pub(crate) fn small_bits(ys: &[Integer], ys_bits: &mut [u64], start: usize) -> usize {
    let mut n_small = start;
    while n_small < ys.len() {
        let Some(b) = small_value(&ys[n_small]) else {
            break;
        };
        let b = b.unsigned_abs().significant_bits();
        ys_bits[n_small] = if n_small == 0 {
            b
        } else {
            max(b, ys_bits[n_small - 1])
        };
        n_small += 1;
    }
    n_small
}

// The first `n` coefficients of the inverse of the power series with coefficients `xs`, where `xs`
// is nonempty, its first coefficient is 1 or -1, and `n` is positive. Each coefficient of the
// inverse is found from the earlier ones; binomials have a closed form.
//
// This is equivalent to `_fmpz_poly_inv_series_basecase` from `fmpz_poly/inv_series.c`, FLINT
// 3.6.0.
crate_test_fn! {inverse_truncated_basecase(xs: &[Integer], n: usize) -> Vec<Integer> {
    let xs = &xs[..min(xs.len(), n)];
    let x_len = xs.len();
    let neg = xs[0] == 1u32;
    let mut out = vec![Integer::ZERO; n];
    out[0] = xs[0].clone();
    if x_len == 1 {
        return out;
    }
    if x_len == 2 || xs[1..x_len - 1].iter().all(|x| *x == 0u32) {
        // Binomials
        let step = x_len - 1;
        out[step] = -&xs[step];
        let mut i = step << 1;
        while i < n {
            let t = if neg {
                &out[i - step] * &out[step]
            } else {
                &out[i - step] * &xs[step]
            };
            out[i] = t;
            i += step;
        }
        return out;
    }
    let mut xs_bits = vec![0; x_len];
    xs_bits[0] = 1;
    let mut n_small = small_bits(xs, &mut xs_bits, 1);
    let mut out_bits = 1;
    let mut buffers = DotScratch::default();
    for i in 1..n {
        if let Some(bits) = small_dot_bits(x_len, &xs_bits, n_small, out_bits, i) {
            let s = small_dot(xs, &out, i, bits);
            out[i] = if neg { -s } else { s };
            match small_value(&out[i]) {
                None => n_small = i,
                Some(v) => out_bits = max(out_bits, v.unsigned_abs().significant_bits()),
            }
        } else {
            let l = min(i, x_len - 1);
            let (done, rest) = out.split_at_mut(i);
            dot_to_out(&mut rest[0], neg, xs, &done[i - l..], &mut buffers);
        }
    }
    out
}}

// The first `n` coefficients of the inverse of the power series with coefficients `xs`, as for
// `inverse_truncated_basecase`. The inverse to about half the precision is found recursively and
// then doubled by one Newton step: with $q$ correct to $m$ coefficients, the next ones are those of
// $-q \cdot \lfloor (xq \bmod x^n) / x^m \rfloor$, where the middle product holds the error.
//
// This is equivalent to `_fmpz_poly_inv_series_newton` from `fmpz_poly/inv_series.c`, FLINT 3.6.0.
crate_test_fn! {inverse_truncated_newton(xs: &[Integer], n: usize) -> Vec<Integer> {
    inverse_truncated_newton_with_cutoff(xs, n, INVERSE_TRUNCATED_NEWTON_CUTOFF)
}}

// `inverse_truncated_newton`, with the recursion ending in `inverse_truncated_basecase` once the
// precision is less than `cutoff`.
crate_test_fn! {inverse_truncated_newton_with_cutoff(
    xs: &[Integer],
    mut n: usize,
    cutoff: usize,
) -> Vec<Integer> {
    let x_len = min(xs.len(), n);
    if x_len < cutoff {
        return inverse_truncated_basecase(xs, n);
    }
    let mut lengths = vec![n];
    while n >= cutoff {
        n = n.div_ceil(2);
        lengths.push(n);
    }
    let mut out = inverse_truncated_basecase(xs, n);
    let mut w = Vec::new();
    for &next in lengths[..lengths.len() - 1].iter().rev() {
        let m = n;
        n = next;
        let xs_n = &xs[..min(x_len, n)];
        let w_len = min(xs_n.len() + m - 1, n);
        w.clear();
        w.resize(w_len - m, Integer::ZERO);
        mul_middle_to_out(&mut w, xs_n, &out, m, w_len);
        out.resize(n, Integer::ZERO);
        let (low, high) = out.split_at_mut(m);
        mul_truncated_to_out(high, low, &w);
        for x in high {
            x.neg_assign();
        }
    }
    out
}}

// The first `n` coefficients of the inverse of the power series with coefficients `xs`, as for
// `inverse_truncated_basecase`. Newton iteration is used, down to a precision of
// `inverse_truncated_newton_cutoff`, when the series and `n` both reach that cutoff.
//
// This is equivalent to `_fmpz_poly_inv_series` from `fmpz_poly/inv_series.c`, FLINT 3.6.0, except
// that the cutoff depends on the sizes of the coefficients, where FLINT's is 64 for every size.
crate_test_fn! {inverse_truncated_dispatch(xs: &[Integer], n: usize) -> Vec<Integer> {
    let xs_n = &xs[..min(xs.len(), n)];
    let cutoff = inverse_truncated_newton_cutoff(vec_max_bits(xs_n).0);
    if xs_n.len() < cutoff || n < cutoff {
        inverse_truncated_basecase(xs, n)
    } else {
        inverse_truncated_newton_with_cutoff(xs, n, cutoff)
    }
}}

// The inverse of `p` as a power series, truncated to `len` coefficients, or `None` if its constant
// coefficient is not 1 or -1.
//
// This is equivalent to `fmpz_poly_inv_series` from `fmpz_poly/inv_series.c`, FLINT 3.6.0, except
// that `None` is returned where FLINT throws on a zero series or computes a meaningless result for
// a constant coefficient other than 1 or -1, and that the inverse modulo $x^0$ is 0.
fn inverse_truncated_helper(p: &IntegerPolynomial, len: u64) -> Option<IntegerPolynomial> {
    if len == 0 {
        return Some(IntegerPolynomial::ZERO);
    }
    let xs = &p.coefficients;
    let x0 = xs.first()?;
    if x0.unsigned_abs_ref() != &1u32 {
        return None;
    }
    if xs.len() == 1 {
        return Some(p.clone());
    }
    let n = usize::exact_from(len);
    Some(trimmed(inverse_truncated_dispatch(xs, n)))
}

impl InverseTruncated for IntegerPolynomial {
    type Output = Self;

    /// Inverts an [`IntegerPolynomial`] as a power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking it by value.
    ///
    /// The inverse exists exactly when the constant coefficient is 1 or -1; otherwise, `None` is
    /// returned. Modulo $x^0$, every series is invertible and the inverse is 0.
    ///
    /// $$
    /// f(p, n) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } p_0 = \pm 1 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg q < n$ and $pq \equiv 1 \pmod{x^n}$.
    ///
    /// Short series are inverted one coefficient at a time; longer ones by Newton iteration, which
    /// doubles the number of correct coefficients with each step. Where the switch happens depends
    /// on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::InverseTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .inverse_truncated(4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+x+1").unwrap())
    ///         .inverse_truncated(6)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^4+x^3-x+1"
    /// );
    /// // The constant coefficient is not 1 or -1.
    /// assert!(
    ///     (IntegerPolynomial::from_str("x+2").unwrap())
    ///         .inverse_truncated(3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_inv_series` from `fmpz_poly/inv_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws on a zero series or computes a meaningless
    /// result for a constant coefficient other than 1 or -1, and that the inverse modulo $x^0$ is
    /// 0, where FLINT throws.
    #[inline]
    fn inverse_truncated(self, len: u64) -> Option<Self> {
        inverse_truncated_helper(&self, len)
    }
}

impl InverseTruncated for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Inverts an [`IntegerPolynomial`] as a power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking it by reference.
    ///
    /// The inverse exists exactly when the constant coefficient is 1 or -1; otherwise, `None` is
    /// returned. Modulo $x^0$, every series is invertible and the inverse is 0.
    ///
    /// $$
    /// f(p, n) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } p_0 = \pm 1 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg q < n$ and $pq \equiv 1 \pmod{x^n}$.
    ///
    /// Short series are inverted one coefficient at a time; longer ones by Newton iteration, which
    /// doubles the number of correct coefficients with each step. Where the switch happens depends
    /// on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::InverseTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .inverse_truncated(4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+x+1").unwrap())
    ///         .inverse_truncated(6)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^4+x^3-x+1"
    /// );
    /// // The constant coefficient is not 1 or -1.
    /// assert!(
    ///     (&IntegerPolynomial::from_str("x+2").unwrap())
    ///         .inverse_truncated(3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_inv_series` from `fmpz_poly/inv_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws on a zero series or computes a meaningless
    /// result for a constant coefficient other than 1 or -1, and that the inverse modulo $x^0$ is
    /// 0, where FLINT throws.
    #[inline]
    fn inverse_truncated(self, len: u64) -> Option<IntegerPolynomial> {
        inverse_truncated_helper(self, len)
    }
}
