// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square_truncated::square_truncated_ref;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::SquareAssign;
use malachite_base::polynomial::{SquareTruncated, SquareTruncatedAssign};

impl SquareTruncated for NaturalPolynomial {
    type Output = Self;

    /// Squares an [`NaturalPolynomial`], keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking it by value.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "13*x^2+12*x+4"
    /// );
    /// // The cross terms combine with the square of the linear coefficient.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x+1").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "3*x^2+2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
    #[inline]
    fn square_truncated(mut self, len: u64) -> Self {
        self.square_truncated_assign(len);
        self
    }
}

impl SquareTruncated for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares an [`NaturalPolynomial`], keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking it by reference.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "13*x^2+12*x+4"
    /// );
    /// // The cross terms combine with the square of the linear coefficient.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x+1").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "3*x^2+2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
    #[inline]
    fn square_truncated(self, len: u64) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: square_truncated_ref(&self.coefficients, len),
        }
    }
}

impl SquareTruncatedAssign for NaturalPolynomial {
    /// Squares an [`NaturalPolynomial`] in place, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`.
    ///
    /// $$
    /// p \gets p^2 \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.square_truncated_assign(3);
    /// assert_eq!(p.to_string(), "13*x^2+12*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_sqrlow` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
    #[inline]
    fn square_truncated_assign(&mut self, len: u64) {
        // The square of a constant is computed in place.
        if len != 0
            && let [c] = self.coefficients.as_mut_slice()
        {
            c.square_assign();
        } else {
            self.coefficients = square_truncated_ref(&self.coefficients, len);
        }
    }
}
