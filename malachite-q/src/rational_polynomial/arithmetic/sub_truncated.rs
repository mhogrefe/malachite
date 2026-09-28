// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::add_truncated::{
    add_or_sub_truncated_owned_ref, add_or_sub_truncated_ref_ref,
};
use core::mem::take;
use core::ptr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Polynomial, SubTruncated, SubTruncatedAssign};

// Computes $q - p$ keeping only the coefficients below $x^{len}$, reusing the storage of $q$, then
// negates it to give $p - q$, as FLINT does when the output aliases the second operand.
fn sub_truncated_ref_owned(
    p: &RationalPolynomial,
    q: RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    -add_or_sub_truncated_owned_ref(q, p, len, true)
}

// Subtracts two `RationalPolynomial`s taken by value, keeping only the coefficients below $x^{len}$
// and reusing the storage of the one with the longer numerator.
fn sub_truncated_owned_owned(
    p: RationalPolynomial,
    q: RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    if q.numerator.len() > p.numerator.len() {
        sub_truncated_ref_owned(&p, q, len)
    } else {
        add_or_sub_truncated_owned_ref(p, &q, len, true)
    }
}

impl SubTruncated<Self> for RationalPolynomial {
    type Output = Self;

    /// Subtracts one [`RationalPolynomial`] from another, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is kept in
    /// lowest terms: cutting a polynomial can remove the coefficients that kept its numerator
    /// coprime to its denominator, so when anything is cut, the whole common factor of the new
    /// numerator and denominator is divided out. The difference is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The quadratic terms cancel.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .sub_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "-1/3*x+1/2"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .sub_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             1
    ///         )
    ///         .to_string(),
    ///     "1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub_series` from `fmpq_poly/sub_series.c`, FLINT 3.6.0.
    #[inline]
    fn sub_truncated(self, other: Self, len: u64) -> Self {
        sub_truncated_owned_owned(self, other, len)
    }
}

impl SubTruncated<&Self> for RationalPolynomial {
    type Output = Self;

    /// Subtracts one [`RationalPolynomial`] from another, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is kept in
    /// lowest terms: cutting a polynomial can remove the coefficients that kept its numerator
    /// coprime to its denominator, so when anything is cut, the whole common factor of the new
    /// numerator and denominator is divided out. The difference is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The quadratic terms cancel.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .sub_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "-1/3*x+1/2"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .sub_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             1
    ///         )
    ///         .to_string(),
    ///     "1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub_series` from `fmpq_poly/sub_series.c`, FLINT 3.6.0.
    #[inline]
    fn sub_truncated(self, other: &Self, len: u64) -> Self {
        add_or_sub_truncated_owned_ref(self, other, len, true)
    }
}

impl SubTruncated<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Subtracts one [`RationalPolynomial`] from another, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is kept in
    /// lowest terms: cutting a polynomial can remove the coefficients that kept its numerator
    /// coprime to its denominator, so when anything is cut, the whole common factor of the new
    /// numerator and denominator is divided out. The difference is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The quadratic terms cancel.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .sub_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "-1/3*x+1/2"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .sub_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             1
    ///         )
    ///         .to_string(),
    ///     "1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub_series` from `fmpq_poly/sub_series.c`, FLINT 3.6.0.
    #[inline]
    fn sub_truncated(self, other: RationalPolynomial, len: u64) -> RationalPolynomial {
        sub_truncated_ref_owned(self, other, len)
    }
}

impl SubTruncated<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Subtracts one [`RationalPolynomial`] from another, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is kept in
    /// lowest terms: cutting a polynomial can remove the coefficients that kept its numerator
    /// coprime to its denominator, so when anything is cut, the whole common factor of the new
    /// numerator and denominator is divided out. The difference is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The quadratic terms cancel.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .sub_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "-1/3*x+1/2"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .sub_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             1
    ///         )
    ///         .to_string(),
    ///     "1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub_series` from `fmpq_poly/sub_series.c`, FLINT 3.6.0.
    fn sub_truncated(self, other: &RationalPolynomial, len: u64) -> RationalPolynomial {
        if ptr::eq(self, other) {
            return RationalPolynomial::ZERO;
        }
        add_or_sub_truncated_ref_ref(self, other, len, true)
    }
}

impl SubTruncatedAssign<Self> for RationalPolynomial {
    /// Subtracts a [`RationalPolynomial`] from a [`RationalPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by value.
    ///
    /// $$
    /// p \gets (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is kept in
    /// lowest terms: cutting a polynomial can remove the coefficients that kept its numerator
    /// coprime to its denominator, so when anything is cut, the whole common factor of the new
    /// numerator and denominator is divided out. The difference is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The quadratic terms cancel.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.sub_truncated_assign(
    ///     RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "-1/3*x+1/2");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.sub_truncated_assign(
    ///     RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     1,
    /// );
    /// assert_eq!(p.to_string(), "1/2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub_series` from `fmpq_poly/sub_series.c`, FLINT 3.6.0.
    #[inline]
    fn sub_truncated_assign(&mut self, other: Self, len: u64) {
        *self = sub_truncated_owned_owned(take(self), other, len);
    }
}

impl SubTruncatedAssign<&Self> for RationalPolynomial {
    /// Subtracts a [`RationalPolynomial`] from a [`RationalPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by reference.
    ///
    /// $$
    /// p \gets (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is kept in
    /// lowest terms: cutting a polynomial can remove the coefficients that kept its numerator
    /// coprime to its denominator, so when anything is cut, the whole common factor of the new
    /// numerator and denominator is divided out. The difference is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The quadratic terms cancel.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.sub_truncated_assign(
    ///     &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "-1/3*x+1/2");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.sub_truncated_assign(
    ///     &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     1,
    /// );
    /// assert_eq!(p.to_string(), "1/2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub_series` from `fmpq_poly/sub_series.c`, FLINT 3.6.0.
    #[inline]
    fn sub_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = add_or_sub_truncated_owned_ref(take(self), other, len, true);
    }
}
