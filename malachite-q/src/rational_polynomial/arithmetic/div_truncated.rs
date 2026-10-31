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

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::div::checked_div_helper;
use crate::rational_polynomial::arithmetic::inverse_truncated::inverse_truncated_newton;
use alloc::borrow::Cow;
use core::mem::take;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    CheckedDivTruncated, DivTruncated, DivTruncatedAssign, MulTruncated, Polynomial,
};

// The length at and below which a series is divided by reversed polynomial division rather than by
// multiplying by the divisor's inverse, unless the divisor has integer coefficients and constant
// coefficient 1 or -1, whose inverse is found over the integers and is faster still. Above it,
// division is faster only when the quotient's coefficients stay small, and much slower when they
// grow. Measured on an Apple M-series machine, 2026-10 (see `tune_div_truncated_rational` in
// `bin_util/tune.rs`); FLINT always inverts.
const DIV_TRUNCATED_DIVISION_CUTOFF: u64 = 4;

// The first `len` coefficients of the quotient of `p` by `q` as power series, where `len` is
// positive and the constant coefficient of `q` is nonzero: `p` times the inverse of `q`.
//
// This is equivalent to `_fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0.
crate_test_fn! {div_truncated_by_inverse(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    p.mul_truncated(inverse_truncated_newton(q, len), len)
}}

// The first `len` coefficients of the quotient of `p` by `q` as power series, as for
// `div_truncated_by_inverse`, by polynomial division: reversed and padded, `p` and `q` are
// polynomials of $2n - 1$ and $n$ coefficients, where $n$ is `len`, whose quotient, reversed, is
// the quotient of the series. Unlike the inverse of `q`, whose coefficients grow, the division's
// intermediate values are only as large as the quotient needs.
crate_test_fn! {div_truncated_by_division(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    let p_reversed = p.truncate(len).reverse((len << 1) - 1);
    let q_reversed = q.truncate(len).reverse(len);
    checked_div_helper(Cow::Owned(p_reversed), &q_reversed)
        .unwrap()
        .reverse(len)
}}

// The quotient of `p` by `q` as power series, truncated to `len` coefficients, or `None` if the
// constant coefficient of `q` is 0: `p` times the inverse of `q`.
//
// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0, except
// that `None` is returned where FLINT throws or computes a meaningless result, that the quotient
// modulo $x^0$ is 0, that a divisor with constant coefficient 0 is rejected even when the dividend
// is 0, and that short series are divided by reversed polynomial division.
fn checked_div_truncated_helper(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> Option<RationalPolynomial> {
    if len == 0 {
        return Some(RationalPolynomial::ZERO);
    }
    if *q.numerator.coefficient(0) == 0u32 {
        return None;
    }
    if *p == RationalPolynomial::ZERO {
        return Some(RationalPolynomial::ZERO);
    }
    let integer_unit =
        q.denominator == 1u32 && *q.numerator.coefficient(0).unsigned_abs_ref() == 1u32;
    Some(if len <= DIV_TRUNCATED_DIVISION_CUTOFF && !integer_unit {
        div_truncated_by_division(p, q, len)
    } else {
        div_truncated_by_inverse(p, q, len)
    })
}

fn div_truncated_helper(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    checked_div_truncated_helper(p, q, len)
        .expect("the divisor's constant coefficient must be nonzero")
}

impl DivTruncated<Self> for RationalPolynomial {
    type Output = Self;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1").unwrap())
    ///         .div_truncated(RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .div_truncated(RationalPolynomial::from_str("x+1").unwrap(), 5)
    ///         .to_string(),
    ///     "x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn div_truncated(self, other: Self, len: u64) -> Self {
        div_truncated_helper(&self, &other, len)
    }
}

impl DivTruncated<&Self> for RationalPolynomial {
    type Output = Self;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1").unwrap())
    ///         .div_truncated(&RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .div_truncated(&RationalPolynomial::from_str("x+1").unwrap(), 5)
    ///         .to_string(),
    ///     "x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn div_truncated(self, other: &Self, len: u64) -> Self {
        div_truncated_helper(&self, other, len)
    }
}

