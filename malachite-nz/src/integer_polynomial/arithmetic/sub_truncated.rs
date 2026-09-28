// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::sub::{rsub_assign_ref, sub_assign_ref, sub_assign_val};
use malachite_base::polynomial::{Polynomial, SubTruncated, SubTruncatedAssign};

// The first `len` elements of `xs`, or all of them if there are fewer.
fn prefix(xs: &[Integer], len: u64) -> &[Integer] {
    &xs[..usize::try_from(len).map_or(xs.len(), |len| len.min(xs.len()))]
}

impl SubTruncated<Self> for IntegerPolynomial {
    type Output = Self;

    /// Subtracts one [`IntegerPolynomial`] from another, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is trimmed, so
    /// when coefficients cancel at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^3+2*x^2-x+5")
    ///         .unwrap()
    ///         .sub_truncated(IntegerPolynomial::from_str("4*x^2+x-2").unwrap(), 3)
    ///         .to_string(),
    ///     "-2*x^2-2*x+7"
    /// );
    /// // The quadratic and linear coefficients cancel.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^3+2*x^2-x+5")
    ///         .unwrap()
    ///         .sub_truncated(IntegerPolynomial::from_str("2*x^2-x+3").unwrap(), 3)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub_series` from `fmpz_poly/sub_series.c`, FLINT 3.6.0.
    fn sub_truncated(mut self, mut other: Self, len: u64) -> Self {
        self.truncate_assign(len);
        other.truncate_assign(len);
        sub_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
        self
    }
}

impl SubTruncated<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Subtracts one [`IntegerPolynomial`] from another, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is trimmed, so
    /// when coefficients cancel at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^3+2*x^2-x+5")
    ///         .unwrap()
    ///         .sub_truncated(&IntegerPolynomial::from_str("4*x^2+x-2").unwrap(), 3)
    ///         .to_string(),
    ///     "-2*x^2-2*x+7"
    /// );
    /// // The quadratic and linear coefficients cancel.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^3+2*x^2-x+5")
    ///         .unwrap()
    ///         .sub_truncated(&IntegerPolynomial::from_str("2*x^2-x+3").unwrap(), 3)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub_series` from `fmpz_poly/sub_series.c`, FLINT 3.6.0.
    fn sub_truncated(mut self, other: &Self, len: u64) -> Self {
        self.truncate_assign(len);
        sub_assign_ref(&mut self.coefficients, prefix(&other.coefficients, len));
        self.trim();
        self
    }
}

impl SubTruncated<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Subtracts one [`IntegerPolynomial`] from another, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is trimmed, so
    /// when coefficients cancel at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap())
    ///         .sub_truncated(IntegerPolynomial::from_str("4*x^2+x-2").unwrap(), 3)
    ///         .to_string(),
    ///     "-2*x^2-2*x+7"
    /// );
    /// // The quadratic and linear coefficients cancel.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap())
    ///         .sub_truncated(IntegerPolynomial::from_str("2*x^2-x+3").unwrap(), 3)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub_series` from `fmpz_poly/sub_series.c`, FLINT 3.6.0.
    fn sub_truncated(self, mut other: IntegerPolynomial, len: u64) -> IntegerPolynomial {
        other.truncate_assign(len);
        rsub_assign_ref(&mut other.coefficients, prefix(&self.coefficients, len));
        other.trim();
        other
    }
}

impl SubTruncated<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Subtracts one [`IntegerPolynomial`] from another, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is trimmed, so
    /// when coefficients cancel at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap())
    ///         .sub_truncated(&IntegerPolynomial::from_str("4*x^2+x-2").unwrap(), 3)
    ///         .to_string(),
    ///     "-2*x^2-2*x+7"
    /// );
    /// // The quadratic and linear coefficients cancel.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap())
    ///         .sub_truncated(&IntegerPolynomial::from_str("2*x^2-x+3").unwrap(), 3)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub_series` from `fmpz_poly/sub_series.c`, FLINT 3.6.0.
    fn sub_truncated(self, other: &IntegerPolynomial, len: u64) -> IntegerPolynomial {
        let mut coefficients = prefix(&self.coefficients, len).to_vec();
        sub_assign_ref(&mut coefficients, prefix(&other.coefficients, len));
        IntegerPolynomial::from_coefficients_asc(coefficients)
    }
}

impl SubTruncatedAssign<Self> for IntegerPolynomial {
    /// Subtracts an [`IntegerPolynomial`] from an [`IntegerPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by value.
    ///
    /// $$
    /// p \gets (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is trimmed, so
    /// when coefficients cancel at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap();
    /// p.sub_truncated_assign(IntegerPolynomial::from_str("4*x^2+x-2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "-2*x^2-2*x+7");
    ///
    /// // The quadratic and linear coefficients cancel.
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap();
    /// p.sub_truncated_assign(IntegerPolynomial::from_str("2*x^2-x+3").unwrap(), 3);
    /// assert_eq!(p.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub_series` from `fmpz_poly/sub_series.c`, FLINT 3.6.0.
    fn sub_truncated_assign(&mut self, mut other: Self, len: u64) {
        self.truncate_assign(len);
        other.truncate_assign(len);
        sub_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
    }
}

impl SubTruncatedAssign<&Self> for IntegerPolynomial {
    /// Subtracts an [`IntegerPolynomial`] from an [`IntegerPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by reference.
    ///
    /// $$
    /// p \gets (p - q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the difference of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The difference is trimmed, so
    /// when coefficients cancel at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SubTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap();
    /// p.sub_truncated_assign(&IntegerPolynomial::from_str("4*x^2+x-2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "-2*x^2-2*x+7");
    ///
    /// // The quadratic and linear coefficients cancel.
    /// let mut p = IntegerPolynomial::from_str("x^3+2*x^2-x+5").unwrap();
    /// p.sub_truncated_assign(&IntegerPolynomial::from_str("2*x^2-x+3").unwrap(), 3);
    /// assert_eq!(p.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sub_series` from `fmpz_poly/sub_series.c`, FLINT 3.6.0.
    fn sub_truncated_assign(&mut self, other: &Self, len: u64) {
        self.truncate_assign(len);
        sub_assign_ref(&mut self.coefficients, prefix(&other.coefficients, len));
        self.trim();
    }
}
