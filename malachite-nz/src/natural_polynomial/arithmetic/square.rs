// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square::square_ref;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

impl Square for NaturalPolynomial {
    type Output = Self;

    /// Squares an [`NaturalPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p) = p^2.
    /// $$
    ///
    /// Squaring takes roughly half the coefficient multiplications of multiplying two different
    /// polynomials of the same length.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// the largest number of significant bits of any of its coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^4+6*x^3+13*x^2+12*x+4"
    /// );
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^2+2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn square(mut self) -> Self {
        self.square_assign();
        self
    }
}

impl Square for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares an [`NaturalPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p) = p^2.
    /// $$
    ///
    /// Squaring takes roughly half the coefficient multiplications of multiplying two different
    /// polynomials of the same length.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// the largest number of significant bits of any of its coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^4+6*x^3+13*x^2+12*x+4"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^2+2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn square(self) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: square_ref(&self.coefficients),
        }
    }
}

impl SquareAssign for NaturalPolynomial {
    /// Squares an [`NaturalPolynomial`] in place.
    ///
    /// $$
    /// p \gets p^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// the largest number of significant bits of any of its coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.square_assign();
    /// assert_eq!(p.to_string(), "x^4+6*x^3+13*x^2+12*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqr` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
    #[inline]
    fn square_assign(&mut self) {
        // The square of a constant is computed in place.
        if let [c] = self.coefficients.as_mut_slice() {
            c.square_assign();
        } else {
            self.coefficients = square_ref(&self.coefficients);
        }
    }
}
