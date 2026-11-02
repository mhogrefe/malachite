// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2013 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::{RationalPolynomial, reduce};
use core::mem::take;
use malachite_base::num::arithmetic::traits::{CanonicalPrimitivePart, Pow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{RemPrecomputed, RemPrecomputedAssign};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::rem_precomputed::{
    RemPrecomputedData, precompute_rem_powers, rem_by_powers,
};

// The remainder by `q` equals the remainder by the primitive part of its numerator, with its
// leading coefficient made positive, so the powers are those of that integer polynomial.
fn precompute_helper(q: &RationalPolynomial) -> RemPrecomputedData {
    assert_ne!(
        *q,
        RationalPolynomial::ZERO,
        "cannot divide by the zero polynomial"
    );
    precompute_rem_powers((&q.numerator).canonical_primitive_part().coefficients_asc())
}

// For p = A/d, the powers give ell^k (A mod q) as an integer polynomial, so the remainder is that
// over d * ell^k, reduced once.
//
// This is equivalent to `fmpq_poly_rem_powers_precomp` from `fmpq_poly/rem_powers_precomp.c`, FLINT
// 3.6.0, except that the remainder is kept in lowest terms and that a dividend with at least twice
// as many coefficients as the divisor is reduced with the powers too.
fn rem_precomputed_helper(
    p: RationalPolynomial,
    q: &RationalPolynomial,
    data: &RemPrecomputedData,
) -> RationalPolynomial {
    let n = q.len();
    assert_eq!(
        n,
        data.divisor_len(),
        "the data was not precomputed from the divisor"
    );
    if p.len() < n {
        return p;
    }
    let RationalPolynomial {
        numerator,
        mut denominator,
    } = p;
    let (xs, k) = rem_by_powers(numerator.into_coefficients_asc(), data);
    if k != 0 && *data.leading_coefficient() != 1u32 {
        denominator *= data.leading_coefficient().unsigned_abs_ref().pow(k);
    }
    reduce(IntegerPolynomial::from_coefficients_asc(xs), denominator)
}

impl RemPrecomputed<Self> for RationalPolynomial {
    type Output = Self;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`RationalPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign).
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`, kept as integer polynomials: the numerator of `other` is replaced by its
    /// primitive part, with leading coefficient $\ell > 0$, and each power is scaled by a power of
    /// $\ell$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.rem_precomputed(q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_powers_precompute` from `fmpq_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that the powers are kept as integer polynomials.
    #[inline]
    fn precompute_rem_data(other: &Self) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`RationalPolynomial`]s, using data precomputed from the divisor,
    /// taking both by value.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, scaled to
    /// integer polynomials. A dividend with fewer than $2n$ coefficients is reduced by adding a
    /// multiple of one power for each numerator coefficient from $x^{n-1}$ up, and a longer one $n$
    /// coefficients at a time, from the top; the result is brought to lowest terms once, at the
    /// end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator and denominator of `self` and in the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.rem_precomputed(q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_rem_powers_precomp` from `fmpq_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that the remainder is in lowest terms and that a dividend with at least
    /// twice as many coefficients as the divisor is also reduced with the powers.
    #[inline]
    fn rem_precomputed(self, other: Self, data: &RemPrecomputedData) -> Self {
        rem_precomputed_helper(self, &other, data)
    }
}

impl RemPrecomputed<&Self> for RationalPolynomial {
    type Output = Self;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`RationalPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign).
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`, kept as integer polynomials: the numerator of `other` is replaced by its
    /// primitive part, with leading coefficient $\ell > 0$, and each power is scaled by a power of
    /// $\ell$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.rem_precomputed(q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_powers_precompute` from `fmpq_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that the powers are kept as integer polynomials.
    #[inline]
    fn precompute_rem_data(other: &&Self) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`RationalPolynomial`]s, using data precomputed from the divisor,
    /// taking the first by value and the second by reference.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, scaled to
    /// integer polynomials. A dividend with fewer than $2n$ coefficients is reduced by adding a
    /// multiple of one power for each numerator coefficient from $x^{n-1}$ up, and a longer one $n$
    /// coefficients at a time, from the top; the result is brought to lowest terms once, at the
    /// end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator and denominator of `self` and in the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.rem_precomputed(&q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!(p.rem_precomputed(&q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_rem_powers_precomp` from `fmpq_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that the remainder is in lowest terms and that a dividend with at least
    /// twice as many coefficients as the divisor is also reduced with the powers.
    #[inline]
    fn rem_precomputed(self, other: &Self, data: &RemPrecomputedData) -> Self {
        rem_precomputed_helper(self, other, data)
    }
}

impl RemPrecomputed<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`RationalPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign).
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`, kept as integer polynomials: the numerator of `other` is replaced by its
    /// primitive part, with leading coefficient $\ell > 0$, and each power is scaled by a power of
    /// $\ell$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.rem_precomputed(q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_powers_precompute` from `fmpq_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that the powers are kept as integer polynomials.
    #[inline]
    fn precompute_rem_data(other: &RationalPolynomial) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`RationalPolynomial`]s, using data precomputed from the divisor,
    /// taking the first by reference and the second by value.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, scaled to
    /// integer polynomials. A dividend with fewer than $2n$ coefficients is reduced by adding a
    /// multiple of one power for each numerator coefficient from $x^{n-1}$ up, and a longer one $n$
    /// coefficients at a time, from the top; the result is brought to lowest terms once, at the
    /// end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator and denominator of `self` and in the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).rem_precomputed(q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!((&p).rem_precomputed(q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_rem_powers_precomp` from `fmpq_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that the remainder is in lowest terms and that a dividend with at least
    /// twice as many coefficients as the divisor is also reduced with the powers.
    #[inline]
    fn rem_precomputed(
        self,
        other: RationalPolynomial,
        data: &RemPrecomputedData,
    ) -> RationalPolynomial {
        rem_precomputed_helper(self.clone(), &other, data)
    }
}

