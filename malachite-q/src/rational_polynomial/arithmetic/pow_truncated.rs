// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::mul_truncated::canonicalize_truncated;
use core::mem::take;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::polynomial::{PowTruncated, PowTruncatedAssign};

impl PowTruncated for RationalPolynomial {
    type Output = Self;

    /// Raises a [`RationalPolynomial`] to a power, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by value.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0. Truncation can
    /// leave the numerator sharing a factor with the denominator, so the result is reduced
    /// afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .pow_truncated(3, 2)
    ///         .to_string(),
    ///     "1/6*x+1/27"
    /// );
    /// // The low coefficients of the cube have no denominator left.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^2+x+1").unwrap())
    ///         .pow_truncated(3, 2)
    ///         .to_string(),
    ///     "3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_pow_trunc` from `fmpq_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that the truncated power of the numerator is computed over the integers and reduced
    /// against the power of the denominator once, at the end, where FLINT reduces after every
    /// truncated multiplication.
    #[inline]
    fn pow_truncated(self, exp: u64, len: u64) -> Self {
        canonicalize_truncated(
            self.numerator.pow_truncated(exp, len),
            self.denominator.pow(exp),
        )
    }
}

impl PowTruncated for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Raises a [`RationalPolynomial`] to a power, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by reference.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0. Truncation can
    /// leave the numerator sharing a factor with the denominator, so the result is reduced
    /// afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .pow_truncated(3, 2)
    ///         .to_string(),
    ///     "1/6*x+1/27"
    /// );
    /// // The low coefficients of the cube have no denominator left.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+x+1").unwrap())
    ///         .pow_truncated(3, 2)
    ///         .to_string(),
    ///     "3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_pow_trunc` from `fmpq_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that the truncated power of the numerator is computed over the integers and reduced
    /// against the power of the denominator once, at the end, where FLINT reduces after every
    /// truncated multiplication.
    #[inline]
    fn pow_truncated(self, exp: u64, len: u64) -> RationalPolynomial {
        canonicalize_truncated(
            (&self.numerator).pow_truncated(exp, len),
            (&self.denominator).pow(exp),
        )
    }
}

impl PowTruncatedAssign for RationalPolynomial {
    /// Raises a [`RationalPolynomial`] to a power in place, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`.
    ///
    /// $$
    /// p \gets p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$.
    /// The zeroth power of every polynomial is 1, truncated to 0 when `len` is 0. Truncation can
    /// leave the numerator sharing a factor with the denominator, so the result is reduced
    /// afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p.pow_truncated_assign(3, 2);
    /// assert_eq!(p.to_string(), "1/6*x+1/27");
    ///
    /// // The low coefficients of the cube have no denominator left.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+x+1").unwrap();
    /// p.pow_truncated_assign(3, 2);
    /// assert_eq!(p.to_string(), "3*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_pow_trunc` from `fmpq_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that the truncated power of the numerator is computed over the integers and reduced
    /// against the power of the denominator once, at the end, where FLINT reduces after every
    /// truncated multiplication.
    #[inline]
    fn pow_truncated_assign(&mut self, exp: u64, len: u64) {
        *self = take(self).pow_truncated(exp, len);
    }
}
