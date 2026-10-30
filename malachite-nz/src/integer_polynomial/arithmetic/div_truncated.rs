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
use crate::integer_polynomial::arithmetic::coefficient::small_value;
use crate::integer_polynomial::arithmetic::div::div_divide_and_conquer;
use crate::integer_polynomial::arithmetic::inverse_truncated::{
    dot_to_out, inverse_truncated_basecase, small_bits, small_dot, small_dot_bits,
};
use crate::integer_polynomial::arithmetic::pseudo_div_rem::trimmed;
use crate::integer_vector::arithmetic::dot_general::DotScratch;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use core::mem::take;
use malachite_base::num::arithmetic::traits::{DivExact, DivisibleBy};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{CheckedDivTruncated, DivTruncated, DivTruncatedAssign};

// The length of the series, and of the divisor, at and above which reversed polynomial division
// beats `div_truncated_basecase`, for series whose coefficients have at most `bits` bits. The
// basecase's sums stay in one or two words for coefficients of up to about 32 bits, which is where
// it wins longest. Measured on an Apple M-series machine, 2026-10, on exact quotients of dense
// random series (see `tune_div_truncated_cutoff` in `bin_util/tune.rs`); FLINT switches at 72 for
// every size.
const fn div_truncated_divide_and_conquer_cutoff(bits: u64) -> usize {
    // The measured cutoffs are not monotonic above 64 bits, so two ranges share a value.
    match bits {
        0..=1 => 384,
        2..=16 => 768,
        17..=32 => 1536,
        33..=64 => 256,
        65..=128 | 257..=512 => 128,
        129..=256 => 192,
        513..=1024 => 96,
        _ => 64,
    }
}

// `x / y`, or `None` if `y` does not divide `x`.
//
// This is equivalent to `fmpz_divexact_checked` from `fmpz_poly/div_series_basecase.c`, FLINT
// 3.6.0, except that `None` is returned where FLINT throws.
fn div_exact_checked(x: &Integer, y: &Integer) -> Option<Integer> {
    if x.divisible_by(y) {
        Some(x.div_exact(y))
    } else {
        None
    }
}

// The first `n` coefficients of the quotient of the power series with coefficients `xs` and `ys`,
// where both are nonempty, the first coefficient of `ys` is nonzero, and `n` is positive, or `None`
// if the quotient does not have integer coefficients. Each coefficient of the quotient is found
// from the earlier ones.
//
// This is equivalent to `_fmpz_poly_div_series_basecase` from `fmpz_poly/div_series_basecase.c`,
// FLINT 3.6.0, except that `None` is returned where FLINT throws.
crate_test_fn! {div_truncated_basecase(
    xs: &[Integer],
    ys: &[Integer],
    n: usize,
) -> Option<Vec<Integer>> {
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    let x_len = xs.len();
    let y_len = ys.len();
    let y0 = &ys[0];
    let y0_is_unit = *y0.unsigned_abs_ref() == 1u32;
    let mut out = vec![Integer::ZERO; n];
    if y_len == 1 {
        for (o, x) in out.iter_mut().zip(xs) {
            *o = if !y0_is_unit {
                div_exact_checked(x, y0)?
            } else if *y0 == 1u32 {
                x.clone()
            } else {
                -x
            };
        }
        return Some(out);
    }
    if x_len == 1 && y0_is_unit {
        let mut out = inverse_truncated_basecase(ys, n);
        if xs[0] != 1u32 {
            for o in &mut out {
                *o *= &xs[0];
            }
        }
        return Some(out);
    }
    out[0] = if !y0_is_unit {
        div_exact_checked(&xs[0], y0)?
    } else if *y0 == 1u32 {
        xs[0].clone()
    } else {
        -&xs[0]
    };
    let mut ys_bits = vec![0; y_len];
    ys_bits[0] = y0.significant_bits();
    let mut n_small = small_bits(ys, &mut ys_bits, 0);
    let mut out_bits = out[0].significant_bits();
    let mut buffers = DotScratch::default();
    for i in 1..n {
        let s = if let Some(bits) = small_dot_bits(y_len, &ys_bits, n_small, out_bits, i) {
            small_dot(ys, &out, i, bits)
        } else {
            let l = min(i, y_len - 1);
            let mut s = Integer::ZERO;
            dot_to_out(&mut s, false, ys, &out[i - l..i], &mut buffers);
            s
        };
        out[i] = match (i < x_len, y0_is_unit) {
            (true, true) if *y0 == 1u32 => &xs[i] - s,
            (true, true) => s - &xs[i],
            (true, false) => div_exact_checked(&(&xs[i] - s), y0)?,
            (false, true) if *y0 == 1u32 => -s,
            (false, true) => s,
            (false, false) => div_exact_checked(&-s, y0)?,
        };
        match small_value(&out[i]) {
            None => n_small = i,
            Some(v) => out_bits = max(out_bits, v.unsigned_abs().significant_bits()),
        }
    }
    Some(out)
}}

