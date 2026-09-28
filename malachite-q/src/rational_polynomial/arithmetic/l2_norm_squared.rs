// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::polynomial::L2NormSquared;

impl L2NormSquared for &RationalPolynomial {
    type Output = Rational;

    /// Computes the sum of the squares of a [`RationalPolynomial`]'s coefficients, which is the
    /// square of its $L^2$ norm.
    ///
    /// $$
    /// f(p) = \sum_i p_i^2.
    /// $$
    ///
    /// With the polynomial written as $x/d$ in lowest terms, this is the integer sum of the squares
    /// of the coefficients of $x$ over $d^2$, reduced.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::L2NormSquared;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2-1/3")
    ///         .unwrap()
    ///         .l2_norm_squared()
    ///         .to_string(),
    ///     "13/36"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/2")
    ///         .unwrap()
    ///         .l2_norm_squared()
    ///         .to_string(),
    ///     "1/2"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x-3")
    ///         .unwrap()
    ///         .l2_norm_squared()
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    #[inline]
    fn l2_norm_squared(self) -> Rational {
        Rational::from_naturals(
            (&self.numerator).l2_norm_squared(),
            (&self.denominator).square(),
        )
    }
}
