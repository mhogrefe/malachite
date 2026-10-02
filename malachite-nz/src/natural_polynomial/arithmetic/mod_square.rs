// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square::square_ref;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_mul::mod_reduce_coefficients;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSquare, ModSquareAssign};

pub(crate) fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

impl ModSquare<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo `m`, taking the polynomial by value and the modulus
    /// by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p^2 \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the square can vanish modulo `m`, and then
    /// the degree of the square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 1, 6, 13, 12, and 4 are reduced modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square(Natural::from(7u32))
    ///         .to_string(),
    ///     "x^4+6*x^3+6*x^2+5*x+4"
    /// );
    /// // Modulo 9, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square(Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn mod_square(mut self, m: Natural) -> Self {
        self.mod_square_assign(&m);
        self
    }
}

impl ModSquare<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo `m`, taking the polynomial by value and the modulus
    /// by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p^2 \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the square can vanish modulo `m`, and then
    /// the degree of the square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 1, 6, 13, 12, and 4 are reduced modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square(&Natural::from(7u32))
    ///         .to_string(),
    ///     "x^4+6*x^3+6*x^2+5*x+4"
    /// );
    /// // Modulo 9, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square(&Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn mod_square(mut self, m: &Natural) -> Self {
        self.mod_square_assign(m);
        self
    }
}

impl ModSquare<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo `m`, taking the polynomial by reference and the
    /// modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p^2 \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the square can vanish modulo `m`, and then
    /// the degree of the square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 1, 6, 13, 12, and 4 are reduced modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square(Natural::from(7u32))
    ///         .to_string(),
    ///     "x^4+6*x^3+6*x^2+5*x+4"
    /// );
    /// // Modulo 9, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square(Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0.
    fn mod_square(self, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, &m);
        mod_reduce_coefficients(square_ref(&self.coefficients), &m)
    }
}

impl ModSquare<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo `m`, taking the polynomial by reference and the
    /// modulus by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, m) = p^2 \bmod m.
    /// $$
    ///
    /// When `m` is not prime, the leading coefficient of the square can vanish modulo `m`, and then
    /// the degree of the square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients 1, 6, 13, 12, and 4 are reduced modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square(&Natural::from(7u32))
    ///         .to_string(),
    ///     "x^4+6*x^3+6*x^2+5*x+4"
    /// );
    /// // Modulo 9, the leading coefficient vanishes and the degree drops.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square(&Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0.
    fn mod_square(self, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, m);
        mod_reduce_coefficients(square_ref(&self.coefficients), m)
    }
}

impl ModSquareAssign<Natural> for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo `m` in place, taking the modulus by value. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p^2 \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_square_assign(Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^4+6*x^3+6*x^2+5*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0.
    fn mod_square_assign(&mut self, m: Natural) {
        assert_reduced(self, &m);
        // The square of a constant is computed in place.
        if let [c] = self.coefficients.as_mut_slice() {
            c.mod_square_assign(&m);
            self.trim();
        } else {
            *self = mod_reduce_coefficients(square_ref(&self.coefficients), &m);
        }
    }
}

impl ModSquareAssign<&Natural> for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo `m` in place, taking the modulus by reference. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p^2 \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_square_assign(&Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^4+6*x^3+6*x^2+5*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0.
    fn mod_square_assign(&mut self, m: &Natural) {
        assert_reduced(self, m);
        // The square of a constant is computed in place.
        if let [c] = self.coefficients.as_mut_slice() {
            c.mod_square_assign(m);
            self.trim();
        } else {
            *self = mod_reduce_coefficients(square_ref(&self.coefficients), m);
        }
    }
}
