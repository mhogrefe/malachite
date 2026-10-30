// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul_middle::{
    mul_middle_ref_ref, mul_middle_val_ref, mul_middle_val_val,
};
use crate::natural_polynomial::NaturalPolynomial;
use core::mem::take;
use malachite_base::polynomial::{MulMiddle, MulMiddleAssign};

impl MulMiddle<Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking both by value.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // (x^2+3*x+2)*(2*x+5) = 2*x^3+11*x^2+19*x+10.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_middle(NaturalPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "11*x+19"
    /// );
    /// // The range extends past the end of the product, x^2+3*x+2.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(NaturalPolynomial::from_str("x+2").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: Self, low: u64, high: u64) -> Self {
        Self {
            coefficients: mul_middle_val_val(self.coefficients, other.coefficients, low, high),
        }
    }
}

impl MulMiddle<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // (x^2+3*x+2)*(2*x+5) = 2*x^3+11*x^2+19*x+10.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_middle(&NaturalPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "11*x+19"
    /// );
    /// // The range extends past the end of the product, x^2+3*x+2.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(&NaturalPolynomial::from_str("x+2").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: &Self, low: u64, high: u64) -> Self {
        Self {
            coefficients: mul_middle_val_ref(self.coefficients, &other.coefficients, low, high),
        }
    }
}

impl MulMiddle<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // (x^2+3*x+2)*(2*x+5) = 2*x^3+11*x^2+19*x+10.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_middle(NaturalPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "11*x+19"
    /// );
    /// // The range extends past the end of the product, x^2+3*x+2.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(NaturalPolynomial::from_str("x+2").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: NaturalPolynomial, low: u64, high: u64) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: mul_middle_val_ref(other.coefficients, &self.coefficients, low, high),
        }
    }
}

impl MulMiddle<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking both by reference.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // (x^2+3*x+2)*(2*x+5) = 2*x^3+11*x^2+19*x+10.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mul_middle(&NaturalPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "11*x+19"
    /// );
    /// // The range extends past the end of the product, x^2+3*x+2.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(&NaturalPolynomial::from_str("x+2").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: &NaturalPolynomial, low: u64, high: u64) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: mul_middle_ref_ref(&self.coefficients, &other.coefficients, low, high),
        }
    }
}

impl MulMiddleAssign<Self> for NaturalPolynomial {
    /// Multiplies an [`NaturalPolynomial`] by another [`NaturalPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for `low` $\leq i <$ `high` and shifting them down by `low`,
    /// taking the right-hand side by value.
    ///
    /// $$
    /// p \gets \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
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
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddleAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mul_middle_assign(NaturalPolynomial::from_str("2*x+5").unwrap(), 1, 3);
    /// assert_eq!(p.to_string(), "11*x+19");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle_assign(&mut self, other: Self, low: u64, high: u64) {
        self.coefficients =
            mul_middle_val_val(take(&mut self.coefficients), other.coefficients, low, high);
    }
}

impl MulMiddleAssign<&Self> for NaturalPolynomial {
    /// Multiplies an [`NaturalPolynomial`] by another [`NaturalPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for `low` $\leq i <$ `high` and shifting them down by `low`,
    /// taking the right-hand side by reference.
    ///
    /// $$
    /// p \gets \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
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
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddleAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mul_middle_assign(&NaturalPolynomial::from_str("2*x+5").unwrap(), 1, 3);
    /// assert_eq!(p.to_string(), "11*x+19");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle_assign(&mut self, other: &Self, low: u64, high: u64) {
        self.coefficients =
            mul_middle_val_ref(take(&mut self.coefficients), &other.coefficients, low, high);
    }
}
