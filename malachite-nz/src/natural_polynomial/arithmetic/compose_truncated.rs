// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::compose_truncated::compose_truncated_ref;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use core::cmp::max;
use core::mem::take;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::polynomial::{ComposeTruncated, ComposeTruncatedAssign, Polynomial};

// Composes the polynomials with coefficients `xs` and `ys` as power series, keeping the first `len`
// coefficients, with the `Integer` algorithms. A non-negative `Integer` becomes a `Natural` again
// without copying its limbs; the result is non-negative, though intermediate values may not be.
fn compose_truncated_naturals(xs: &[Integer], ys: &[Integer], len: u64) -> NaturalPolynomial {
    NaturalPolynomial {
        coefficients: compose_truncated_ref(xs, ys, len)
            .into_coefficients_asc()
            .into_iter()
            .map(UnsignedAbs::unsigned_abs)
            .collect(),
    }
}

// The number of leading coefficients to keep: the first `len`, which are all that can affect the
// result, but at least 1, so that a nonzero constant coefficient of the inner polynomial is still
// rejected.
fn kept_len(len: u64) -> usize {
    max(usize::try_from(len).unwrap_or(usize::MAX), 1)
}

// The first `kept_len(len)` coefficients of a polynomial, as `Integer`s.
fn integers_from_naturals(mut xs: Vec<Natural>, len: u64) -> Vec<Integer> {
    xs.truncate(kept_len(len));
    xs.into_iter().map(Integer::from).collect()
}

fn integers_from_natural_refs(xs: &[Natural], len: u64) -> Vec<Integer> {
    xs.iter().take(kept_len(len)).map(Integer::from).collect()
}

impl ComposeTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result. Every coefficient of the result is a sum of products
    /// of coefficients of $p$ and $q$, so it is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial); see
    /// [`compose_truncated`](ComposeTruncated::compose_truncated). The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("x^2+x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: Self, len: u64) -> Self {
        compose_truncated_naturals(
            &integers_from_naturals(self.coefficients, len),
            &integers_from_naturals(other.coefficients, len),
            len,
        )
    }
}

impl ComposeTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result. Every coefficient of the result is a sum of products
    /// of coefficients of $p$ and $q$, so it is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial); see
    /// [`compose_truncated`](ComposeTruncated::compose_truncated). The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("x^2+x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: &Self, len: u64) -> Self {
        compose_truncated_naturals(
            &integers_from_naturals(self.coefficients, len),
            &integers_from_natural_refs(&other.coefficients, len),
            len,
        )
    }
}

impl ComposeTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result. Every coefficient of the result is a sum of products
    /// of coefficients of $p$ and $q$, so it is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial); see
    /// [`compose_truncated`](ComposeTruncated::compose_truncated). The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("x^2+x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(NaturalPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: NaturalPolynomial, len: u64) -> NaturalPolynomial {
        compose_truncated_naturals(
            &integers_from_natural_refs(&self.coefficients, len),
            &integers_from_naturals(other.coefficients, len),
            len,
        )
    }
}

impl ComposeTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result. Every coefficient of the result is a sum of products
    /// of coefficients of $p$ and $q$, so it is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial); see
    /// [`compose_truncated`](ComposeTruncated::compose_truncated). The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("x^2+x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&NaturalPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: &NaturalPolynomial, len: u64) -> NaturalPolynomial {
        compose_truncated_naturals(
            &integers_from_natural_refs(&self.coefficients, len),
            &integers_from_natural_refs(&other.coefficients, len),
            len,
        )
    }
}

impl ComposeTruncatedAssign<Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another in place and truncates the result to its first
    /// `len` coefficients, taking the second by value.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result. Every coefficient of the result is a sum of products
    /// of coefficients of $p$ and $q$, so it is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial); see
    /// [`compose_truncated`](ComposeTruncated::compose_truncated). The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(NaturalPolynomial::from_str("x^2+x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+3");
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// let mut p = NaturalPolynomial::from_str("x^3+x^2+x+1").unwrap();
    /// p.compose_truncated_assign(NaturalPolynomial::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "4*x^2+2*x+1");
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// let mut p = NaturalPolynomial::from_str("x^4+x+1").unwrap();
    /// p.compose_truncated_assign(NaturalPolynomial::from_str("x^2+x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "x^2+x+1");
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(NaturalPolynomial::from_str("x^2+x").unwrap(), 0);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    #[inline]
    fn compose_truncated_assign(&mut self, other: Self, len: u64) {
        let xs = integers_from_naturals(take(&mut self.coefficients), len);
        *self =
            compose_truncated_naturals(&xs, &integers_from_naturals(other.coefficients, len), len);
    }
}

impl ComposeTruncatedAssign<&Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another in place and truncates the result to its first
    /// `len` coefficients, taking the second by reference.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result. Every coefficient of the result is a sum of products
    /// of coefficients of $p$ and $q$, so it is non-negative. The composition is done as for an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial); see
    /// [`compose_truncated`](ComposeTruncated::compose_truncated). The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(&NaturalPolynomial::from_str("x^2+x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+3");
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// let mut p = NaturalPolynomial::from_str("x^3+x^2+x+1").unwrap();
    /// p.compose_truncated_assign(&NaturalPolynomial::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "4*x^2+2*x+1");
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// let mut p = NaturalPolynomial::from_str("x^4+x+1").unwrap();
    /// p.compose_truncated_assign(&NaturalPolynomial::from_str("x^2+x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "x^2+x+1");
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(&NaturalPolynomial::from_str("x^2+x").unwrap(), 0);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    #[inline]
    fn compose_truncated_assign(&mut self, other: &Self, len: u64) {
        let xs = integers_from_naturals(take(&mut self.coefficients), len);
        *self = compose_truncated_naturals(
            &xs,
            &integers_from_natural_refs(&other.coefficients, len),
            len,
        );
    }
}
