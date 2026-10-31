// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2014 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::arithmetic::div::checked_div_helper;
use crate::rational_polynomial::arithmetic::div_rem::{mul_integer, over_integer};
use crate::rational_polynomial::{RationalPolynomial, reduce};
use alloc::borrow::Cow;
use alloc::vec;
use core::cmp::min;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    InverseTruncated, MulMiddle, MulPowerOfX, MulTruncated, Polynomial,
};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// The precision at and below which the inverse is found by division rather than Newton iteration.
// The basecase divides a power of $x$ by the reversed series, which is costly at any coefficient
// size, so one Newton step wins from a precision of about 6 for every size measured. Measured on an
// Apple M-series machine, 2026-10, on dense random series (see `tune_inverse_truncated_rational` in
// `bin_util/tune.rs`); FLINT's cutoff, `FMPQ_POLY_INV_NEWTON_CUTOFF` in
// `fmpq_poly/inv_series_newton.c`, FLINT 3.6.0, is 24.
const INVERSE_TRUNCATED_NEWTON_CUTOFF: usize = 5;

// The first `n` coefficients of the inverse of `p`, of `p_len` coefficients modulo $x^n$, found by
// dividing $x^{n + l - 2}$ by the reversal of `p`'s first $l$ coefficients, where $l$ is $\min(l_p,
// n)$, and reversing the quotient.
//
// This is equivalent to `_fmpq_poly_inv_series_basecase_rev` from `fmpq_poly/inv_series_newton.c`,
// FLINT 3.6.0.
crate_test_fn! {inverse_truncated_basecase(p: &RationalPolynomial, n: u64) -> RationalPolynomial {
    let l = min(p.len(), n);
    let p_reversed = p.truncate(l).reverse(l);
    let x_power = RationalPolynomial::one().mul_power_of_x(n + l - 2);
    checked_div_helper(Cow::Owned(x_power), &p_reversed).unwrap().reverse(n)
}}

// The first `n` coefficients of the inverse of `p`, whose constant coefficient is nonzero, where
// `n` is positive. A series with integer coefficients and constant coefficient 1 or -1 is inverted
// over the integers; a constant, directly; others by Newton iteration from an inverse found by
// `inverse_truncated_basecase`, each step doubling the precision: with $q$ correct to $m$
// coefficients, the next ones are those of $-q \cdot \lfloor (pq \bmod x^n) / x^m \rfloor$.
//
// This is equivalent to `_fmpq_poly_inv_series_newton` from `fmpq_poly/inv_series_newton.c`, FLINT
// 3.6.0.
crate_test_fn! {inverse_truncated_newton(p: &RationalPolynomial, n: u64) -> RationalPolynomial {
    inverse_truncated_newton_with_cutoff(p, n, INVERSE_TRUNCATED_NEWTON_CUTOFF)
}}

// `inverse_truncated_newton`, with Newton iteration stopping, and `inverse_truncated_basecase`
// taking over, at a precision of at most `cutoff`.
crate_test_fn! {inverse_truncated_newton_with_cutoff(
    p: &RationalPolynomial,
    n: u64,
    cutoff: usize,
) -> RationalPolynomial {
    let x_0 = p.numerator.coefficient(0);
    if p.denominator == 1u32 && *x_0.unsigned_abs_ref() == 1u32 {
        return RationalPolynomial::from((&p.numerator).inverse_truncated(n).unwrap());
    }
    let p_len = min(p.len(), n);
    if p_len == 1 {
        return over_integer(
            IntegerPolynomial::from(Integer::from(&p.denominator)),
            x_0.clone(),
        );
    }
    let mut lengths = vec![usize::exact_from(n)];
    let mut m = usize::exact_from(n);
    while m > cutoff {
        m = m.div_ceil(2);
        lengths.push(m);
    }
    let mut m = u64::exact_from(m);
    let mut q = inverse_truncated_basecase(p, m);
    for &next in lengths[..lengths.len() - 1].iter().rev() {
        let next = u64::exact_from(next);
        let p_n = p.numerator.truncate(min(p_len, next));
        let w_len = min(p_n.len() + m - 1, next);
        let w = (&p_n).mul_middle(&q.numerator, m, w_len);
        let w_denominator = &p.denominator * &q.denominator;
        let correction = (&q.numerator).mul_truncated(&w, next - m);
        let numerator = mul_integer(q.numerator, &Integer::from(&w_denominator))
            - correction.mul_power_of_x(m);
        q = reduce(numerator, q.denominator * w_denominator);
        m = next;
    }
    q
}}

