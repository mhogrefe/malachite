// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::pow_truncated::{
    pow_truncated_assign_vec, pow_truncated_ref,
};
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::polynomial::{PowTruncated, PowTruncatedAssign};

impl PowTruncated for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by value.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$,
    /// so only its first `len` coefficients are read. The zeroth power of every polynomial is 1,
    /// truncated to 0 when `len` is 0. The power is computed by repeated truncated squaring and
    /// multiplication.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x+1").unwrap())
    ///         .pow_truncated(5, 3)
    ///         .to_string(),
    ///     "10*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .pow_truncated(4, 4)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^k$ is removed before powering, and that the intermediate powers
    /// are kept at their own lengths rather than padded to `len`.
    #[inline]
    fn pow_truncated(mut self, exp: u64, len: u64) -> Self {
        self.pow_truncated_assign(exp, len);
        self
    }
}

impl PowTruncated for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by reference.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$,
    /// so only its first `len` coefficients are read. The zeroth power of every polynomial is 1,
    /// truncated to 0 when `len` is 0. The power is computed by repeated truncated squaring and
    /// multiplication.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .pow_truncated(5, 3)
    ///         .to_string(),
    ///     "10*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .pow_truncated(4, 4)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^k$ is removed before powering, and that the intermediate powers
    /// are kept at their own lengths rather than padded to `len`.
    #[inline]
    fn pow_truncated(self, exp: u64, len: u64) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: pow_truncated_ref(&self.coefficients, exp, len),
        }
    }
}

impl PowTruncatedAssign for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power in place, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`.
    ///
    /// $$
    /// p \gets p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$,
    /// so only its first `len` coefficients are read. The zeroth power of every polynomial is 1,
    /// truncated to 0 when `len` is 0. The power is computed by repeated truncated squaring and
    /// multiplication.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.pow_truncated_assign(5, 3);
    /// assert_eq!(p.to_string(), "10*x^2+5*x+1");
    ///
    /// // The power is a multiple of x^4.
    /// let mut p = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// p.pow_truncated_assign(4, 4);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^k$ is removed before powering, and that the intermediate powers
    /// are kept at their own lengths rather than padded to `len`.
    #[inline]
    fn pow_truncated_assign(&mut self, exp: u64, len: u64) {
        pow_truncated_assign_vec(&mut self.coefficients, exp, len);
    }
}