// The first `n` coefficients of the quotient of the power series with coefficients `xs` and `ys`,
// as for `div_truncated_basecase`. Reversed and padded, the dividend and divisor are polynomials of
// $2n - 1$ and $n$ coefficients whose quotient, reversed, is the quotient of the series.
//
// This is equivalent to `_fmpz_poly_div_series_divconquer` from
// `fmpz_poly/div_series_divconquer.c`, FLINT 3.6.0, except that `None` is returned where FLINT
// throws.
crate_test_fn! {div_truncated_divide_and_conquer(
    xs: &[Integer],
    ys: &[Integer],
    n: usize,
) -> Option<Vec<Integer>> {
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    let mut xs_reversed = vec![Integer::ZERO; (n << 1) - 1];
    for (r, x) in xs_reversed.iter_mut().rev().zip(xs) {
        *r = x.clone();
    }
    let mut ys_reversed = vec![Integer::ZERO; n];
    for (r, y) in ys_reversed.iter_mut().rev().zip(ys) {
        *r = y.clone();
    }
    let mut out = div_divide_and_conquer(&xs_reversed, &ys_reversed)?;
    out.reverse();
    Some(out)
}}

// The first `n` coefficients of the quotient of the power series with coefficients `xs` and `ys`,
// as for `div_truncated_basecase`. Long series are divided by reversed polynomial division, whose
// cost, like the basecase's, follows the size of the quotient's coefficients.
//
// This is equivalent to `_fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0, except
// that `None` is returned where FLINT throws, that the cutoff depends on the sizes of the
// coefficients, where FLINT's is 72, and that divisors with constant coefficient 1 or -1 are not
// inverted: the inverse's coefficients grow even when the quotient's do not, which made dividing by
// the inverse 10 to 100 times slower on exact quotients.
crate_test_fn! {div_truncated_dispatch(
    xs: &[Integer],
    ys: &[Integer],
    n: usize,
) -> Option<Vec<Integer>> {
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    let bits = max(vec_max_bits(xs).0, vec_max_bits(ys).0);
    let cutoff = div_truncated_divide_and_conquer_cutoff(bits);
    if n < cutoff || ys.len() < cutoff || xs.len() == 1 {
        div_truncated_basecase(xs, ys, n)
    } else {
        div_truncated_divide_and_conquer(xs, ys, n)
    }
}}

// The quotient of `p` by `q` as power series, truncated to `len` coefficients, or `None` if the
// constant coefficient of `q` is 0 or the quotient does not have integer coefficients.
//
// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0, except
// that `None` is returned where FLINT throws, that the quotient modulo $x^0$ is 0, and that a
// divisor with constant coefficient 0 is rejected even when the dividend is 0 modulo $x^n$.
fn checked_div_truncated_helper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> Option<IntegerPolynomial> {
    if len == 0 {
        return Some(IntegerPolynomial::ZERO);
    }
    let ys = &q.coefficients;
    if *ys.first()? == 0u32 {
        return None;
    }
    let xs = &p.coefficients;
    if xs.is_empty() {
        return Some(IntegerPolynomial::ZERO);
    }
    let n = usize::exact_from(len);
    Some(trimmed(div_truncated_dispatch(xs, ys, n)?))
}

fn div_truncated_helper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    checked_div_truncated_helper(p, q, len).expect(
        "the divisor's constant coefficient must be nonzero, and the quotient must have integer \
        coefficients",
    )
}

impl DivTruncated<Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0, or if the quotient
    /// does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .div_truncated(IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("1").unwrap())
    ///         .div_truncated(IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn div_truncated(self, other: Self, len: u64) -> Self {
        div_truncated_helper(&self, &other, len)
    }
}

