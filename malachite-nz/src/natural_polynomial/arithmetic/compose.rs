// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::compose::compose_val;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::polynomial::{Compose, ComposeAssign, Polynomial};

// Composes the polynomial with coefficients `xs` with the one with coefficients `ys`, with the
// `Integer` algorithms. A `Natural` becomes an `Integer`, and a non-negative `Integer` becomes a
// `Natural` again, without copying its limbs, and composing polynomials with non-negative
// coefficients leaves every intermediate coefficient non-negative.
fn compose_naturals(xs: Vec<Integer>, ys: &[Integer]) -> NaturalPolynomial {
    NaturalPolynomial {
        coefficients: compose_val(xs, ys)
            .into_coefficients_asc()
            .into_iter()
            .map(UnsignedAbs::unsigned_abs)
            .collect(),
    }
}

fn integers_from_naturals(xs: Vec<Natural>) -> Vec<Integer> {
    xs.into_iter().map(Integer::from).collect()
}

fn integers_from_natural_refs(xs: &[Natural]) -> Vec<Integer> {
    xs.iter().map(Integer::from).collect()
}

impl Compose<Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s, giving $p(q(x))$, taking both by value.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Every coefficient of the result is a sum of products of coefficients of $p$ and $q$, so it
    /// is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): when $q$ is a constant
    /// $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift
    /// by $a$ followed by scaling the coefficient of $x^i$ by $b^i$; otherwise short polynomials
    /// are composed with Horner's rule and longer ones by divide and conquer. If $q$ is not
    /// constant and $p$ is nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .compose(NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+26*x+42"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .compose(NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+1")
    ///         .unwrap()
    ///         .compose(NaturalPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    #[inline]
    fn compose(self, other: Self) -> Self {
        compose_naturals(
            integers_from_naturals(self.coefficients),
            &integers_from_naturals(other.coefficients),
        )
    }
}

impl Compose<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s, giving $p(q(x))$, taking the first by value and the
    /// second by reference.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Every coefficient of the result is a sum of products of coefficients of $p$ and $q$, so it
    /// is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): when $q$ is a constant
    /// $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift
    /// by $a$ followed by scaling the coefficient of $x^i$ by $b^i$; otherwise short polynomials
    /// are composed with Horner's rule and longer ones by divide and conquer. If $q$ is not
    /// constant and $p$ is nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .compose(&NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+26*x+42"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .compose(&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+1")
    ///         .unwrap()
    ///         .compose(&NaturalPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    #[inline]
    fn compose(self, other: &Self) -> Self {
        compose_naturals(
            integers_from_naturals(self.coefficients),
            &integers_from_natural_refs(&other.coefficients),
        )
    }
}

impl Compose<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s, giving $p(q(x))$, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Every coefficient of the result is a sum of products of coefficients of $p$ and $q$, so it
    /// is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): when $q$ is a constant
    /// $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift
    /// by $a$ followed by scaling the coefficient of $x^i$ by $b^i$; otherwise short polynomials
    /// are composed with Horner's rule and longer ones by divide and conquer. If $q$ is not
    /// constant and $p$ is nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .compose(NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+26*x+42"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3").unwrap())
    ///         .compose(NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+1").unwrap())
    ///         .compose(NaturalPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    #[inline]
    fn compose(self, other: NaturalPolynomial) -> NaturalPolynomial {
        compose_naturals(
            integers_from_natural_refs(&self.coefficients),
            &integers_from_naturals(other.coefficients),
        )
    }
}

impl Compose<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s, giving $p(q(x))$, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// Every coefficient of the result is a sum of products of coefficients of $p$ and $q$, so it
    /// is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): when $q$ is a constant
    /// $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift
    /// by $a$ followed by scaling the coefficient of $x^i$ by $b^i$; otherwise short polynomials
    /// are composed with Horner's rule and longer ones by divide and conquer. If $q$ is not
    /// constant and $p$ is nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .compose(&NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+26*x+42"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3").unwrap())
    ///         .compose(&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+1").unwrap())
    ///         .compose(&NaturalPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    #[inline]
    fn compose(self, other: &NaturalPolynomial) -> NaturalPolynomial {
        compose_naturals(
            integers_from_natural_refs(&self.coefficients),
            &integers_from_natural_refs(&other.coefficients),
        )
    }
}

impl ComposeAssign<Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another in place, replacing $p$ with $p(q(x))$, taking
    /// the other by value.
    ///
    /// $$
    /// p \\gets p(q(x)).
    /// $$
    ///
    /// Every coefficient of the result is a sum of products of coefficients of $p$ and $q$, so it
    /// is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): when $q$ is a constant
    /// $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift
    /// by $a$ followed by scaling the coefficient of $x^i$ by $b^i$; otherwise short polynomials
    /// are composed with Horner's rule and longer ones by divide and conquer. If $q$ is not
    /// constant and $p$ is nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.compose_assign(NaturalPolynomial::from_str("2*x+5").unwrap());
    /// assert_eq!(p.to_string(), "4*x^2+26*x+42");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.compose_assign(NaturalPolynomial::from_str("x^2+x").unwrap());
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Composing with a constant evaluates.
    /// let mut p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// p.compose_assign(NaturalPolynomial::from_str("3").unwrap());
    /// assert_eq!(p.to_string(), "10");
    /// ```
    #[inline]
    fn compose_assign(&mut self, other: Self) {
        *self = compose_naturals(
            integers_from_naturals(take(&mut self.coefficients)),
            &integers_from_naturals(other.coefficients),
        );
    }
}

impl ComposeAssign<&Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another in place, replacing $p$ with $p(q(x))$, taking
    /// the other by reference.
    ///
    /// $$
    /// p \\gets p(q(x)).
    /// $$
    ///
    /// Every coefficient of the result is a sum of products of coefficients of $p$ and $q$, so it
    /// is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial): when $q$ is a constant
    /// $c$, the result is the constant $p(c)$; when $q$ is linear, $bx + a$, it is a Taylor shift
    /// by $a$ followed by scaling the coefficient of $x^i$ by $b^i$; otherwise short polynomials
    /// are composed with Horner's rule and longer ones by divide and conquer. If $q$ is not
    /// constant and $p$ is nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.compose_assign(&NaturalPolynomial::from_str("2*x+5").unwrap());
    /// assert_eq!(p.to_string(), "4*x^2+26*x+42");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.compose_assign(&NaturalPolynomial::from_str("x^2+x").unwrap());
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Composing with a constant evaluates.
    /// let mut p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// p.compose_assign(&NaturalPolynomial::from_str("3").unwrap());
    /// assert_eq!(p.to_string(), "10");
    /// ```
    #[inline]
    fn compose_assign(&mut self, other: &Self) {
        *self = compose_naturals(
            integers_from_naturals(take(&mut self.coefficients)),
            &integers_from_natural_refs(&other.coefficients),
        );
    }
}