// The inverse of `p` as a power series, truncated to `len` coefficients, or `None` if its constant
// coefficient is 0.
//
// This is equivalent to `fmpq_poly_inv_series` from `fmpq_poly.h`, FLINT 3.6.0, except that `None`
// is returned where FLINT throws or computes a meaningless result for a zero constant coefficient,
// and that the inverse modulo $x^0$ is 0.
fn inverse_truncated_helper(p: &RationalPolynomial, len: u64) -> Option<RationalPolynomial> {
    if len == 0 {
        return Some(RationalPolynomial::ZERO);
    }
    if *p.numerator.coefficient(0) == 0u32 {
        return None;
    }
    Some(inverse_truncated_newton(p, len))
}

impl InverseTruncated for RationalPolynomial {
    type Output = Self;

    /// Inverts a [`RationalPolynomial`] as a power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking it by value.
    ///
    /// The inverse exists exactly when the constant coefficient is nonzero; otherwise, `None` is
    /// returned. Modulo $x^0$, every series is invertible and the inverse is 0.
    ///
    /// $$
    /// f(p, n) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } p_0 \neq 0 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg q < n$ and $pq \equiv 1 \pmod{x^n}$.
    ///
    /// A series with integer coefficients and constant coefficient 1 or -1 is inverted over the
    /// integers. Others are inverted by Newton iteration, which doubles the number of correct
    /// coefficients with each step, from an inverse of a few coefficients found by division.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::InverseTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x+2").unwrap())
    ///         .inverse_truncated(3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// // The constant coefficient is 0.
    /// assert!(
    ///     (RationalPolynomial::from_str("x").unwrap())
    ///         .inverse_truncated(3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_inv_series` from `fmpq_poly.h`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws or computes a meaningless result for a zero constant
    /// coefficient, that the inverse modulo $x^0$ is 0, and that Newton iteration takes over at a
    /// shorter length.
    #[inline]
    fn inverse_truncated(self, len: u64) -> Option<Self> {
        inverse_truncated_helper(&self, len)
    }
}

impl InverseTruncated for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Inverts a [`RationalPolynomial`] as a power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking it by reference.
    ///
    /// The inverse exists exactly when the constant coefficient is nonzero; otherwise, `None` is
    /// returned. Modulo $x^0$, every series is invertible and the inverse is 0.
    ///
    /// $$
    /// f(p, n) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } p_0 \neq 0 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg q < n$ and $pq \equiv 1 \pmod{x^n}$.
    ///
    /// A series with integer coefficients and constant coefficient 1 or -1 is inverted over the
    /// integers. Others are inverted by Newton iteration, which doubles the number of correct
    /// coefficients with each step, from an inverse of a few coefficients found by division.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::InverseTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x+2").unwrap())
    ///         .inverse_truncated(3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// // The constant coefficient is 0.
    /// assert!(
    ///     (&RationalPolynomial::from_str("x").unwrap())
    ///         .inverse_truncated(3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_inv_series` from `fmpq_poly.h`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws or computes a meaningless result for a zero constant
    /// coefficient, that the inverse modulo $x^0$ is 0, and that Newton iteration takes over at a
    /// shorter length.
    #[inline]
    fn inverse_truncated(self, len: u64) -> Option<RationalPolynomial> {
        inverse_truncated_helper(self, len)
    }
}
