// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul::{mul_ref_ref, mul_val_ref, mul_val_val};
use crate::natural_polynomial::NaturalPolynomial;
use core::mem::take;
use core::ops::{Mul, MulAssign};

impl Mul<Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The natural numbers have no zero divisors, so the degree of a product of nonzero polynomials
    /// is the sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         * NaturalPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "2*x^3+11*x^2+19*x+10"
    /// );
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap()
    ///         * NaturalPolynomial::from_str("x+2").unwrap())
    ///     .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap() * NaturalPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            coefficients: mul_val_val(self.coefficients, other.coefficients),
        }
    }
}

impl Mul<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The natural numbers have no zero divisors, so the degree of a product of nonzero polynomials
    /// is the sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         * &NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "2*x^3+11*x^2+19*x+10"
    /// );
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap()
    ///         * &NaturalPolynomial::from_str("x+2").unwrap())
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap() * &NaturalPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: &Self) -> Self {
        Self {
            coefficients: mul_val_ref(self.coefficients, &other.coefficients),
        }
    }
}

impl Mul<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s, taking the first by reference and the second by
    /// value.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The natural numbers have no zero divisors, so the degree of a product of nonzero polynomials
    /// is the sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         * NaturalPolynomial::from_str("2*x+5").unwrap())
    ///     .to_string(),
    ///     "2*x^3+11*x^2+19*x+10"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap()
    ///         * NaturalPolynomial::from_str("x+2").unwrap())
    ///     .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap() * NaturalPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, other: NaturalPolynomial) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: mul_val_ref(other.coefficients, &self.coefficients),
        }
    }
}

impl Mul<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(p, q) = pq.
    /// $$
    ///
    /// The natural numbers have no zero divisors, so the degree of a product of nonzero polynomials
    /// is the sum of their degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap()
    ///         * &NaturalPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "2*x^3+11*x^2+19*x+10"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap()
    ///         * &NaturalPolynomial::from_str("x+2").unwrap())
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap() * &NaturalPolynomial::ZERO).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    fn mul(self, other: &NaturalPolynomial) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: mul_ref_ref(&self.coefficients, &other.coefficients),
        }
    }
}

impl MulAssign<Self> for NaturalPolynomial {
    /// Multiplies an [`NaturalPolynomial`] by another [`NaturalPolynomial`] in place, taking the
    /// right-hand side by value.
    ///
    /// $$
    /// p \gets pq.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p *= NaturalPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+19*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.coefficients = mul_val_val(take(&mut self.coefficients), other.coefficients);
    }
}

impl MulAssign<&Self> for NaturalPolynomial {
    /// Multiplies an [`NaturalPolynomial`] by another [`NaturalPolynomial`] in place, taking the
    /// right-hand side by reference.
    ///
    /// $$
    /// p \gets pq.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p *= &NaturalPolynomial::from_str("2*x+5").unwrap();
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+19*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mul` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        self.coefficients = mul_val_ref(take(&mut self.coefficients), &other.coefficients);
    }
}