impl DivTruncated<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0, or if the quotient
    /// does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .div_truncated(&IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("1").unwrap())
    ///         .div_truncated(&IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn div_truncated(self, other: &Self, len: u64) -> Self {
        div_truncated_helper(&self, other, len)
    }
}

impl DivTruncated<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0, or if the quotient
    /// does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .div_truncated(IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("1").unwrap())
    ///         .div_truncated(IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn div_truncated(self, other: IntegerPolynomial, len: u64) -> IntegerPolynomial {
        div_truncated_helper(self, &other, len)
    }
}

impl DivTruncated<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0, or if the quotient
    /// does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .div_truncated(&IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("1").unwrap())
    ///         .div_truncated(&IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn div_truncated(self, other: &IntegerPolynomial, len: u64) -> IntegerPolynomial {
        div_truncated_helper(self, other, len)
    }
}

impl DivTruncatedAssign<Self> for IntegerPolynomial {
    /// Divides an [`IntegerPolynomial`] by another as power series in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by value.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0, or if the quotient
    /// does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap();
    /// p.div_truncated_assign(IntegerPolynomial::from_str("3*x+2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn div_truncated_assign(&mut self, other: Self, len: u64) {
        *self = div_truncated_helper(&take(self), &other, len);
    }
}

impl DivTruncatedAssign<&Self> for IntegerPolynomial {
    /// Divides an [`IntegerPolynomial`] by another as power series in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by reference.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0, or if the quotient
    /// does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap();
    /// p.div_truncated_assign(&IntegerPolynomial::from_str("3*x+2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn div_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = div_truncated_helper(&take(self), other, len);
    }
}

impl CheckedDivTruncated<Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if there is no such quotient, taking both by
    /// value.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } r \text{ exists}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .checked_div_truncated(IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// // The quotient's constant coefficient would be 1/2.
    /// assert!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(IntegerPolynomial::from_str("x+2").unwrap(), 2)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn checked_div_truncated(self, other: Self, len: u64) -> Option<Self> {
        checked_div_truncated_helper(&self, &other, len)
    }
}

impl CheckedDivTruncated<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if there is no such quotient, taking the first by
    /// value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } r \text{ exists}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .checked_div_truncated(&IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(&IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// // The quotient's constant coefficient would be 1/2.
    /// assert!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(&IntegerPolynomial::from_str("x+2").unwrap(), 2)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn checked_div_truncated(self, other: &Self, len: u64) -> Option<Self> {
        checked_div_truncated_helper(&self, other, len)
    }
}

impl CheckedDivTruncated<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if there is no such quotient, taking the first by
    /// reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } r \text{ exists}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .checked_div_truncated(IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// // The quotient's constant coefficient would be 1/2.
    /// assert!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(IntegerPolynomial::from_str("x+2").unwrap(), 2)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn checked_div_truncated(
        self,
        other: IntegerPolynomial,
        len: u64,
    ) -> Option<IntegerPolynomial> {
        checked_div_truncated_helper(self, &other, len)
    }
}

impl CheckedDivTruncated<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if there is no such quotient, taking both by
    /// reference.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } r \text{ exists}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// The quotient exists when the constant coefficient of the divisor is nonzero and the quotient
    /// of the series over the rationals has integer coefficients. Modulo $x^0$, the quotient is 0.
    ///
    /// Short series are divided one coefficient at a time; longer ones by reversing both series and
    /// dividing the polynomials. Where the switch happens depends on the sizes of the coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("6*x^2+7*x+2").unwrap())
    ///         .checked_div_truncated(&IntegerPolynomial::from_str("3*x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(&IntegerPolynomial::from_str("x+1").unwrap(), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-x^3+x^2-x+1"
    /// );
    /// // The quotient's constant coefficient would be 1/2.
    /// assert!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(&IntegerPolynomial::from_str("x+2").unwrap(), 2)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div_series` from `fmpz_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws, and that the quotient modulo $x^0$ is 0.
    #[inline]
    fn checked_div_truncated(
        self,
        other: &IntegerPolynomial,
        len: u64,
    ) -> Option<IntegerPolynomial> {
        checked_div_truncated_helper(self, other, len)
    }
}
