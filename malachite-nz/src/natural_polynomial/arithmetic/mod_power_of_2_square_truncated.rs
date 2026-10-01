// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square_truncated::square_truncated_ref;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::reduce_coefficients;
use crate::natural_polynomial::arithmetic::mod_power_of_2_square::assert_reduced;
use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
use malachite_base::polynomial::{ModPowerOf2SquareTruncated, ModPowerOf2SquareTruncatedAssign};

impl ModPowerOf2SquareTruncated for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by value. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \\bmod x^n) \\bmod 2^k.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 8.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 3)
    ///         .to_string(),
    ///     "5*x^2+4*x+4"
    /// );
    /// // The square is 16*x^2+8*x+1; truncation and reduction leave 8*x+1.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same and the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_square_truncated(mut self, len: u64, pow: u64) -> Self {
        self.mod_power_of_2_square_truncated_assign(len, pow);
        self
    }
}

impl ModPowerOf2SquareTruncated for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by reference. The coefficients must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \\bmod x^n) \\bmod 2^k.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 8.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 3)
    ///         .to_string(),
    ///     "5*x^2+4*x+4"
    /// );
    /// // The square is 16*x^2+8*x+1; truncation and reduction leave 8*x+1.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same and the modulus $2^k$.
    fn mod_power_of_2_square_truncated(self, len: u64, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, pow);
        reduce_coefficients(square_truncated_ref(&self.coefficients, len), pow)
    }
}

impl ModPowerOf2SquareTruncatedAssign for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo $2^k$ in place, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets (p^2 \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_square_truncated_assign(3, 3);
    /// assert_eq!(p.to_string(), "5*x^2+4*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same and the modulus $2^k$.
    fn mod_power_of_2_square_truncated_assign(&mut self, len: u64, pow: u64) {
        assert_reduced(self, pow);
        // The square of a constant is computed in place.
        if len != 0
            && let [c] = self.coefficients.as_mut_slice()
        {
            c.mod_power_of_2_square_assign(pow);
            self.trim();
        } else {
            *self = reduce_coefficients(square_truncated_ref(&self.coefficients, len), pow);
        }
    }
}
