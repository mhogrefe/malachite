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
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::polynomial::{SquareTruncated, SquareTruncatedAssign};

impl SquareTruncated for RationalPolynomial {
    type Output = Self;

    /// Squares a [`RationalPolynomial`], keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking it by value.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$.
    /// Truncation can leave the numerator sharing a factor with the denominator, so the result is
    /// reduced afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The square is 1/4*x^2+1/3*x+1/9; its low two coefficients.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3")
    ///         .unwrap()
    ///         .square_truncated(2)
    ///         .to_string(),
    ///     "1/3*x+1/9"
    /// );
    /// // The square is 1/4*x^4+x^3+2*x^2+2*x+1; once the quartic term is cut, nothing is left over
    /// // 4.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+x+1")
    ///         .unwrap()
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "2*x^2+2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors the same polynomial.
    #[inline]
    fn square_truncated(self, len: u64) -> Self {
        canonicalize_truncated(
            self.numerator.square_truncated(len),
            self.denominator.square(),
        )
    }
}

impl SquareTruncated for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Squares a [`RationalPolynomial`], keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking it by reference.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$.
    /// Truncation can leave the numerator sharing a factor with the denominator, so the result is
    /// reduced afterwards.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The square is 1/4*x^2+1/3*x+1/9; its low two coefficients.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .square_truncated(2)
    ///         .to_string(),
    ///     "1/3*x+1/9"
    /// );
    /// // The square is 1/4*x^4+x^3+2*x^2+2*x+1; once the quartic term is cut, nothing is left over
    /// // 4.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+x+1").unwrap())
    ///         .square_truncated(3)
    ///         .to_string(),
    ///     "2*x^2+2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors the same polynomial.
    #[inline]
    fn square_truncated(self, len: u64) -> RationalPolynomial {
        canonicalize_truncated(
            (&self.numerator).square_truncated(len),
            (&self.denominator).square(),
        )
    }
}

impl SquareTruncatedAssign for RationalPolynomial {
    /// Squares a [`RationalPolynomial`] in place, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`.
    ///
    /// $$
    /// p \gets p^2 \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::SquareTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+x+1").unwrap();
    /// p.square_truncated_assign(3);
    /// assert_eq!(p.to_string(), "2*x^2+2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mullow` from `fmpq_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors the same polynomial.
    fn square_truncated_assign(&mut self, len: u64) {
        *self = take(self).square_truncated(len);
    }
}
