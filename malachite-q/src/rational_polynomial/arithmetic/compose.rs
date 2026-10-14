// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::{RationalPolynomial, reduce};
use core::mem::take;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::{Compose, ComposeAssign, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// Composes the polynomial `numerator / denominator`, in lowest terms, with `q`.
fn compose_helper(
    numerator: IntegerPolynomial,
    denominator: Natural,
    q: &RationalPolynomial,
) -> RationalPolynomial {
    if numerator.len() <= 1 {
        // A constant polynomial is unchanged by composition, and is already in lowest terms.
        return RationalPolynomial {
            numerator,
            denominator,
        };
    }
    if q.numerator == 0u32 {
        // p(0) is the constant term.
        let constant = numerator.into_coefficients_asc().swap_remove(0);
        return reduce(IntegerPolynomial::from(constant), denominator);
    }
    let e = &q.denominator;
    if *e == 1u32 {
        return reduce(numerator.compose(&q.numerator), denominator);
    }
    // p(x / e) = (sum_i N_i e^(n - 1 - i) x^i) / (d e^(n - 1)), where p = N / d has n coefficients.
    let e_integer = Integer::from(e);
    let mut xs = numerator.into_coefficients_asc();
    let mut power = Integer::ONE;
    for x in xs.iter_mut().rev().skip(1) {
        power *= &e_integer;
        *x *= &power;
    }
    let rescaled = reduce(
        IntegerPolynomial::from_coefficients_asc(xs),
        denominator * power.unsigned_abs(),
    );
    reduce(
        rescaled.numerator.compose(&q.numerator),
        rescaled.denominator,
    )
}

impl Compose<Self> for RationalPolynomial {
    type Output = Self;

    /// Composes two [`RationalPolynomial`]s, giving $p(q(x))$, taking both by value.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Writing $q = Q/e$ with $Q$ an integer polynomial and $e$ a positive integer, $p(q)$ is
    /// $p(x/e)$ evaluated at $Q$. The numerator of $p(x/e)$, scaled to clear its denominators, is
    /// an integer polynomial, so the work is one integer composition, with the numerators reduced
    /// to lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. A constant $p$ is returned as it is. If $q$ is not constant and $p$ is nonzero, the
    /// degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// numerator coefficient or denominator of either polynomial: $s$ bounds the size of the
    /// numerator coefficients of the integer composition.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3")
    ///         .unwrap()
    ///         .compose(RationalPolynomial::from_str("2*x+1/2").unwrap())
    ///         .to_string(),
    ///     "2*x^2+x+11/24"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .compose(RationalPolynomial::from_str("x^2+1/2*x").unwrap())
    ///         .to_string(),
    ///     "x^6+3/2*x^5+3/4*x^4+1/8*x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^2-1/3*x+1/2")
    ///         .unwrap()
    ///         .compose(RationalPolynomial::from_str("5/2").unwrap())
    ///         .to_string(),
    ///     "71/12"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_compose` from `fmpq_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: Self) -> Self {
        compose_helper(self.numerator, self.denominator, &other)
    }
}

impl Compose<&Self> for RationalPolynomial {
    type Output = Self;

    /// Composes two [`RationalPolynomial`]s, giving $p(q(x))$, taking the first by value and the
    /// second by reference.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Writing $q = Q/e$ with $Q$ an integer polynomial and $e$ a positive integer, $p(q)$ is
    /// $p(x/e)$ evaluated at $Q$. The numerator of $p(x/e)$, scaled to clear its denominators, is
    /// an integer polynomial, so the work is one integer composition, with the numerators reduced
    /// to lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. A constant $p$ is returned as it is. If $q$ is not constant and $p$ is nonzero, the
    /// degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// numerator coefficient or denominator of either polynomial: $s$ bounds the size of the
    /// numerator coefficients of the integer composition.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3")
    ///         .unwrap()
    ///         .compose(&RationalPolynomial::from_str("2*x+1/2").unwrap())
    ///         .to_string(),
    ///     "2*x^2+x+11/24"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .compose(&RationalPolynomial::from_str("x^2+1/2*x").unwrap())
    ///         .to_string(),
    ///     "x^6+3/2*x^5+3/4*x^4+1/8*x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^2-1/3*x+1/2")
    ///         .unwrap()
    ///         .compose(&RationalPolynomial::from_str("5/2").unwrap())
    ///         .to_string(),
    ///     "71/12"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_compose` from `fmpq_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: &Self) -> Self {
        compose_helper(self.numerator, self.denominator, other)
    }
}

impl Compose<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Composes two [`RationalPolynomial`]s, giving $p(q(x))$, taking the first by reference and
    /// the second by value.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Writing $q = Q/e$ with $Q$ an integer polynomial and $e$ a positive integer, $p(q)$ is
    /// $p(x/e)$ evaluated at $Q$. The numerator of $p(x/e)$, scaled to clear its denominators, is
    /// an integer polynomial, so the work is one integer composition, with the numerators reduced
    /// to lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. A constant $p$ is returned as it is. If $q$ is not constant and $p$ is nonzero, the
    /// degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// numerator coefficient or denominator of either polynomial: $s$ bounds the size of the
    /// numerator coefficients of the integer composition.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3").unwrap())
    ///         .compose(RationalPolynomial::from_str("2*x+1/2").unwrap())
    ///         .to_string(),
    ///     "2*x^2+x+11/24"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^3").unwrap())
    ///         .compose(RationalPolynomial::from_str("x^2+1/2*x").unwrap())
    ///         .to_string(),
    ///     "x^6+3/2*x^5+3/4*x^4+1/8*x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2-1/3*x+1/2").unwrap())
    ///         .compose(RationalPolynomial::from_str("5/2").unwrap())
    ///         .to_string(),
    ///     "71/12"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_compose` from `fmpq_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: RationalPolynomial) -> RationalPolynomial {
        compose_helper(self.numerator.clone(), self.denominator.clone(), &other)
    }
}