impl RemPrecomputed<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;
    type Data = RemPrecomputedData;

    /// Precomputes the data for dividing [`RationalPolynomial`]s by `other` with
    /// [`rem_precomputed`](RemPrecomputed::rem_precomputed) or
    /// [`rem_precomputed_assign`](RemPrecomputedAssign::rem_precomputed_assign).
    ///
    /// The data is the powers $x^{n-1}, x^n, \ldots, x^{2n-2}$ reduced modulo `other`, where $n$ is
    /// `other.len()`, kept as integer polynomials: the numerator of `other` is replaced by its
    /// primitive part, with leading coefficient $\ell > 0$, and each power is scaled by a power of
    /// $\ell$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator of `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.rem_precomputed(q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!(p.rem_precomputed(q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_powers_precompute` from `fmpq_poly/powers_precompute.c`,
    /// FLINT 3.6.0, except that the powers are kept as integer polynomials.
    #[inline]
    fn precompute_rem_data(other: &&RationalPolynomial) -> RemPrecomputedData {
        precompute_helper(other)
    }

    /// Finds the remainder of two [`RationalPolynomial`]s, using data precomputed from the divisor,
    /// taking both by reference.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, scaled to
    /// integer polynomials. A dividend with fewer than $2n$ coefficients is reduced by adding a
    /// multiple of one power for each numerator coefficient from $x^{n-1}$ up, and a longer one $n$
    /// coefficients at a time, from the top; the result is brought to lowest terms once, at the
    /// end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator and denominator of `self` and in the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::RemPrecomputed;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).rem_precomputed(&q.clone(), &data).to_string(),
    ///     "49/16*x+19/16"
    /// );
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// assert_eq!((&p).rem_precomputed(&q, &data).to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_rem_powers_precomp` from `fmpq_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that the remainder is in lowest terms and that a dividend with at least
    /// twice as many coefficients as the divisor is also reduced with the powers.
    #[inline]
    fn rem_precomputed(
        self,
        other: &RationalPolynomial,
        data: &RemPrecomputedData,
    ) -> RationalPolynomial {
        rem_precomputed_helper(self.clone(), other, data)
    }
}

impl RemPrecomputedAssign<Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its remainder by another, using data precomputed from
    /// the divisor, taking the divisor by value.
    ///
    /// $$
    /// a \gets r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, scaled to
    /// integer polynomials. A dividend with fewer than $2n$ coefficients is reduced by adding a
    /// multiple of one power for each numerator coefficient from $x^{n-1}$ up, and a longer one $n$
    /// coefficients at a time, from the top; the result is brought to lowest terms once, at the
    /// end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator and denominator of `self` and in the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::{
    ///     RemPrecomputed, RemPrecomputedAssign,
    /// };
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let mut p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.rem_precomputed_assign(q.clone(), &data);
    /// assert_eq!(p.to_string(), "49/16*x+19/16");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// p.rem_precomputed_assign(q, &data);
    /// assert_eq!(p.to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_rem_powers_precomp` from `fmpq_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that the remainder is in lowest terms and that a dividend with at least
    /// twice as many coefficients as the divisor is also reduced with the powers.
    #[inline]
    fn rem_precomputed_assign(&mut self, other: Self, data: &RemPrecomputedData) {
        *self = rem_precomputed_helper(take(self), &other, data);
    }
}

impl RemPrecomputedAssign<&Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its remainder by another, using data precomputed from
    /// the divisor, taking the divisor by reference.
    ///
    /// $$
    /// a \gets r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The data should be obtained with
    /// [`precompute_rem_data`](RemPrecomputed::precompute_rem_data). It holds the powers $x^{n-1},
    /// \ldots, x^{2n-2}$ reduced modulo the divisor, where $n$ is the divisor's length, scaled to
    /// integer polynomials. A dividend with fewer than $2n$ coefficients is reduced by adding a
    /// multiple of one power for each numerator coefficient from $x^{n-1}$ up, and a longer one $n$
    /// coefficients at a time, from the top; the result is brought to lowest terms once, at the
    /// end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerator and denominator of `self` and in the precomputed powers.
    ///
    /// # Panics
    /// Panics if `data` was precomputed from a divisor with a different length. If `data` was not
    /// precomputed from the divisor, the result is meaningless.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::{
    ///     RemPrecomputed, RemPrecomputedAssign,
    /// };
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let q = RationalPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let data = RationalPolynomial::precompute_rem_data(&q);
    ///
    /// let mut p = RationalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.rem_precomputed_assign(&q.clone(), &data);
    /// assert_eq!(p.to_string(), "49/16*x+19/16");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^3+2*x").unwrap();
    /// p.rem_precomputed_assign(&q, &data);
    /// assert_eq!(p.to_string(), "15/8*x+1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_rem_powers_precomp` from `fmpq_poly/rem_powers_precomp.c`,
    /// FLINT 3.6.0, except that the remainder is in lowest terms and that a dividend with at least
    /// twice as many coefficients as the divisor is also reduced with the powers.
    #[inline]
    fn rem_precomputed_assign(&mut self, other: &Self, data: &RemPrecomputedData) {
        *self = rem_precomputed_helper(take(self), other, data);
    }
}
