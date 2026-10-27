// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::{RationalPolynomial, reduce};
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::take;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{ComposeTruncated, ComposeTruncatedAssign, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// Panics unless the constant coefficient of `q` is 0, and returns the number of leading
// coefficients of an outer polynomial with `p_len` coefficients that can affect its composition
// with `q` truncated to `len` coefficients: at most `len`, and, if the first nonzero coefficient of
// `q` is that of $x^v$, at most $\lceil len/v \rceil$.
fn kept_len(p_len: usize, q: &RationalPolynomial, len: u64) -> usize {
    let ys = q.numerator.coefficients_asc();
    if let Some(y) = ys.first() {
        assert_eq!(
            *y, 0u32,
            "the inner polynomial must have constant coefficient 0"
        );
    }
    let n = usize::try_from(len).unwrap_or(usize::MAX);
    match ys.iter().position(|y| *y != 0u32) {
        Some(v) if n != 0 => min(p_len, (n - 1) / v + 1),
        _ => min(p_len, n),
    }
}

// Composes the polynomial `xs / d`, where `xs` holds the coefficients that can affect the result
// and `d` is positive, with `q` as power series, keeping the first `len` coefficients.
fn compose_truncated_helper(
    mut xs: Vec<Integer>,
    d: Natural,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    if xs.is_empty() || len == 0 {
        return RationalPolynomial::ZERO;
    }
    if xs.len() == 1 || q.numerator == 0u32 {
        // Only the constant coefficient survives. The truncated polynomial need not be in lowest
        // terms.
        xs.truncate(1);
        return reduce(IntegerPolynomial::from_coefficients_asc(xs), d);
    }
    let e = &q.denominator;
    if *e == 1u32 {
        return reduce(
            IntegerPolynomial::from_coefficients_asc(xs).compose_truncated(&q.numerator, len),
            d,
        );
    }
    // p(x / e) = (sum_i N_i e^(l - 1 - i) x^i) / (d e^(l - 1)), where l is the number of
    // coefficients kept.
    let e_integer = Integer::from(e);
    let mut power = Integer::ONE;
    for x in xs.iter_mut().rev().skip(1) {
        power *= &e_integer;
        *x *= &power;
    }
    let rescaled = reduce(
        IntegerPolynomial::from_coefficients_asc(xs),
        d * power.unsigned_abs(),
    );
    reduce(
        rescaled.numerator.compose_truncated(&q.numerator, len),
        rescaled.denominator,
    )
}

fn compose_truncated_owned(
    p: RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    let mut xs = p.numerator.into_coefficients_asc();
    xs.truncate(kept_len(xs.len(), q, len));
    compose_truncated_helper(xs, p.denominator, q, len)
}

fn compose_truncated_ref(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    let xs = p.numerator.coefficients_asc();
    let xs = xs[..kept_len(xs.len(), q, len)].to_vec();
    compose_truncated_helper(xs, p.denominator.clone(), q, len)
}

impl ComposeTruncated<Self> for RationalPolynomial {
    type Output = Self;

    /// Composes two [`RationalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. Writing $q = Q/e$ with $Q$
    /// an integer polynomial and $e$ a positive integer, $p(q)$ is $p(x/e)$ composed with $Q$; the
    /// numerator of $p(x/e)$, with the coefficients of $p$ that matter scaled to clear its
    /// denominators, is an integer polynomial, so the work is one composition of integer
    /// polynomials (see the `IntegerPolynomial` implementation of
    /// [`compose_truncated`](ComposeTruncated::compose_truncated)), with the numerators reduced to
    /// lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{7/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial: scaling
    /// gives integer coefficients of $O(nm)$ bits.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^2+x+1/3").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("x^2+1/2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "9/8*x^2+1/2*x+1/3"
    /// );
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("2/3*x").unwrap(), 4)
    ///         .to_string(),
    ///     "8/27*x^3+4/9*x^2+2/3*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^4+x+1").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("2*x+2").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("1/3*x").unwrap(), 1)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: Self, len: u64) -> Self {
        compose_truncated_owned(self, &other, len)
    }
}

impl ComposeTruncated<&Self> for RationalPolynomial {
    type Output = Self;

    /// Composes two [`RationalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. Writing $q = Q/e$ with $Q$
    /// an integer polynomial and $e$ a positive integer, $p(q)$ is $p(x/e)$ composed with $Q$; the
    /// numerator of $p(x/e)$, with the coefficients of $p$ that matter scaled to clear its
    /// denominators, is an integer polynomial, so the work is one composition of integer
    /// polynomials (see the `IntegerPolynomial` implementation of
    /// [`compose_truncated`](ComposeTruncated::compose_truncated)), with the numerators reduced to
    /// lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{7/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial: scaling
    /// gives integer coefficients of $O(nm)$ bits.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^2+x+1/3").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("x^2+1/2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "9/8*x^2+1/2*x+1/3"
    /// );
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("2/3*x").unwrap(), 4)
    ///         .to_string(),
    ///     "8/27*x^3+4/9*x^2+2/3*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^4+x+1").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("2*x+2").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("1/3*x").unwrap(), 1)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: &Self, len: u64) -> Self {
        compose_truncated_owned(self, other, len)
    }
}