impl Compose<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Composes two [`RationalPolynomial`]s, giving $p(q(x))$, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Writing $q = Q/e$ with $Q$ an integer polynomial and $e$ a positive integer, $p(q)$ is
    /// $p(x/e)$ evaluated at $Q$. The numerator of $p(x/e)$, scaled to clear its denominators, is
    /// an integer polynomial, so the work is one integer composition, with the numerators reduced
    /// to lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. A constant $p$ is returned as it is. If $q$ is not constant and $p$ is nonzero, the
    /// degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// numerator coefficient or denominator of either polynomial: $s$ bounds the size of the
    /// numerator coefficients of the integer composition.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3").unwrap())
    ///         .compose(&RationalPolynomial::from_str("2*x+1/2").unwrap())
    ///         .to_string(),
    ///     "2*x^2+x+11/24"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^3").unwrap())
    ///         .compose(&RationalPolynomial::from_str("x^2+1/2*x").unwrap())
    ///         .to_string(),
    ///     "x^6+3/2*x^5+3/4*x^4+1/8*x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2-1/3*x+1/2").unwrap())
    ///         .compose(&RationalPolynomial::from_str("5/2").unwrap())
    ///         .to_string(),
    ///     "71/12"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_compose` from `fmpq_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: &RationalPolynomial) -> RationalPolynomial {
        compose_helper(self.numerator.clone(), self.denominator.clone(), other)
    }
}

impl ComposeAssign<Self> for RationalPolynomial {
    /// Composes a [`RationalPolynomial`] with another in place, replacing $p$ with $p(q(x))$,
    /// taking the other by value.
    ///
    /// $$
    /// p \\gets p(q(x)).
    /// $$
    ///
    /// Writing $q = Q/e$ with $Q$ an integer polynomial and $e$ a positive integer, $p(q)$ is
    /// $p(x/e)$ evaluated at $Q$. The numerator of $p(x/e)$, scaled to clear its denominators, is
    /// an integer polynomial, so the work is one integer composition, with the numerators reduced
    /// to lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. A constant $p$ is returned as it is. If $q$ is not constant and $p$ is nonzero, the
    /// degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// numerator coefficient or denominator of either polynomial: $s$ bounds the size of the
    /// numerator coefficients of the integer composition.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// p.compose_assign(RationalPolynomial::from_str("2*x+1/2").unwrap());
    /// assert_eq!(p.to_string(), "2*x^2+x+11/24");
    ///
    /// let mut p = RationalPolynomial::from_str("x^3").unwrap();
    /// p.compose_assign(RationalPolynomial::from_str("x^2+1/2*x").unwrap());
    /// assert_eq!(p.to_string(), "x^6+3/2*x^5+3/4*x^4+1/8*x^3");
    ///
    /// // Composing with a constant evaluates.
    ///
    /// let mut p = RationalPolynomial::from_str("x^2-1/3*x+1/2").unwrap();
    /// p.compose_assign(RationalPolynomial::from_str("5/2").unwrap());
    /// assert_eq!(p.to_string(), "71/12");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_compose` from `fmpq_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose_assign(&mut self, other: Self) {
        *self = compose_helper(
            take(&mut self.numerator),
            take(&mut self.denominator),
            &other,
        );
    }
}

impl ComposeAssign<&Self> for RationalPolynomial {
    /// Composes a [`RationalPolynomial`] with another in place, replacing $p$ with $p(q(x))$,
    /// taking the other by reference.
    ///
    /// $$
    /// p \\gets p(q(x)).
    /// $$
    ///
    /// Writing $q = Q/e$ with $Q$ an integer polynomial and $e$ a positive integer, $p(q)$ is
    /// $p(x/e)$ evaluated at $Q$. The numerator of $p(x/e)$, scaled to clear its denominators, is
    /// an integer polynomial, so the work is one integer composition, with the numerators reduced
    /// to lowest terms before and after it; when $e$ is 1, the scaling and the first reduction are
    /// skipped. A constant $p$ is returned as it is. If $q$ is not constant and $p$ is nonzero, the
    /// degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// numerator coefficient or denominator of either polynomial: $s$ bounds the size of the
    /// numerator coefficients of the integer composition.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// p.compose_assign(&RationalPolynomial::from_str("2*x+1/2").unwrap());
    /// assert_eq!(p.to_string(), "2*x^2+x+11/24");
    ///
    /// let mut p = RationalPolynomial::from_str("x^3").unwrap();
    /// p.compose_assign(&RationalPolynomial::from_str("x^2+1/2*x").unwrap());
    /// assert_eq!(p.to_string(), "x^6+3/2*x^5+3/4*x^4+1/8*x^3");
    ///
    /// // Composing with a constant evaluates.
    ///
    /// let mut p = RationalPolynomial::from_str("x^2-1/3*x+1/2").unwrap();
    /// p.compose_assign(&RationalPolynomial::from_str("5/2").unwrap());
    /// assert_eq!(p.to_string(), "71/12");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_compose` from `fmpq_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose_assign(&mut self, other: &Self) {
        *self = compose_helper(
            take(&mut self.numerator),
            take(&mut self.denominator),
            other,
        );
    }
}