impl DivTruncated<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1").unwrap())
    ///         .div_truncated(RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .div_truncated(RationalPolynomial::from_str("x+1").unwrap(), 5)
    ///         .to_string(),
    ///     "x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn div_truncated(self, other: RationalPolynomial, len: u64) -> RationalPolynomial {
        div_truncated_helper(self, &other, len)
    }
}

impl DivTruncated<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1").unwrap())
    ///         .div_truncated(&RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .div_truncated(&RationalPolynomial::from_str("x+1").unwrap(), 5)
    ///         .to_string(),
    ///     "x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn div_truncated(self, other: &RationalPolynomial, len: u64) -> RationalPolynomial {
        div_truncated_helper(self, other, len)
    }
}

impl DivTruncatedAssign<Self> for RationalPolynomial {
    /// Divides a [`RationalPolynomial`] by another as power series in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by value.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1").unwrap();
    /// p.div_truncated_assign(RationalPolynomial::from_str("x+2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "1/8*x^2-1/4*x+1/2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn div_truncated_assign(&mut self, other: Self, len: u64) {
        *self = div_truncated_helper(&take(self), &other, len);
    }
}

impl DivTruncatedAssign<&Self> for RationalPolynomial {
    /// Divides a [`RationalPolynomial`] by another as power series in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by reference.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if `len` is positive and the constant coefficient of `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DivTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1").unwrap();
    /// p.div_truncated_assign(&RationalPolynomial::from_str("x+2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "1/8*x^2-1/4*x+1/2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn div_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = div_truncated_helper(&take(self), other, len);
    }
}

impl CheckedDivTruncated<Self> for RationalPolynomial {
    type Output = Self;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if the divisor's constant coefficient is 0,
    /// taking both by value.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } q_0 \neq 0 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// // The divisor's constant coefficient is 0.
    /// assert!(
    ///     (RationalPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(RationalPolynomial::from_str("x").unwrap(), 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn checked_div_truncated(self, other: Self, len: u64) -> Option<Self> {
        checked_div_truncated_helper(&self, &other, len)
    }
}

impl CheckedDivTruncated<&Self> for RationalPolynomial {
    type Output = Self;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if the divisor's constant coefficient is 0,
    /// taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } q_0 \neq 0 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(&RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// // The divisor's constant coefficient is 0.
    /// assert!(
    ///     (RationalPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(&RationalPolynomial::from_str("x").unwrap(), 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn checked_div_truncated(self, other: &Self, len: u64) -> Option<Self> {
        checked_div_truncated_helper(&self, other, len)
    }
}

impl CheckedDivTruncated<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if the divisor's constant coefficient is 0,
    /// taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } q_0 \neq 0 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// // The divisor's constant coefficient is 0.
    /// assert!(
    ///     (&RationalPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(RationalPolynomial::from_str("x").unwrap(), 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn checked_div_truncated(
        self,
        other: RationalPolynomial,
        len: u64,
    ) -> Option<RationalPolynomial> {
        checked_div_truncated_helper(self, &other, len)
    }
}

impl CheckedDivTruncated<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s as power series, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, or returns `None` if the divisor's constant coefficient is 0,
    /// taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } q_0 \neq 0 \text{ or } n = 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    ///
    /// Short series are divided by reversing both series and dividing the polynomials; longer ones,
    /// and those whose divisor has integer coefficients and constant coefficient 1 or -1, by
    /// multiplying by the inverse of the divisor.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// bits in any numerator or denominator of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1").unwrap())
    ///         .checked_div_truncated(&RationalPolynomial::from_str("x+2").unwrap(), 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/8*x^2-1/4*x+1/2"
    /// );
    /// // The divisor's constant coefficient is 0.
    /// assert!(
    ///     (&RationalPolynomial::from_str("x+1").unwrap())
    ///         .checked_div_truncated(&RationalPolynomial::from_str("x").unwrap(), 3)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div_series` from `fmpq_poly/div_series.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT throws or computes a meaningless result, that the
    /// quotient modulo $x^0$ is 0, and that short series are divided by polynomial division.
    #[inline]
    fn checked_div_truncated(
        self,
        other: &RationalPolynomial,
        len: u64,
    ) -> Option<RationalPolynomial> {
        checked_div_truncated_helper(self, other, len)
    }
}
