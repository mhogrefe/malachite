// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square_truncated::square_truncated_ref;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_mul::mod_reduce_coefficients;
use crate::natural_polynomial::arithmetic::mod_square::assert_reduced;
use malachite_base::num::arithmetic::traits::ModSquareAssign;
use malachite_base::polynomial::{ModSquareTruncated, ModSquareTruncatedAssign};

impl ModSquareTruncated<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the polynomial by value and the modulus by value. The coefficients
    /// must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square_truncated(3, Natural::from(7u32))
    ///         .to_string(),
    ///     "6*x^2+5*x+4"
    /// );
    /// // The square is 9*x^2+6*x+1; truncation and reduction modulo 9 leave 6*x+1.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square_truncated(3, Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same.
    #[inline]
    fn mod_square_truncated(mut self, len: u64, m: Natural) -> Self {
        self.mod_square_truncated_assign(len, &m);
        self
    }
}

impl ModSquareTruncated<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the polynomial by value and the modulus by reference. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 7.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square_truncated(3, &Natural::from(7u32))
    ///         .to_string(),
    ///     "6*x^2+5*x+4"
    /// );
    /// // The square is 9*x^2+6*x+1; truncation and reduction modulo 9 leave 6*x+1.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square_truncated(3, &Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same.
    #[inline]
    fn mod_square_truncated(mut self, len: u64, m: &Natural) -> Self {
        self.mod_square_truncated_assign(len, m);
        self
    }
}

impl ModSquareTruncated<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the polynomial by reference and the modulus by value. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square_truncated(3, Natural::from(7u32))
    ///         .to_string(),
    ///     "6*x^2+5*x+4"
    /// );
    /// // The square is 9*x^2+6*x+1; truncation and reduction modulo 9 leave 6*x+1.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square_truncated(3, Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same.
    fn mod_square_truncated(self, len: u64, m: Natural) -> NaturalPolynomial {
        assert_reduced(self, &m);
        mod_reduce_coefficients(square_truncated_ref(&self.coefficients, len), &m)
    }
}

impl ModSquareTruncated<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo `m`, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the polynomial by reference and the modulus by reference. The
    /// coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
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
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 7.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square_truncated(3, &Natural::from(7u32))
    ///         .to_string(),
    ///     "6*x^2+5*x+4"
    /// );
    /// // The square is 9*x^2+6*x+1; truncation and reduction modulo 9 leave 6*x+1.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("3*x+1").unwrap())
    ///         .mod_square_truncated(3, &Natural::from(9u32))
    ///         .to_string(),
    ///     "6*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same.
    fn mod_square_truncated(self, len: u64, m: &Natural) -> NaturalPolynomial {
        assert_reduced(self, m);
        mod_reduce_coefficients(square_truncated_ref(&self.coefficients, len), m)
    }
}

impl ModSquareTruncatedAssign<Natural> for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo `m` in place, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the modulus by value. The coefficients must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_square_truncated_assign(3, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "6*x^2+5*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same.
    fn mod_square_truncated_assign(&mut self, len: u64, m: Natural) {
        assert_reduced(self, &m);
        // The square of a constant is computed in place.
        if len != 0
            && let [c] = self.coefficients.as_mut_slice()
        {
            c.mod_square_assign(&m);
            self.trim();
        } else {
            *self = mod_reduce_coefficients(square_truncated_ref(&self.coefficients, len), &m);
        }
    }
}

impl ModSquareTruncatedAssign<&Natural> for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo `m` in place, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the modulus by reference. The coefficients must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_square_truncated_assign(3, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "6*x^2+5*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with both polynomials the same.
    fn mod_square_truncated_assign(&mut self, len: u64, m: &Natural) {
        assert_reduced(self, m);
        // The square of a constant is computed in place.
        if len != 0
            && let [c] = self.coefficients.as_mut_slice()
        {
            c.mod_square_assign(m);
            self.trim();
        } else {
            *self = mod_reduce_coefficients(square_truncated_ref(&self.coefficients, len), m);
        }
    }
}
