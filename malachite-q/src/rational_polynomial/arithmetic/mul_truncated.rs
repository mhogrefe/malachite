// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::mem::take;
use core::ptr;
use malachite_base::num::arithmetic::traits::DivExactAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign, SquareTruncated};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_vector::arithmetic::content_chained::integers_content_chained;
use malachite_nz::natural::Natural;

// Divides out whatever `numerator` shares with `denominator`, which is positive, giving a canonical
// `RationalPolynomial`. A zero numerator gets the denominator 1, since the content of zero is taken
// to be 0.
//
// This is equivalent to `fmpq_poly_canonicalise` from `fmpq_poly/canonicalise.c`, FLINT 3.6.0.
pub(crate) fn canonicalize_truncated(
    mut numerator: IntegerPolynomial,
    mut denominator: Natural,
) -> RationalPolynomial {
    if denominator != 1u32 {
        let g = integers_content_chained(numerator.coefficients_asc(), &denominator);
        if g != 1u32 {
            numerator.div_exact_assign(Integer::from(&g));
            denominator.div_exact_assign(g);
        }
    }
    RationalPolynomial {
        numerator,
        denominator,
    }
}

impl MulTruncated<Self> for RationalPolynomial {
    type Output = Self;

    /// Multiplies two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$. Truncation can leave the numerator sharing a factor with the denominator, so the
    /// result is reduced afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The product is 1/2*x^2+1/12*x-1/6; its low two coefficients.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3")
    ///         .unwrap()
    ///         .mul_truncated(RationalPolynomial::from_str("x-1/2").unwrap(), 2)
    ///         .to_string(),
    ///     "1/12*x-1/6"
    /// );
    /// // The product is 1/2*x^3+2*x^2+3*x+2; once the cubic term is cut, nothing is left over 2.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+x+1")
    ///         .unwrap()
    ///         .mul_truncated(RationalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0.
    fn mul_truncated(self, other: Self, len: u64) -> Self {
        if self == Self::ZERO || other == Self::ZERO || len == 0 {
            return Self::ZERO;
        }
        canonicalize_truncated(
            self.numerator.mul_truncated(other.numerator, len),
            self.denominator * other.denominator,
        )
    }
}

impl MulTruncated<&Self> for RationalPolynomial {
    type Output = Self;

    /// Multiplies two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$. Truncation can leave the numerator sharing a factor with the denominator, so the
    /// result is reduced afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The product is 1/2*x^2+1/12*x-1/6; its low two coefficients.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3")
    ///         .unwrap()
    ///         .mul_truncated(&RationalPolynomial::from_str("x-1/2").unwrap(), 2)
    ///         .to_string(),
    ///     "1/12*x-1/6"
    /// );
    /// // The product is 1/2*x^3+2*x^2+3*x+2; once the cubic term is cut, nothing is left over 2.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+x+1")
    ///         .unwrap()
    ///         .mul_truncated(&RationalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0.
    fn mul_truncated(self, other: &Self, len: u64) -> Self {
        if self == Self::ZERO || *other == Self::ZERO || len == 0 {
            return Self::ZERO;
        }
        canonicalize_truncated(
            self.numerator.mul_truncated(&other.numerator, len),
            self.denominator * &other.denominator,
        )
    }
}

impl MulTruncated<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Multiplies two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$. Truncation can leave the numerator sharing a factor with the denominator, so the
    /// result is reduced afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The product is 1/2*x^2+1/12*x-1/6; its low two coefficients.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .mul_truncated(RationalPolynomial::from_str("x-1/2").unwrap(), 2)
    ///         .to_string(),
    ///     "1/12*x-1/6"
    /// );
    /// // The product is 1/2*x^3+2*x^2+3*x+2; once the cubic term is cut, nothing is left over 2.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+x+1").unwrap())
    ///         .mul_truncated(RationalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: RationalPolynomial, len: u64) -> RationalPolynomial {
        other.mul_truncated(self, len)
    }
}

impl MulTruncated<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Multiplies two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$. Truncation can leave the numerator sharing a factor with the denominator, so the
    /// result is reduced afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The product is 1/2*x^2+1/12*x-1/6; its low two coefficients.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .mul_truncated(&RationalPolynomial::from_str("x-1/2").unwrap(), 2)
    ///         .to_string(),
    ///     "1/12*x-1/6"
    /// );
    /// // The product is 1/2*x^3+2*x^2+3*x+2; once the cubic term is cut, nothing is left over 2.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+x+1").unwrap())
    ///         .mul_truncated(&RationalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0.
    fn mul_truncated(self, other: &RationalPolynomial, len: u64) -> RationalPolynomial {
        // A product with itself is a square.
        if ptr::eq(self, other) {
            return self.square_truncated(len);
        }
        if *self == RationalPolynomial::ZERO || *other == RationalPolynomial::ZERO || len == 0 {
            return RationalPolynomial::ZERO;
        }
        canonicalize_truncated(
            (&self.numerator).mul_truncated(&other.numerator, len),
            &self.denominator * &other.denominator,
        )
    }
}

impl MulTruncatedAssign<Self> for RationalPolynomial {
    /// Multiplies a [`RationalPolynomial`] by another [`RationalPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by value.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+x+1").unwrap();
    /// p.mul_truncated_assign(RationalPolynomial::from_str("x+2").unwrap(), 2);
    /// assert_eq!(p.to_string(), "3*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0.
    fn mul_truncated_assign(&mut self, other: Self, len: u64) {
        *self = take(self).mul_truncated(other, len);
    }
}

impl MulTruncatedAssign<&Self> for RationalPolynomial {
    /// Multiplies a [`RationalPolynomial`] by another [`RationalPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by reference.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or denominator of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+x+1").unwrap();
    /// p.mul_truncated_assign(&RationalPolynomial::from_str("x+2").unwrap(), 2);
    /// assert_eq!(p.to_string(), "3*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0.
    fn mul_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = take(self).mul_truncated(other, len);
    }
}
