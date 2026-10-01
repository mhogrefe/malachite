// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul_truncated::{
    mul_truncated_ref_ref, mul_truncated_val_ref, mul_truncated_val_val,
};
use crate::natural_polynomial::NaturalPolynomial;
use core::mem::take;
use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign};

impl MulTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_truncated(NaturalPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "19*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(NaturalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: Self, len: u64) -> Self {
        Self {
            coefficients: mul_truncated_val_val(self.coefficients, other.coefficients, len),
        }
    }
}

impl MulTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_truncated(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "19*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(&NaturalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: &Self, len: u64) -> Self {
        Self {
            coefficients: mul_truncated_val_ref(self.coefficients, &other.coefficients, len),
        }
    }
}

impl MulTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_truncated(NaturalPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "19*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(NaturalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: NaturalPolynomial, len: u64) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: mul_truncated_val_ref(other.coefficients, &self.coefficients, len),
        }
    }
}

impl MulTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_truncated(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "19*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(&NaturalPolynomial::from_str("x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: &NaturalPolynomial, len: u64) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len),
        }
    }
}

impl MulTruncatedAssign<Self> for NaturalPolynomial {
    /// Multiplies an [`NaturalPolynomial`] by another [`NaturalPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by value.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mul_truncated_assign(NaturalPolynomial::from_str("2*x+5").unwrap(), 2);
    /// assert_eq!(p.to_string(), "19*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated_assign(&mut self, other: Self, len: u64) {
        self.coefficients =
            mul_truncated_val_val(take(&mut self.coefficients), other.coefficients, len);
    }
}

impl MulTruncatedAssign<&Self> for NaturalPolynomial {
    /// Multiplies an [`NaturalPolynomial`] by another [`NaturalPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by reference.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mul_truncated_assign(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2);
    /// assert_eq!(p.to_string(), "19*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated_assign(&mut self, other: &Self, len: u64) {
        self.coefficients =
            mul_truncated_val_ref(take(&mut self.coefficients), &other.coefficients, len);
    }
}
