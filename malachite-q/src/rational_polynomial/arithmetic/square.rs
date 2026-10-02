// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

impl Square for RationalPolynomial {
    type Output = Self;

    /// Squares a [`RationalPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p) = p^2.
    /// $$
    ///
    /// With $p = x/a$ in lowest terms, $x^2/a^2$ is in lowest terms too: by Gauss's lemma the
    /// content of $x^2$ is the square of the content of $x$, which is coprime to $a$. So no GCD is
    /// needed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3")
    ///         .unwrap()
    ///         .square()
    ///         .to_string(),
    ///     "1/4*x^2+1/3*x+1/9"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x-1")
    ///         .unwrap()
    ///         .square()
    ///         .to_string(),
    ///     "x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0, with both factors
    /// the same polynomial.
    fn square(self) -> Self {
        Self {
            numerator: self.numerator.square(),
            denominator: self.denominator.square(),
        }
    }
}

impl Square for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Squares a [`RationalPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p) = p^2.
    /// $$
    ///
    /// With $p = x/a$ in lowest terms, $x^2/a^2$ is in lowest terms too: by Gauss's lemma the
    /// content of $x^2$ is the square of the content of $x$, which is coprime to $a$. So no GCD is
    /// needed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "1/4*x^2+1/3*x+1/9"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x-1").unwrap())
    ///         .square()
    ///         .to_string(),
    ///     "x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0, with both factors
    /// the same polynomial.
    fn square(self) -> RationalPolynomial {
        RationalPolynomial {
            numerator: (&self.numerator).square(),
            denominator: (&self.denominator).square(),
        }
    }
}

impl SquareAssign for RationalPolynomial {
    /// Squares a [`RationalPolynomial`] in place.
    ///
    /// $$
    /// p \gets p^2.
    /// $$
    ///
    /// No GCD is needed; see the [`Square`] implementation for why.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is the largest
    /// number of significant bits of any numerator coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p.square_assign();
    /// assert_eq!(p.to_string(), "1/4*x^2+1/3*x+1/9");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_mul` from `fmpq_poly/mul.c`, FLINT 3.6.0, with both factors
    /// the same polynomial.
    fn square_assign(&mut self) {
        self.numerator.square_assign();
        self.denominator.square_assign();
    }
}