impl ComposeTruncated<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Composes two [`RationalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. Writing $q = Q/e$ with $Q$
    /// an integer polynomial and $e$ a positive integer, $p(q)$ is $p(x/e)$ composed with $Q$; the
    /// numerator of $p(x/e)$, with the coefficients of $p$ that matter scaled to clear its
    /// denominators, is an integer polynomial, so the work is one composition of integer
    /// polynomials (see the `IntegerPolynomial` implementation of
    /// [`compose_truncated`](ComposeTruncated::compose_truncated)), with the numerators reduced to
    /// lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{7/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial: scaling
    /// gives integer coefficients of $O(nm)$ bits.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+x+1/3").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("x^2+1/2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "9/8*x^2+1/2*x+1/3"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("2/3*x").unwrap(), 4)
    ///         .to_string(),
    ///     "8/27*x^3+4/9*x^2+2/3*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^4+x+1").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("2*x+2").unwrap())
    ///         .compose_truncated(RationalPolynomial::from_str("1/3*x").unwrap(), 1)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: RationalPolynomial, len: u64) -> RationalPolynomial {
        compose_truncated_ref(self, &other, len)
    }
}

impl ComposeTruncated<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Composes two [`RationalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. Writing $q = Q/e$ with $Q$
    /// an integer polynomial and $e$ a positive integer, $p(q)$ is $p(x/e)$ composed with $Q$; the
    /// numerator of $p(x/e)$, with the coefficients of $p$ that matter scaled to clear its
    /// denominators, is an integer polynomial, so the work is one composition of integer
    /// polynomials (see the `IntegerPolynomial` implementation of
    /// [`compose_truncated`](ComposeTruncated::compose_truncated)), with the numerators reduced to
    /// lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{7/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial: scaling
    /// gives integer coefficients of $O(nm)$ bits.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+x+1/3").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("x^2+1/2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "9/8*x^2+1/2*x+1/3"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("2/3*x").unwrap(), 4)
    ///         .to_string(),
    ///     "8/27*x^3+4/9*x^2+2/3*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^4+x+1").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("2*x+2").unwrap())
    ///         .compose_truncated(&RationalPolynomial::from_str("1/3*x").unwrap(), 1)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: &RationalPolynomial, len: u64) -> RationalPolynomial {
        compose_truncated_ref(self, other, len)
    }
}

impl ComposeTruncatedAssign<Self> for RationalPolynomial {
    /// Composes a [`RationalPolynomial`] with another in place and truncates the result to its
    /// first `len` coefficients, taking the second by value.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. Writing $q = Q/e$ with $Q$
    /// an integer polynomial and $e$ a positive integer, $p(q)$ is $p(x/e)$ composed with $Q$; the
    /// numerator of $p(x/e)$, with the coefficients of $p$ that matter scaled to clear its
    /// denominators, is an integer polynomial, so the work is one composition of integer
    /// polynomials (see the `IntegerPolynomial` implementation of
    /// [`compose_truncated`](ComposeTruncated::compose_truncated)), with the numerators reduced to
    /// lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{7/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial: scaling
    /// gives integer coefficients of $O(nm)$ bits.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+x+1/3").unwrap();
    /// p.compose_truncated_assign(RationalPolynomial::from_str("x^2+1/2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "9/8*x^2+1/2*x+1/3");
    /// let mut p = RationalPolynomial::from_str("x^3+x^2+x+1").unwrap();
    /// p.compose_truncated_assign(RationalPolynomial::from_str("2/3*x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "8/27*x^3+4/9*x^2+2/3*x+1");
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// let mut p = RationalPolynomial::from_str("1/2*x^4+x+1").unwrap();
    /// p.compose_truncated_assign(RationalPolynomial::from_str("x^2-x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "x^2-x+1");
    /// let mut p = RationalPolynomial::from_str("2*x+2").unwrap();
    /// p.compose_truncated_assign(RationalPolynomial::from_str("1/3*x").unwrap(), 1);
    /// assert_eq!(p.to_string(), "2");
    /// ```
    #[inline]
    fn compose_truncated_assign(&mut self, other: Self, len: u64) {
        *self = compose_truncated_owned(take(self), &other, len);
    }
}

impl ComposeTruncatedAssign<&Self> for RationalPolynomial {
    /// Composes a [`RationalPolynomial`] with another in place and truncates the result to its
    /// first `len` coefficients, taking the second by reference.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. Writing $q = Q/e$ with $Q$
    /// an integer polynomial and $e$ a positive integer, $p(q)$ is $p(x/e)$ composed with $Q$; the
    /// numerator of $p(x/e)$, with the coefficients of $p$ that matter scaled to clear its
    /// denominators, is an integer polynomial, so the work is one composition of integer
    /// polynomials (see the `IntegerPolynomial` implementation of
    /// [`compose_truncated`](ComposeTruncated::compose_truncated)), with the numerators reduced to
    /// lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{7/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial: scaling
    /// gives integer coefficients of $O(nm)$ bits.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+x+1/3").unwrap();
    /// p.compose_truncated_assign(&RationalPolynomial::from_str("x^2+1/2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "9/8*x^2+1/2*x+1/3");
    /// let mut p = RationalPolynomial::from_str("x^3+x^2+x+1").unwrap();
    /// p.compose_truncated_assign(&RationalPolynomial::from_str("2/3*x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "8/27*x^3+4/9*x^2+2/3*x+1");
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// let mut p = RationalPolynomial::from_str("1/2*x^4+x+1").unwrap();
    /// p.compose_truncated_assign(&RationalPolynomial::from_str("x^2-x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "x^2-x+1");
    /// let mut p = RationalPolynomial::from_str("2*x+2").unwrap();
    /// p.compose_truncated_assign(&RationalPolynomial::from_str("1/3*x").unwrap(), 1);
    /// assert_eq!(p.to_string(), "2");
    /// ```
    #[inline]
    fn compose_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = compose_truncated_owned(take(self), other, len);
    }
}
